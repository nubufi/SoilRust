use crate::models::{foundation::Foundation, soil_profile::SoilProfile};
use serde::{Deserialize, Serialize};

/// Represents the swelling potential data for a soil layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwellingPotentialData {
    /// The center depth of the layer in meters.
    pub layer_center: f64,
    /// The effective stress at the center of the layer in ton/m2.
    pub effective_stress: f64,
    /// The change in stress due to the foundation load in ton/m2.
    pub delta_stress: f64,
    /// The calculated swelling pressure for the layer in ton/m2.
    pub swelling_pressure: f64,
    /// Indicates whether the swelling pressure is safe compared to the effective stress.
    pub is_safe: bool,
}

/// Represents the result of the swelling potential calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwellingPotentialResult {
    pub data: Vec<SwellingPotentialData>,
    /// The net foundation pressure in ton/m2.
    pub net_foundation_pressure: f64,
}

fn validate(soil_profile: &SoilProfile, foundation: &Foundation) -> Result<(), String> {
    if foundation.foundation_depth <= 0.0 {
        return Err("Foundation depth must be greater than zero.".to_string());
    }
    if foundation.foundation_width <= 0.0 {
        return Err("Foundation width must be greater than zero.".to_string());
    }
    if foundation.foundation_length <= 0.0 {
        return Err("Foundation length must be greater than zero.".to_string());
    }
    if soil_profile.ground_water_level < 0.0 {
        return Err("Groundwater level must be greater than or equal to zero.".to_string());
    }
    if soil_profile.layers.is_empty() {
        return Err("Soil profile must contain at least one layer.".to_string());
    }

    for layer in &soil_profile.layers {
        if layer.thickness <= 0.0 {
            return Err("Thickness of soil layer must be greater than zero.".to_string());
        }
        if layer.plastic_limit.is_none() {
            return Err("Plastic limit must be provided for each soil layer.".to_string());
        }
        if layer.plastic_limit.unwrap() < 0.0 {
            return Err("Plastic limit must be greater than or equal to zero.".to_string());
        }
        if layer.dry_unit_weight.is_none() {
            return Err("Dry unit weight must be provided for each soil layer.".to_string());
        }
        if layer.dry_unit_weight.unwrap() <= 0.0 {
            return Err("Dry unit weight must be greater than zero.".to_string());
        }
        if layer.water_content.is_none() {
            return Err("Water content must be provided for each soil layer.".to_string());
        }
        if layer.water_content.unwrap() < 0.0 {
            return Err("Water content must be greater than or equal to zero.".to_string());
        }
        if layer.liquid_limit.is_none() {
            return Err("Liquid limit must be provided for each soil layer.".to_string());
        }
        if layer.liquid_limit.unwrap() < 0.0 {
            return Err("Liquid limit must be greater than or equal to zero.".to_string());
        }
    }
    Ok(())
}

/// Calculates the swelling potential of a soil profile based on the foundation parameters using
/// Kayabalu & Yaldız (2014) method.
///
/// # Arguments
/// * `soil_profile`: The soil profile containing the layers of soil.
/// * `foundation`: The foundation parameters including depth, width, and length.
/// * `foundation_pressure`: The foundation pressure applied to the soil in ton/m2.
///
/// # Returns
/// A `SwellingPotentialResult` containing the swelling potential data for each layer and the net foundation pressure.
pub fn calc_swelling_potential(
    soil_profile: &mut SoilProfile,
    foundation: &Foundation,
    foundation_pressure: f64,
) -> SwellingPotentialResult {
    validate(soil_profile, foundation).expect("Validation failed");
    soil_profile.calc_layer_depths();
    let df = foundation.foundation_depth;
    let width = foundation.foundation_width;
    let length = foundation.foundation_length;
    let net_foundation_pressure = foundation_pressure - soil_profile.calc_normal_stress(df);

    let vertical_load = net_foundation_pressure * width * length;

    let mut data = Vec::new();

    for layer in soil_profile.layers.iter() {
        let z = layer.center.unwrap();
        let mut effective_stress = 0.;
        let mut delta_stress = 0.;
        if z >= df {
            effective_stress = soil_profile.calc_effective_stress(z);
            delta_stress = vertical_load / ((width + z - df) * (length + z - df));
        }

        let swelling_pressure = if let Some(plastic_limit) = layer.plastic_limit {
            let water_content = layer.water_content.unwrap();
            let liquid_limit = layer.liquid_limit.unwrap();
            let dry_unit_weight = layer.dry_unit_weight.unwrap();
            -3.08 * water_content
                + 102.5 * dry_unit_weight
                + 0.635 * liquid_limit
                + 4.24 * plastic_limit
                - 220.8
        } else {
            0.0
        };

        let is_safe = swelling_pressure <= (effective_stress + delta_stress);

        data.push(SwellingPotentialData {
            layer_center: layer.center.unwrap(),
            effective_stress,
            delta_stress,
            swelling_pressure,
            is_safe,
        });
    }

    SwellingPotentialResult {
        data,
        net_foundation_pressure,
    }
}
