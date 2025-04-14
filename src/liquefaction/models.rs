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
    pub vs1: Option<f64>,
    pub vs1c: Option<f64>,
    pub cn: Option<f64>,
}

impl Default for LiquefactionLayerResult {
    fn default() -> Self {
        Self {
            normal_stress: 0.0,
            effective_stress: 0.0,
            crr: None,
            crr75: None,
            csr: None,
            safety_factor: None,
            is_safe: true,
            settlement: 0.0,
            rd: 0.0,
            vs1: None,
            vs1c: None,
            cn: None,
        }
    }
}

/// Result of liquefaction analysis for entire soil profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquefactionResult {
    pub layers: Vec<LiquefactionLayerResult>, // All layer results
    pub total_settlement: f64,                // Sum of settlements
    pub msf: f64,                             // Magnitude Scaling Factor
}
