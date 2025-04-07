use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum SelectionMethod {
    Min,
    Avg,
    Max,
}

/// Load cases
///
/// # Variants
/// * `ServiceLoad` - Service load case (G + Q)
/// * `UltimateLoad` - Ultimate load case (1.4G + 1.6Q)
/// * `SeismicLoad` - Seismic load case (G + Q + E/0.9G + E)
///
/// # Note
/// * `G` - Dead load
/// * `Q` - Live load
/// * `E` - Earthquake load
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum LoadCase {
    ServiceLoad,
    UltimateLoad,
    SeismicLoad,
}

/// Analysis term
///
/// # Variants
/// * `Short` - Short term analysis
/// * `Long` - Long term analysis
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum AnalysisTerm {
    Short,
    Long,
}
