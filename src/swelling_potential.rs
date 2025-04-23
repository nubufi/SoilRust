use crate::{
    models::{foundation::Foundation, soil_profile::SoilProfile},
    validation::{validate_field, ValidationError},
};
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

/// Validates the input data for swelling potential calculations.
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
        "water_content",
        "liquid_limit",
        "plastic_limit",
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
) -> Result<SwellingPotentialResult, ValidationError> {
    validate_input(soil_profile, foundation, foundation_pressure)?;
    soil_profile.calc_layer_depths();
    let df = foundation.foundation_depth.unwrap();
    let width = foundation.foundation_width.unwrap();
    let length = foundation.foundation_length.unwrap();

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

    Ok(SwellingPotentialResult {
        data,
        net_foundation_pressure,
    })
}
