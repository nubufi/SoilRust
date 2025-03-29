use crate::enums::AnalysisTerm;
use crate::models::foundation::Foundation;
use crate::models::soil_profile::SoilProfile;

use super::model::SoilParams;

/// Computes the equivalent dry (γ1) and saturated (γ2) unit weights
/// up to a specified depth_limit.
/// Returns a tuple (γ1, γ2), both rounded to 3 decimal places.
pub fn compute_equivalent_unit_weights(profile: &SoilProfile, depth_limit: f64) -> (f64, f64) {
    let mut prev_depth = 0.;
    let mut gamma_dry_sum = 0.0;
    let mut gamma_saturated_sum = 0.0;

    let depth_index = profile.get_layer_index(depth_limit);

    for layer in profile.layers.iter().take(depth_index + 1) {
        let thickness = if layer.depth.unwrap() >= depth_limit {
            depth_limit - prev_depth
        } else {
            layer.thickness
        };

        gamma_dry_sum += layer.dry_unit_weight.unwrap() * thickness;
        gamma_saturated_sum += layer.saturated_unit_weight.unwrap() * thickness;

        prev_depth = layer.depth.unwrap();
    }
    let total_depth = depth_limit.min(profile.layers.last().unwrap().depth.unwrap());

    let gamma_dry = (gamma_dry_sum / total_depth * 1000.0).round() / 1000.0;
    let gamma_saturated = (gamma_saturated_sum / total_depth * 1000.0).round() / 1000.0;

    (gamma_dry, gamma_saturated)
}

/// Calculates the effective surcharge (overburden pressure) at the foundation level.
///
/// # Arguments
/// * `soil_profile` - SoilProfile with unit weights and groundwater depth.
/// * `foundation_data` - Foundation data containing foundation depth and width.
/// * `term` - Load duration term (`Short` or `Long`).
///
/// # Returns
/// * Effective vertical stress at foundation level in kPa.
pub fn calc_effective_surcharge(
    soil_profile: &SoilProfile,
    foundation_data: &Foundation,
    term: AnalysisTerm,
) -> f64 {
    let df = foundation_data.foundation_depth;
    let width = foundation_data.effective_width.unwrap();

    let (gamma_dry, gamma_saturated) = compute_equivalent_unit_weights(soil_profile, df);
    let gamma_effective = gamma_saturated - 0.981; // γ_w assumed as 0.981 tf/m³ (≈ 9.81 kN/m³)

    let gwt = match term {
        AnalysisTerm::Short => soil_profile.ground_water_level,
        AnalysisTerm::Long => df + width,
    };

    if gwt <= df {
        gamma_dry * gwt + gamma_effective * (df - gwt)
    } else {
        gamma_dry * df
    }
}

/// Calculates the effective unit weight between the surface and Df + B,
/// depending on groundwater position and load duration.
///
/// # Arguments
/// * `soil_profile` - The soil profile with layers and water level.
/// * `foundation` - The foundation depth and width.
/// * `term` - Short-term or long-term condition.
///
/// # Returns
/// * `f64`: Effective unit weight (γ') in kN/m³.
pub fn calc_effective_unit_weight(
    soil_profile: &SoilProfile,
    foundation: &Foundation,
    term: AnalysisTerm,
) -> f64 {
    let df = foundation.foundation_depth;
    let width = foundation.effective_width.unwrap();

    let (gamma_dry, gamma_saturated) = compute_equivalent_unit_weights(soil_profile, df);
    let gamma_effective = gamma_saturated - 0.981; // Subtract unit weight of water (kN/m³)

    let gwt = match term {
        AnalysisTerm::Short => soil_profile.ground_water_level,
        AnalysisTerm::Long => df + width,
    };

    if gwt <= df {
        // Entire zone is below groundwater
        gamma_effective
    } else if gwt < df + width {
        // Partially submerged zone
        let d = df + width - gwt;
        gamma_effective + d * (gamma_dry - gamma_effective) / width
    } else {
        // Entire zone is above groundwater
        gamma_dry
    }
}

/// Retrieves the soil parameters (φ, c, γ') for a given foundation depth and term.
/// The term can be either short-term or long-term.
/// The soil parameters are based on the soil layer at the foundation depth.
/// The effective unit weight is calculated based on the groundwater level and term.
/// Returns a `SoilParams` struct containing the friction angle, cohesion, and unit weight.
///
/// # Arguments
/// * `soil_profile` - The soil profile with layers and water level.
/// * `foundation` - The foundation depth and width.
/// * `term` - Short-term or long-term condition.
///
/// # Returns
/// * `SoilParams`: Soil parameters (φ, c, γ') for the foundation depth and term.
pub fn get_soil_params(
    soil_profile: &SoilProfile,
    foundation: &Foundation,
    term: AnalysisTerm,
) -> SoilParams {
    let depth = foundation.foundation_depth;
    let layer = soil_profile.get_layer_at_depth(depth);

    let (friction_angle, cohesion) = match term {
        AnalysisTerm::Short => (layer.phi_u.unwrap(), layer.cu.unwrap()),
        AnalysisTerm::Long => (layer.phi_prime.unwrap(), layer.c_prime.unwrap()),
    };

    let unit_weight = calc_effective_unit_weight(soil_profile, foundation, term);

    SoilParams {
        friction_angle,
        cohesion,
        unit_weight,
    }
}
