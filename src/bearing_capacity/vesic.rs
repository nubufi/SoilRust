use super::helper_functions::*;
use std::f64::consts::PI;

use crate::{
    enums::AnalysisTerm,
    models::{foundation::Foundation, loads::Loads, soil_profile::SoilProfile},
    validation::ValidationError,
};

use super::{helper_functions::get_soil_params, model::*};

/// Validates the input data for vesics bearing capacity calculations.
///
/// # Arguments
/// * `soil_profile` - The soil profile data.
/// * `foundation` - The foundation data.
/// * `loading` - The applied loads.
/// * `term` - Short or long-term condition.
///
/// # Returns
/// * `Result<(), &'static str>`: Ok if valid, Err with a message if invalid.
pub fn validate_input(
    soil_profile: &SoilProfile,
    foundation: &Foundation,
    loading: &Loads,
    term: AnalysisTerm,
) -> Result<(), ValidationError> {
    soil_profile.validate(&["thickness", "dry_unit_weight", "saturated_unit_weight"])?;
    foundation.validate(&["foundation_depth", "foundation_width", "foundation_length"])?;
    loading.validate(&["vertical_load"])?;

    if soil_profile.layers.last().unwrap().depth.unwrap() < foundation.foundation_depth.unwrap() {
        return Err(ValidationError {
            code: "foundation.foundation_depth.smaller_than_soil_profile_depth".to_string(),
            message: "Foundation depth is smaller than the soil profile depth.".to_string(),
        });
    }

    for layer in soil_profile.layers.iter() {
        match term {
            AnalysisTerm::Short => {
                let fields_to_validate = ["cu", "phi_u"];
                layer.validate_fields(&fields_to_validate).unwrap();

                if layer.cu.unwrap() == 0. && layer.phi_u.unwrap() == 0. {
                    return Err(
                        ValidationError{
                            code: "soil_profile.layer.cu_or_phi_u_zero".to_string(),
                            message: "Either undrained shear strength (cu) or undrained friction angle (phi_u) must be greater than zero.".to_string(),
                        }
                    );
                }
            }
            AnalysisTerm::Long => {
                let fields_to_validate = ["c_prime", "phi_prime"];
                layer.validate_fields(&fields_to_validate).unwrap();

                if layer.c_prime.unwrap() == 0. && layer.phi_prime.unwrap() == 0. {
                    return Err(
                        ValidationError{
                            code: "soil_profile.layer.c_prime_or_phi_prime_zero".to_string(),
                            message: "Either effective cohesion (c') or effective friction angle (phi') must be greater than zero.".to_string(),
                        }
                    );
                }
            }
        }
    }

    Ok(())
}
/// Computes the bearing capacity factors Nc, Nq, and Ngamma based on the friction angle φ (degrees).
///
/// # Arguments
/// * `phi` - Friction angle in degrees.
///
/// # Returns
/// * `BearingCapacityFactors` containing Nc, Nq, and Ng.
pub fn calc_bearing_capacity_factors(phi: f64) -> BearingCapacityFactors {
    let phi_rad = phi.to_radians();

    let tan_phi = phi_rad.tan();
    let nq = (PI * tan_phi).exp() * (45.0 + phi / 2.0).to_radians().tan().powi(2);

    let nc = if phi == 0.0 {
        5.14
    } else {
        (nq - 1.0) / tan_phi
    };

    let ng = 2.0 * (nq - 1.0) * tan_phi;

    BearingCapacityFactors { nc, nq, ng }
}

/// Calculates shape factors (Sc, Sq, Sg) based on foundation geometry and bearing capacity factors.
///
/// # Arguments
/// * `foundation` - Foundation data (width and length)
/// * `bearing_capacity_factors` - Nc, Nq, Ng
/// * `phi` - Friction angle in degrees
///
/// # Returns
/// * `ShapeFactors`: shape coefficients for Sc, Sq, and Sg
pub fn calc_shape_factors(
    foundation: &Foundation,
    bearing_capacity_factors: BearingCapacityFactors,
    phi: f64,
) -> ShapeFactors {
    let width = foundation.foundation_width.unwrap();
    let length = foundation.foundation_length.unwrap();
    let w_l = width / length;

    let nc = bearing_capacity_factors.nc;
    let nq = bearing_capacity_factors.nq;

    let sc = if phi == 0. {
        0.2 * w_l
    } else {
        1.0 + w_l * (nq / nc)
    };
    let sq = 1.0 + w_l * (phi.to_radians().sin());

    let sg = 1.0 - 0.4 * w_l;

    ShapeFactors {
        sc,
        sq,
        sg: sg.max(0.6),
    }
}

