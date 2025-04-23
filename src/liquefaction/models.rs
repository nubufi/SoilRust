use crate::models::spt::SPTExp;
use serde::{Deserialize, Serialize};

/// Result of liquefaction analysis for a single layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonLiquefactionLayerResult {
    pub depth: f64,
    pub normal_stress: f64,
    pub effective_stress: f64,
    pub crr: Option<f64>,
    pub crr75: Option<f64>,
    pub csr: Option<f64>,
    pub safety_factor: Option<f64>,
    pub is_safe: bool,
    pub settlement: f64,
    pub rd: f64,
}

impl Default for CommonLiquefactionLayerResult {
    fn default() -> Self {
        Self {
            depth: 0.0,
            normal_stress: 0.0,
            effective_stress: 0.0,
            crr: None,
            crr75: None,
            csr: None,
            safety_factor: None,
            is_safe: true,
            settlement: 0.0,
            rd: 0.0,
        }
    }
}

/// Result of liquefaction analysis for a single layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSLiquefactionLayerResult {
    pub vs1: Option<f64>,
    pub vs1c: Option<f64>,
    pub cn: Option<f64>,
}

/// Result of liquefaction analysis for entire soil profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSLiquefactionResult {
    pub layers: Vec<CommonLiquefactionLayerResult>, // All layer results
    pub vs_layers: Vec<VSLiquefactionLayerResult>,  // VS layer results
    pub total_settlement: f64,                      // Sum of settlements
    pub msf: f64,                                   // Magnitude Scaling Factor
}

/// Result of liquefaction analysis for entire soil profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SptLiquefactionResult {
    pub layers: Vec<CommonLiquefactionLayerResult>, // All layer results
    pub spt_exp: SPTExp,
    pub total_settlement: f64, // Sum of settlements
    pub msf: f64,              // Magnitude Scaling Factor
}
