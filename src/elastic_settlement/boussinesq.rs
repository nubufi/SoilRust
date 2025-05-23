use crate::{
    consolidation_settlement::model::SettlementResult,
    models::{foundation::Foundation, soil_profile::SoilProfile},
    validation::{validate_field, ValidationError},
};

use super::reduction_factors::interpolate_if;

/// Validates the input data for elastic settlement calculations.
///
/// # Arguments
/// * `soil_profile` - The soil profile data.
/// * `foundation` - The foundation data.
/// * `foundation_pressure` - The foundation pressure (q) [t/m²].
///
/// # Returns
/// * `Result<(), &'static str>`: Ok if valid, Err with a message if invalid.
pub fn validate_input(
    soil_profile: &SoilProfile,
    foundation: &Foundation,
    foundation_pressure: f64,
) -> Result<(), ValidationError> {
    soil_profile.validate(&[
        "thickness",
        "dry_unit_weight",
        "saturated_unit_weight",
        "elastic_modulus",
        "poissons_ratio",
    ])?;
    foundation.validate(&["foundation_depth", "foundation_width", "foundation_length"])?;

    validate_field(
        "foundation_pressure",
        Some(foundation_pressure),
        Some(0.0),
        None,
        "loads",
    )?;

    Ok(())
}

/// Calculates the influence factor (Ip) for settlement under a rectangular foundation
///
/// # Arguments
/// * `h` - Depth of the layer (H) [m]
/// * `b` - Width of foundation (B) [m]
/// * `l` - Length of foundation (L) [m]
/// * `u` - Poisson's ratio of the soil (ν) [-]
///
/// # Returns
/// * `Ip` - Influence factor (dimensionless)
///
/// # Reference
/// Bowles, J.E. (1996). *Foundation Analysis and Design*, 5th Ed.
pub fn calc_ip(h: f64, b: f64, l: f64, u: f64) -> f64 {
    let m = l / b;
    let n = 2.0 * h / b;

    let m2 = m * m;
    let n2 = n * n;

    let a0 = m
        * ((1.0 + (1.0 + m2).sqrt()) * (m2 + n2).sqrt() / (m * (1.0 + (1.0 + m2 + n2).sqrt())))
            .ln();
    let a1 = ((m + (1.0 + m2).sqrt()) * (1.0 + n2).sqrt() / (m + (1.0 + m2 + n2).sqrt())).ln();
    let a2 = m / (n * (1.0 + m2 + n2).sqrt());

    let f1 = (a0 + a1) / std::f64::consts::PI;
    let f2 = 0.5 * (n / std::f64::consts::PI) * a2.atan();

    f1 + ((1.0 - 2.0 * u) / (1.0 - u)) * f2
}

/// Calculates the settlement (S) of a single soil layer under a rectangular foundation.
///
/// # Arguments
/// * `h` - Thickness of the soil layer (H) [m]
/// * `u` - Poisson's ratio of the soil (ν) [-]
/// * `e` - Elastic Modulus of the soil (E) [kPa]
/// * `l` - Length of the foundation (L) [m]
/// * `b` - Width of the foundation (B) [m]
/// * `df` - Depth of foundation (Df) [m]
/// * `q_net` - Net foundation pressure (qNet) [t/m²]
///
/// # Returns
/// * `S` - Settlement in centimeters [cm]
///
/// # Formula
/// S = 100 * qNet * 4 * B * If * Ip * (1 - u²) * 0.5 / E
///
/// Reference: Bowles, J.E. (1996)
pub fn single_layer_settlement(h: f64, u: f64, e: f64, l: f64, b: f64, df: f64, q_net: f64) -> f64 {
    let lb = l / b;
    let db = df / b;
    let ip = calc_ip(h, b, l, u);
    let if_value = interpolate_if(u, db, lb);

    100.0 * q_net * 4.0 * b * if_value * ip * (1.0 - u.powi(2)) * 0.5 / e
}

/// Calculates the elastic settlement of a foundation based on the soil profile and foundation parameters.
///
/// # Arguments
/// * `soil_profile` - The soil profile containing the layers of soil.
/// * `foundation` - The foundation parameters.
/// * `foundation_pressure` - The foundation pressure (q) [t/m²].
///
/// # Returns
/// * A vector of settlements for each layer in the soil profile.
///
/// Reference: Bowles, J.E. (1996)
pub fn calc_elastic_settlement(
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
    let df_index = soil_profile.get_layer_index(df);

    for i in 0..soil_profile.layers.len() {
        let layer = &soil_profile.layers[i];
        let h = layer.depth.unwrap() - df;
        let u = layer.poissons_ratio.unwrap();
        let e = layer.elastic_modulus.unwrap();

        if i < df_index {
            settlements.push(0.0);
        } else {
            let settlement_all = single_layer_settlement(h, u, e, length, width, df, q_net);
            if i == 0 {
                settlements.push(settlement_all.max(0.));
            } else {
                let h0 = soil_profile.layers[i - 1].depth.unwrap() - df;
                let settlement_prevlayer =
                    single_layer_settlement(h0, u, e, length, width, df, q_net);
                settlements.push((settlement_all - settlement_prevlayer).max(0.));
            }
        }
    }

    Ok(SettlementResult {
        settlement_per_layer: settlements.clone(),
        total_settlement: settlements.iter().sum(),
        qnet: q_net,
    })
}
