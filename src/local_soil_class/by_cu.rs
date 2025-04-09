use crate::models::soil_profile::SoilProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuLayerData {
    /// Layer thickness (h) in meters
    pub thickness: f64,
    /// Undrained shear strength (Cu) in t/m²
    pub cu: f64,
    /// H/Cu ratio
    pub h_over_cu: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuSoilClassificationResult {
    /// Per-layer Cu information
    pub layers: Vec<CuLayerData>,
    /// Sum of h/Cu values across all layers (unit: m / (t/m²))
    pub sum_h_over_cu: f64,
    /// (Cu)_30 value calculated from the layers
    pub cu_30: f64,
    /// Final local soil class (e.g., ZE, ZD, ZC)
    pub soil_class: String,
}

/// Calculates (cu)_30 based on the harmonic average over the top 30m of the profile.
pub fn compute_cu_30(profile: &SoilProfile) -> Vec<CuLayerData> {
    let mut remaining_depth = 30.0;
    let mut result = Vec::new();

    for layer in &profile.layers {
        if remaining_depth <= 0.0 {
            break;
        }

        let thickness = layer.thickness.min(remaining_depth);
        let cu = layer.cu.unwrap_or(0.0);

        if cu <= 0.0 {
            continue; // Skip layer with Cu == 0
        }

        let h_over_cu = thickness / cu;
        result.push(CuLayerData {
            thickness,
            cu,
            h_over_cu,
        });

        remaining_depth -= thickness;
    }

    result
}

/// Calculates the local soil class (ZE, ZD, ZC) based on the harmonic average of Cu values
/// over the top 30m of the profile.
///
/// # Arguments
///
/// * `soil_profile` - A mutable reference to a `SoilProfile` object.
///
/// # Returns
///
/// A `CuSoilClassificationResult` object containing the calculated local soil class and other related data.
pub fn calc_lsc_by_cu(soil_profile: &mut SoilProfile) -> CuSoilClassificationResult {
    soil_profile.calc_layer_depths();
    let cu_layers = compute_cu_30(soil_profile);

    let sum_h_over_cu: f64 = cu_layers.iter().map(|l| l.h_over_cu).sum();

    let depth = soil_profile.layers.last().unwrap().depth.unwrap().min(30.);

    let cu_30 = if sum_h_over_cu > 0.0 {
        depth / sum_h_over_cu
    } else {
        0.0
    };

    let soil_class = match cu_30 {
        c if c > 25.0 => "ZC",
        c if c >= 7.0 => "ZD",
        _ => "ZE",
    }
    .to_string();

    CuSoilClassificationResult {
        layers: cu_layers,
        sum_h_over_cu,
        cu_30,
        soil_class,
    }
}
