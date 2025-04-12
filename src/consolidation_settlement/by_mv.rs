use crate::models::{foundation::Foundation, soil_profile::SoilProfile};

use super::helper_functions::{calc_delta_stress, get_center_and_thickness};

fn validate(
    soil_profile: &SoilProfile,
    foundation: &Foundation,
    foundation_pressure: f64,
) -> Result<(), String> {
    if soil_profile.layers.is_empty() {
        return Err("Soil profile must contain at least one layer.".to_string());
    }
    if foundation.foundation_depth < 0.0 {
        return Err("Foundation depth must be greater than or equal to 0.".to_string());
    }
    if foundation.foundation_width <= 0.0 {
        return Err("Foundation width must be greater than 0.".to_string());
    }
    if foundation.foundation_length <= 0.0 {
        return Err("Foundation length must be greater than 0.".to_string());
    }
    if foundation_pressure < 0.0 {
        return Err("Foundation pressure must be greater than or equal to 0.".to_string());
    }
    for layer in &soil_profile.layers {
        if layer.center.is_none() {
            return Err("All soil layers must have a defined depth.".to_string());
        }
        if layer.mv.is_none() {
            return Err(
                "All soil layers must have a defined coefficient of volume compressibility."
                    .to_string(),
            );
        }
    }
    Ok(())
}

/// This module provides functions to calculate consolidation settlement using the coefficient of volume compressibility (mv).
/// It includes functions to calculate the settlement of a single layer.
///
/// #Arguments:
/// * `mv` - Coefficient of volume compressibility [m²/t]
/// * `h` - Thickness of the layer [m]
/// * `delta_stress` - Change in effective stress [t/m²]
///
/// # Returns:
/// * Settlement of the layer [cm]
pub fn calc_single_layer_settlement(mv: f64, h: f64, delta_stress: f64) -> f64 {
    mv * h * delta_stress * 100.
}

/// Calculates the consolidation settlement of a foundation based on the soil profile and foundation parameters.
///
/// # Arguments
/// * `soil_profile` - The soil profile containing the layers.
/// * `foundation` - The foundation parameters.
/// * `foundation_pressure` - The foundation pressure (q) [t/m²].
///
/// # Returns
/// * A vector of settlements for each layer in the soil profile.
pub fn calc_settlement(
    soil_profile: &SoilProfile,
    foundation: &Foundation,
    foundation_pressure: f64,
) -> Vec<f64> {
    validate(soil_profile, foundation, foundation_pressure).unwrap();
    let mut settlements = vec![];
    let df = foundation.foundation_depth;
    let width = foundation.foundation_width;
    let length = foundation.foundation_length;
    let q_net = foundation_pressure - soil_profile.calc_normal_stress(df);
    let gwt = soil_profile.ground_water_level;

    for i in 0..soil_profile.layers.len() {
        if soil_profile.get_layer_index(gwt) > i || soil_profile.get_layer_index(df) > i {
            settlements.push(0.0);
            continue;
        }
        let layer = &soil_profile.layers[i];
        let (center, thickness) = get_center_and_thickness(soil_profile, df, i);
        let mv = layer.mv.unwrap();
        let delta_stress = calc_delta_stress(q_net, width, length, center);
        let settlement = calc_single_layer_settlement(mv, thickness, delta_stress);
        settlements.push(settlement);
    }
    settlements
}
