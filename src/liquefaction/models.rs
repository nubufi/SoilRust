use serde::{Deserialize, Serialize};

/// Result of liquefaction analysis for a single layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquefactionLayerResult {
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

/// Result of liquefaction analysis for entire soil profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquefactionResult {
    pub layers: Vec<LiquefactionLayerResult>, // All layer results
    pub total_settlement: f64,                // Sum of settlements
    pub msf: f64,                             // Magnitude Scaling Factor
}
