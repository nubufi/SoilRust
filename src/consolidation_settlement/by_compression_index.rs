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
        if layer.compression_index.is_none() {
            return Err("All soil layers must have a defined compression index.".to_string());
        }
        if layer.recompression_index.is_none() {
            return Err("All soil layers must have a defined recompression index.".to_string());
        }
        if layer.void_ratio.is_none() {
            return Err("All soil layers must have a defined void ratio.".to_string());
        }
        if layer.preconsolidation_pressure.is_none() {
            return Err(
                "All soil layers must have a defined preconsolidation pressure.".to_string(),
            );
        }
    }
    Ok(())
}
/// Calculates consolidation settlement using Cc-Cr method (logarithmic compression)
///
/// # Arguments
/// * `h` - Thickness of the compressible layer [m]
/// * `cc` - Compression Index (Cc)
/// * `cr` - Recompression Index (Cr)
/// * `e0` - Initial Void Ratio (e₀)
/// * `gp` - Preconsolidation Pressure [kPa]
/// * `g0` - Initial Effective Stress [kPa]
/// * `delta_stress` - Stress increase due to foundation [kPa]
///
/// # Returns
/// Settlement [cm]
pub fn calc_single_layer_settlement(
    h: f64,
    cc: f64,
    cr: f64,
    e0: f64,
    gp: f64,
    g0: f64,
    delta_stress: f64,
) -> f64 {
    let log10 = |x: f64| x.log10();

    let settlement = if g0 >= gp {
        cc * (h / (1.0 + e0)) * log10((delta_stress + g0) / g0)
    } else if (delta_stress + g0) <= gp {
        cr * (h / (1.0 + e0)) * log10((delta_stress + g0) / g0)
    } else {
        cr * (h / (1.0 + e0)) * log10(gp / g0)
            + cc * (h / (1.0 + e0)) * log10((delta_stress + g0) / gp)
    };

    settlement.max(0.0) * 100.0 // Convert to cm
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
        let delta_stress = calc_delta_stress(q_net, width, length, center);
        let g0 = soil_profile.calc_effective_stress(center);
        let cc = layer.compression_index.unwrap();
        let cr = layer.recompression_index.unwrap();
        let e0 = layer.void_ratio.unwrap();
        let gp = layer.preconsolidation_pressure.unwrap();
        let settlement = calc_single_layer_settlement(thickness, cc, cr, e0, gp, g0, delta_stress);
        settlements.push(settlement);
    }
    settlements
}