/// Calculates the base inclination factors (bc, bq, bg) for a given friction angle and foundation geometry.
///
/// # Arguments
/// * `phi` - Internal friction angle in degrees
/// * `foundation` - Foundation struct with optional slope and foundation angles
///
/// # Returns
/// * `BaseFactors`: The base inclination factors
pub fn calc_base_factors(phi: f64, foundation: &Foundation) -> BaseFactors {
    let slope_angle = foundation.slope_angle.unwrap_or(0.0);
    let base_tilt_angle = foundation.base_tilt_angle.unwrap_or(0.0);

    let slope_rad = slope_angle.to_radians();
    let phi_rad = phi.to_radians();
    let base_rad = base_tilt_angle.to_radians();

    let bc = if phi == 0.0 {
        slope_rad / 5.14
    } else {
        1.0 - 2.0 * slope_rad / (5.14 * phi_rad.tan())
    };

    let bq = (1.0 - base_rad * phi_rad.tan()).powi(2);
    let bg = bq;

    BaseFactors { bc, bq, bg }
}

/// Calculates the inclination factors (ic, iq, ig) for a foundation under inclined loading.
///
/// Based on Coduto et al. (Appendix 4), accounts for both cohesive and frictional soils.
///
/// # Arguments
/// - `phi`: Internal friction angle of the soil in degrees.
/// - `cohesion`: Cohesion of the soil in kPa.
/// - `bearing_capacity_factors`: Reference to the `BearingCapacityFactors` struct.
/// - `foundation`: Reference to the `Foundation` struct (must have effective width, length, and optionally base angle).
/// - `loading`: Reference to the `Loads` struct (must have vertical load and optionally horizontal components).
///
/// # Returns
/// - `InclinationFactors`: Struct containing `ic`, `iq`, and `ig`.
pub fn calc_inclination_factors(
    phi: f64,
    cohesion: f64,
    bearing_capacity_factors: BearingCapacityFactors,
    foundation: &Foundation,
    loading: &Loads,
) -> InclinationFactors {
    let w = foundation.foundation_width.unwrap();
    let l = foundation.foundation_length.unwrap();

    let vertical_load = loading.vertical_load.unwrap();
    let hb = loading.horizontal_load_x.unwrap_or(0.);
    let hl = loading.horizontal_load_y.unwrap_or(0.);
    let hi = hb + hl;

    let effective_width = foundation.effective_width.unwrap();
    let effective_length = foundation.effective_length.unwrap();
    let area = effective_length * effective_width;

    let ca = cohesion * 0.75;
    let mb = (2. + w / l) / (1. + w / l);
    let ml = (2. + l / w) / (1. + l / w);
    let mut m = (mb.powi(2) + ml.powi(2)).sqrt();

    if hb == 0. {
        m = ml;
    } else if hl == 0. {
        m = mb;
    }

    let nc = bearing_capacity_factors.nc;
    let nq = bearing_capacity_factors.nq;

    let iq = if phi == 0. {
        1.
    } else {
        (1. - hi / (vertical_load + area * ca / phi.to_radians().tan())).powf(m)
    };

    let ic = if phi == 0. {
        1.0 - m * hi / (area * ca * nc)
    } else {
        iq - (1.0 - iq) / (nq - 1.0)
    };

    let ig = if phi == 0. {
        1.
    } else {
        (1. - hi / (vertical_load + area * ca / phi.to_radians().tan())).powf(m + 1.)
    };

    InclinationFactors { ic, iq, ig }
}

/// Calculates the depth factors (dc, dq, dg) based on foundation geometry and soil friction angle.
///
/// # Arguments
/// * `foundation` - Foundation data
/// * `phi` - Friction angle in degrees
///
/// # Returns
/// * `DepthFactors`: dc, dq, dg coefficients
pub fn calc_depth_factors(foundation: &Foundation, phi: f64) -> DepthFactors {
    let df = foundation.foundation_depth.unwrap();
    let w = foundation.foundation_width.unwrap();

    let db = if df / w <= 1.0 {
        df / w
    } else {
        (df / w).to_radians().atan()
    };

    let phi_rad = phi.to_radians();
    let tan_phi = phi_rad.tan();
    let sin_phi = phi_rad.sin();

    let dc = if phi == 0. { 0.4 * db } else { 1.0 + 0.4 * db };
    let dq = 1.0 + 2.0 * tan_phi * (1.0 - sin_phi).powi(2) * db;
    let dg = 1.0;

    DepthFactors { dc, dq, dg }
}

