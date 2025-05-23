use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementResult {
    pub settlement_per_layer: Vec<f64>,
    pub total_settlement: f64,
    pub qnet: f64,
}
