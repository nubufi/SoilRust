use std::f64::consts::PI;

use crate::models::{foundation::Foundation, loads::Loads, soil_profile::SoilProfile};

pub struct HorizontalSlidingResult {
    pub rth: f64,
    pub ptv: f64,
    pub rpk_x: f64,
    pub rpk_y: f64,
    pub rpt_x: f64,
    pub rpt_y: f64,
    pub sum_x: f64,
    pub sum_y: f64,
    pub is_safe_x: bool,
    pub is_safe_y: bool,
    pub ac: f64,
    pub vth_x: f64,
    pub vth_y: f64,
}

fn validate(
    soil_profile: &SoilProfile,
    foundation: &Foundation,
    loads: &Loads,
    foundation_pressure: f64,
) {
    if foundation_pressure <= 0.0 {
        panic!("Foundation pressure must be greater than zero.");
    }
    if foundation.foundation_depth <= 0.0 {
        panic!("Foundation depth must be greater than zero.");
    }
    if foundation.foundation_width <= 0.0 {
        panic!("Foundation width must be greater than zero.");
    }
    if foundation.foundation_length <= 0.0 {
        panic!("Foundation length must be greater than zero.");
    }
    if soil_profile.ground_water_level < 0.0 {
        panic!("Groundwater level must be greater than or equal to zero.");
    }
    if soil_profile.layers.is_empty() {
        panic!("Soil profile must contain at least one layer.");
    }
    if foundation.surface_friction_coefficient.is_none() {
        panic!("Surface friction coefficient must be provided.");
    }
    if foundation.surface_friction_coefficient.unwrap() < 0.0 {
        panic!("Surface friction coefficient must be greater than or equal to zero.");
    }
    if foundation.surface_friction_coefficient.unwrap() > 1.0 {
        panic!("Surface friction coefficient must be less than or equal to one.");
    }
    if foundation.foundation_depth > soil_profile.ground_water_level {
        panic!("Foundation depth cannot be greater than groundwater level.");
    }
    if loads.horizontal_load_x.is_none() {
        panic!("Horizontal load in X direction must be provided.");
    }
    if loads.horizontal_load_y.is_none() {
        panic!("Horizontal load in Y direction must be provided.");
    }
}

/// Extracts cohesion, friction angle, and unit weight based on groundwater level and soil properties.
fn get_soil_params(soil_profile: &SoilProfile, df: f64) -> (f64, f64, f64) {
    let layer = soil_profile.get_layer_at_depth(df);

    let c_prime = layer.c_prime.unwrap();
    let cu = layer.cu.unwrap();
    let phi_prime = layer.phi_prime.unwrap();
    let phi_u = layer.phi_u.unwrap();
    let dry_unit_weight = layer.dry_unit_weight.unwrap();
    let saturated_unit_weight = layer.saturated_unit_weight.unwrap();

    let (selected_unit_weight, selected_cohesion, selected_phi) =
        if soil_profile.ground_water_level <= df {
            (saturated_unit_weight - 1.0, cu, phi_u)
        } else {
            (dry_unit_weight, c_prime, phi_prime)
        };

    (selected_cohesion, selected_phi, selected_unit_weight)
}

/// Calculates horizontal sliding stability based on foundation and soil parameters.
///
/// # Arguments
///
/// * `soil_profile` - The soil profile containing soil layers and properties.
/// * `foundation` - The foundation parameters including dimensions and friction coefficient.
/// * `loads` - The loads acting on the foundation.
/// * `foundation_pressure` - The pressure exerted by the foundation on the soil.
///
/// # Returns
/// A `HorizontalSlidingResult` struct containing the calculated values and safety checks.
pub fn calc_horizontal_sliding(
    soil_profile: &SoilProfile,
    foundation: &Foundation,
    loads: &Loads,
    foundation_pressure: f64,
) -> HorizontalSlidingResult {
    validate(soil_profile, foundation, loads, foundation_pressure);
    let df = foundation.foundation_depth;
    let b = foundation.foundation_width;
    let l = foundation.foundation_length;

    let vx = loads.horizontal_load_x.unwrap();
    let vy = loads.horizontal_load_y.unwrap();
    let surface_friction = foundation.surface_friction_coefficient.unwrap();

    let ptv = foundation_pressure * b * l;

    let (cohesion, phi, unit_weight) = get_soil_params(soil_profile, df);

    let kp = (f64::tan((45.0 + phi / 2.0) * PI / 180.0)).powi(2);

    let rth = if soil_profile.ground_water_level > df {
        ptv * surface_friction / 1.1
    } else {
        l * b * cohesion / 1.1
    };

    let rpk_x = b * 0.5 * df.powi(2) * unit_weight * kp;
    let rpk_y = l * 0.5 * df.powi(2) * unit_weight * kp;

    let rpt_x = rpk_x / 1.4;
    let rpt_y = rpk_y / 1.4;

    let sum_x = rth + 0.3 * rpt_x;
    let sum_y = rth + 0.3 * rpt_y;

    HorizontalSlidingResult {
        rth,
        ptv,
        rpk_x,
        rpk_y,
        rpt_x,
        rpt_y,
        sum_x,
        sum_y,
        is_safe_x: vx <= sum_x,
        is_safe_y: vy <= sum_y,
        ac: l * b,
        vth_x: vx,
        vth_y: vy,
    }
}
