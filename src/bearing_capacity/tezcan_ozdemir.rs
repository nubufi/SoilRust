use crate::models::{
    foundation::Foundation,
    masw::{Masw, MaswExp},
    soil_profile::SoilProfile,
};
use serde::Serialize;

/// Represents the bearing capacity result for a given soil and foundation setup.
#[derive(Debug, Serialize)]
pub struct Output {
    /// Shear wave velocity (Vs) in m/s.
    pub vs: f64,

    /// Unit weight of the soil in t/m³.
    pub unit_weight: f64,

    /// Allowable bearing capacity in ton/m2.
    pub allowable_bearing_capacity: f64,

    /// Indicates whether the bearing capacity is safe.
    pub is_safe: bool,

    /// Safety factor used in the design.
    pub safety_factor: f64,
}

fn validate_input(
    masw_exp: &MaswExp,
    soil_profile: &SoilProfile,
    foundation: &Foundation,
) -> Result<(), &'static str> {
    if masw_exp.layers.is_empty() {
        return Err("MASW experiment layers are empty.");
    }

    if soil_profile.layers.is_empty() {
        return Err("Soil profile is empty.");
    }

    if foundation.foundation_depth <= 0.0 {
        return Err("Foundation depth must be greater than zero.");
    }

    for layer in soil_profile.layers.iter() {
        if layer.dry_unit_weight.is_none() {
            return Err("Dry unit weight must be provided for all soil layers.");
        }

        if layer.saturated_unit_weight.is_none() {
            return Err("Saturated unit weight must be provided for all soil layers.");
        }
    }
    Ok(())
}
/// Retrieves the soil parameters (unit weight and shear wave velocity) at a given depth.
///
/// # Arguments
/// - `df`: Depth at which to retrieve the soil parameters.
/// - `soil_profile`: The soil profile containing the layers and their properties.
///
/// # Returns
/// - The unit weight of the soil at the given depth.
fn get_unit_weight(df: f64, soil_profile: SoilProfile) -> f64 {
    let layer = soil_profile.get_layer_at_depth(df);

    let gwt = soil_profile.ground_water_level;

    let mut unit_weight = layer.dry_unit_weight.unwrap();

    if gwt <= df {
        unit_weight = layer.saturated_unit_weight.unwrap();
    }

    unit_weight
}

/// Calculates the ultimate bearing capacity of a foundation based on
/// shear wave velocity (Vs), soil unit weight, and empirical relationships.
/// It uses the method proposed by Tezcan and Ozdemir (2007).
///
/// # Arguments
/// - `soil_profile`: A struct containing the soil layers and properties.
/// - `masw`: A struct representing the MASW data.
/// - `foundation`: A struct representing the foundation geometry (e.g., depth).
/// - `foundation_pressure`: The pressure applied by the foundation in t/m2.
///
/// # Returns
/// - `f64`: The calculated bearing capacity in kPa.
pub fn calc_bearing_capacity(
    soil_profile: SoilProfile,
    masw: &mut Masw,
    foundation: Foundation,
    foundation_pressure: f64,
) -> Output {
    let masw_exp = masw.get_idealized_exp("idealized".to_string());
    // Validate the input parameters
    validate_input(&masw_exp, &soil_profile, &foundation).unwrap();

    let masw_layer = masw_exp.get_layer_at_depth(foundation.foundation_depth);
    let vs = masw_layer.vs;
    let df = foundation.foundation_depth;
    let unit_weight = get_unit_weight(df, soil_profile);

    let (safety_factor, bearing_capacity): (f64, f64) = match vs {
        vs if vs < 750.0 => {
            let sf = 4.0;
            let q = 0.025 * unit_weight * vs;
            (sf, q)
        }
        vs if vs < 4000.0 => {
            let sf = 4.6 - vs * 8.0e-4;
            let q = 0.1 * unit_weight * vs / sf;
            (sf, q)
        }
        _ => {
            let sf = 1.4;
            let q = 0.071 * unit_weight * vs;
            (sf, q)
        }
    };

    Output {
        vs,
        unit_weight,
        allowable_bearing_capacity: bearing_capacity,
        is_safe: bearing_capacity >= foundation_pressure,
        safety_factor,
    }
}
