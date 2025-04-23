use serde::{Deserialize, Serialize};

use crate::{
    models::masw::{Masw, MaswExp},
    validation::ValidationError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VsLayerData {
    /// Layer thickness (h) in meters
    pub thickness: f64,
    /// Shear wave velocity (Vs) in m/s
    pub vs: f64,
    /// H/Vs ratio
    pub h_over_vs: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VsSoilClassificationResult {
    /// Per-layer Vs information
    pub layers: Vec<VsLayerData>,
    /// Sum of h/Vs values across all layers (unit: m / (m/s))
    pub sum_h_over_vs: f64,
    /// (Vs)_30 value calculated from the layers
    pub vs_30: f64,
    /// Final local soil class (e.g., ZE, ZD, ZC, ZB, ZA)
    pub soil_class: String,
}

/// Validates the input data for local soil class calculations.
///
/// # Arguments
/// * `masw` - The MASW data.
///
/// # Returns
/// * `Result<(), ValidationError>`: Ok if valid, Err if invalid.
pub fn validate_input(masw: &Masw) -> Result<(), ValidationError> {
    masw.validate(&["thickness", "vs"])?;

    Ok(())
}
/// Calculates (vs)_30 based on the harmonic average over the top 30m of the profile.
pub fn compute_vs_30(masw_exp: &MaswExp) -> Vec<VsLayerData> {
    let mut remaining_depth = 30.0;
    let mut result = Vec::new();

    for layer in &masw_exp.layers {
        if remaining_depth <= 0.0 {
            break;
        }

        let thickness = layer.thickness.unwrap().min(remaining_depth);
        let vs = layer.vs.unwrap();

        if vs <= 0.0 {
            continue; // Skip layer with vs == 0
        }

        let h_over_vs = thickness / vs;
        result.push(VsLayerData {
            thickness,
            vs,
            h_over_vs,
        });

        remaining_depth -= thickness;
    }

    result
}

/// Calculates the local soil class (ZE, ZD, ZC, ZB, ZA) based on the harmonic average of Vs values
/// over the top 30m of the profile.
///
/// # Arguments
///
/// * `masw` - A mutable reference to a `Masw` object containing the masw data.
///
/// # Returns
///
/// A `VsSoilClassificationResult` object containing the calculated local soil class and other related data.
pub fn calc_lsc_by_vs(masw: &mut Masw) -> Result<VsSoilClassificationResult, ValidationError> {
    validate_input(masw)?;
    let mut masw_exp = masw.get_idealized_exp("idealized".to_string());
    masw_exp.calc_depths();

    let vs_layers = compute_vs_30(&masw_exp);

    let sum_h_over_vs: f64 = vs_layers.iter().map(|l| l.h_over_vs).sum();

    let depth = masw_exp.layers.last().unwrap().depth.unwrap().min(30.);

    let vs_30 = if sum_h_over_vs > 0.0 {
        depth / sum_h_over_vs
    } else {
        0.0
    };

    let soil_class = match vs_30 {
        c if c > 1500.0 => "ZA",
        c if c >= 760.0 => "ZB",
        c if c >= 360.0 => "ZC",
        c if c >= 180.0 => "ZD",
        _ => "ZE",
    }
    .to_string();

    Ok(VsSoilClassificationResult {
        layers: vs_layers,
        sum_h_over_vs,
        vs_30,
        soil_class,
    })
}
