use crate::models::{foundation::Foundation, soil_profile::SoilProfile};

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

/// Calculates the center and thickness of a soil layer based on the ground water table (GWT) and the depth of the foundation (df).
///
/// # Arguments
/// * `soil_profile` - The soil profile containing the layers.
/// * `df` - The depth of the foundation.
/// * `layer_index` - The index of the layer.
///
/// # Returns
/// * A tuple containing the center and thickness of the layer.
pub fn get_center_and_thickness(
    soil_profile: &SoilProfile,
    df: f64,
    layer_index: usize,
) -> (f64, f64) {
    let gwt = soil_profile.ground_water_level;
    let gwt_layer_index = soil_profile.get_layer_index(gwt);
    let df_layer_index = soil_profile.get_layer_index(df);
    let layer = &soil_profile.layers[layer_index];

    let (center, thickness) = if gwt_layer_index < layer_index {
        if layer_index == df_layer_index {
            let thickness = layer.thickness - df;
            let center = df + thickness / 2.0;
            (center, thickness)
        } else {
            let thickness = layer.thickness;
            let center = layer.center.expect("Layer center must be Some");
            (center, thickness)
        }
    } else {
        let max_depth = df.max(gwt);
        let thickness = layer.thickness - max_depth;
        let center = max_depth + thickness / 2.0;
        (center, thickness)
    };

    (center, thickness)
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

    for i in 0..soil_profile.layers.len() {
        let layer = &soil_profile.layers[i];
        let (center, thickness) = get_center_and_thickness(soil_profile, df, i);
        let mv = layer.mv.unwrap();
        let delta_stress = q_net * width * length / (width + center) * (length + center);
        let settlement = calc_single_layer_settlement(mv, thickness, delta_stress);
        settlements.push(settlement);
    }
    settlements
}