/// Calculates the ground modification factors (gc, gq, gg) due to slope.
///
/// # Arguments
/// * `iq` - Load inclination factor (between 0 and 1)
/// * `slope_angle` - Slope angle in degrees
/// * `phi` - Soil friction angle in degrees
///
/// # Returns
/// * `GroundFactors` with gc, gq, and gg
pub fn calc_ground_factors(iq: f64, slope_angle: f64, phi: f64) -> GroundFactors {
    let slope_rad = slope_angle.to_radians();
    let phi_rad = phi.to_radians();

    let gc = if phi == 0.0 {
        slope_rad / 5.14
    } else {
        iq - (1.0 - iq) / (5.14 * phi_rad.tan())
    };

    let tan_beta = slope_rad.tan();
    let gq = (1.0 - tan_beta).powi(2);
    let gg = gq;

    GroundFactors { gc, gq, gg }
}

/// Calculates the ultimate and allowable bearing capacity of a foundation.
///
/// # Arguments
/// * `soil_profile` - The soil profile data.
/// * `foundation` - The foundation data.
/// * `loading` - The applied loads.
/// * `foundation_pressure` - The pressure on the foundation.
/// * `factor_of_safety` - The safety factor to apply.
/// * `term` - Short or long-term condition.
///
/// # Returns
/// * `BearingCapacityResult` with detailed components and safety check.
pub fn calc_bearing_capacity(
    soil_profile: &mut SoilProfile,
    foundation: &mut Foundation,
    loading: &Loads,
    foundation_pressure: f64,
    factor_of_safety: f64,
    term: AnalysisTerm,
) -> Result<BearingCapacityResult, ValidationError> {
    // Validate input data
    validate_input(soil_profile, foundation, loading, term)?;
    soil_profile.calc_layer_depths();
    // Calculate effective foundation dimensions
    foundation.calc_effective_lengths(
        loading.moment_x.unwrap_or(0.),
        loading.moment_y.unwrap_or(0.),
    );

    let soil_params = get_soil_params(soil_profile, foundation, term);
    let phi = soil_params.friction_angle;
    let cohesion = soil_params.cohesion;
    let effective_unit_weight = soil_params.unit_weight;

    let effective_surcharge = calc_effective_surcharge(soil_profile, foundation, term);

    let bearing_capacity_factors = calc_bearing_capacity_factors(phi);
    let shape_factors = calc_shape_factors(foundation, bearing_capacity_factors, phi);
    let inclination_factors =
        calc_inclination_factors(phi, cohesion, bearing_capacity_factors, foundation, loading);
    let depth_factors = calc_depth_factors(foundation, phi);
    let base_factors = calc_base_factors(phi, foundation);
    let ground_factors = calc_ground_factors(
        inclination_factors.iq,
        foundation.slope_angle.unwrap_or(0.0),
        phi,
    );

    let q_ult = if phi == 0. {
        5.14 * cohesion
            * (1. + shape_factors.sc + depth_factors.dc
                - inclination_factors.ic
                - base_factors.bc
                - ground_factors.gc)
            + effective_surcharge
    } else {
        let part_1 = cohesion
            * bearing_capacity_factors.nc
            * shape_factors.sc
            * depth_factors.dc
            * base_factors.bc
            * ground_factors.gc
            * inclination_factors.ic;

        let part_2 = effective_surcharge
            * bearing_capacity_factors.nq
            * shape_factors.sq
            * depth_factors.dq
            * base_factors.bq
            * ground_factors.gq
            * inclination_factors.iq;

        let part_3 = 0.5
            * effective_unit_weight
            * foundation.effective_width.unwrap()
            * bearing_capacity_factors.ng
            * shape_factors.sg
            * depth_factors.dg
            * base_factors.bg
            * ground_factors.gg
            * inclination_factors.ig;

        part_1 + part_2 + part_3
    };

    let q_allow = q_ult / factor_of_safety;

    let is_safe = foundation_pressure <= q_allow;

    Ok(BearingCapacityResult {
        bearing_capacity_factors,
        shape_factors,
        depth_factors,
        load_inclination_factors: inclination_factors,
        soil_params,
        ultimate_bearing_capacity: q_ult,
        allowable_bearing_capacity: q_allow,
        is_safe,
        ground_factors,
        base_factors,
    })
}
