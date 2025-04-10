use serde::{Deserialize, Serialize};

use crate::models::spt::SPTExp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NLayerData {
    /// Layer thickness (h) in meters
    pub thickness: f64,
    /// N-value (N60 or N1_60f) in blows/30cm
    pub n: f64,
    /// H/N ratio
    pub h_over_n: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SptSoilClassificationResult {
    /// Per-layer N information
    pub layers: Vec<NLayerData>,
    /// Sum of h/N values across all layers (unit: m / blows)
    pub sum_h_over_n: f64,
    /// (N)_30 value calculated from the layers
    pub n_30: f64,
    /// Final local soil class (e.g., ZE, ZD, ZC)
    pub soil_class: String,
}

/// Calculates (N60)_30 based on the harmonic average over the top 30m of the profile.
pub fn compute_n_30(spt_exp: &SPTExp) -> Vec<NLayerData> {
    let mut result = Vec::new();

    let mut remaining_depth = 30.0;
    let blows = &spt_exp.blows;

    for (i, blow) in blows.iter().enumerate() {
        if remaining_depth <= 0.0 {
            break;
        }

        let previous_depth = if i == 0 { 0.0 } else { blows[i - 1].depth };

        let thickness = (blow.depth - previous_depth).min(remaining_depth);

        if thickness <= 0.0 {
            continue; // Skip invalid thickness
        }

        let n = blow.n60.unwrap().to_i32() as f64; // Refusal handled inside to_i32()

        if n <= 0.0 {
            continue; // Skip invalid or missing n values
        }

        let h_over_n = thickness / n;

        result.push(NLayerData {
            thickness,
            n,
            h_over_n,
        });

        remaining_depth -= thickness;
    }

    result
}

/// Calculates the local soil class (ZE, ZD, ZC) based on the harmonic average of N60 values
/// over the top 30m of the profile.
///
/// # Arguments
///
/// * `spt_exp` - A mutable reference to a `SptExp` object containing the spt data.
///
/// # Returns
///
/// A `SptSoilClassificationResult` object containing the calculated local soil class and other related data.
pub fn calc_lsc_by_spt(spt_exp: &mut SPTExp) -> SptSoilClassificationResult {
    let n_layers = compute_n_30(spt_exp);

    let sum_h_over_n: f64 = n_layers.iter().map(|l| l.h_over_n).sum();

    let depth = spt_exp.blows.last().unwrap().depth.min(30.);

    let n_30 = if sum_h_over_n > 0.0 {
        depth / sum_h_over_n
    } else {
        0.0
    };

    let soil_class = match n_30 {
        c if c > 50.0 => "ZC",
        c if c >= 15.0 => "ZD",
        _ => "ZE",
    }
    .to_string();

    SptSoilClassificationResult {
        layers: n_layers,
        sum_h_over_n,
        n_30,
        soil_class,
    }
}
