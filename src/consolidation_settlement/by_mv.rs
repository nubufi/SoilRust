use crate::{
    models::{foundation::Foundation, soil_profile::SoilProfile},
    validation::{validate_field, ValidationError},
};

use super::{
    helper_functions::{calc_delta_stress, get_center_and_thickness},
    model::SettlementResult,
};

/// Validates the input parameters for the consolidation settlement calculation.
///
/// # Arguments
/// * `soil_profile` - The soil profile containing the layers.
/// * `foundation` - The foundation parameters.
/// * `foundation_pressure` - The foundation pressure (q) [t/m²].
///
/// # Returns
/// * A result indicating whether the validation was successful or an error occurred.
pub fn validate_input(
    soil_profile: &SoilProfile,
    foundation: &Foundation,
    foundation_pressure: f64,
) -> Result<(), ValidationError> {
    soil_profile.validate(&["thickness", "mv"])?;
    foundation.validate(&["foundation_depth"])?;
    validate_field(
        "foundation_pressure",
        Some(foundation_pressure),
        Some(0.0),
        None,
        "loads",
    )?;
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
    soil_profile: &mut SoilProfile,
    foundation: &Foundation,
    foundation_pressure: f64,
) -> Result<SettlementResult, ValidationError> {
    validate_input(soil_profile, foundation, foundation_pressure)?;
    soil_profile.calc_layer_depths();
    let mut settlements = vec![];
    let df = foundation.foundation_depth.unwrap();
    let width = foundation.foundation_width.unwrap();
    let length = foundation.foundation_length.unwrap();
    let q_net = foundation_pressure - soil_profile.calc_normal_stress(df);
    let gwt = soil_profile.ground_water_level.unwrap();

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
    Ok(SettlementResult {
        settlement_per_layer: settlements.clone(),
        total_settlement: settlements.iter().sum(),
        qnet: q_net,
    })
}
