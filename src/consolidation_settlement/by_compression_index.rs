use crate::{
    models::{foundation::Foundation, soil_profile::SoilProfile},
    validation::{validate_field, ValidationError},
};

use super::{
    helper_functions::{calc_delta_stress, get_center_and_thickness},
    model::SettlementResult,
};

pub fn validate_input(
    soil_profile: &SoilProfile,
    foundation: &Foundation,
    foundation_pressure: f64,
) -> Result<(), ValidationError> {
    soil_profile.validate(&[
        "thickness",
        "compression_index",
        "recompression_index",
        "void_ratio",
        "preconsolidation_pressure",
    ])?;
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
        let delta_stress = calc_delta_stress(q_net, width, length, center);
        let g0 = soil_profile.calc_effective_stress(center);
        let cc = layer.compression_index.unwrap();
        let cr = layer.recompression_index.unwrap();
        let e0 = layer.void_ratio.unwrap();
        let gp = layer.preconsolidation_pressure.unwrap();
        let settlement = calc_single_layer_settlement(thickness, cc, cr, e0, gp, g0, delta_stress);
        settlements.push(settlement);
    }
    Ok(SettlementResult {
        settlement_per_layer: settlements.clone(),
        total_settlement: settlements.iter().sum(),
        qnet: q_net,
    })
}
