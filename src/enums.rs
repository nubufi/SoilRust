#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub enum LoadCase {
    ServiceLoad,
    UltimateLoad,
    SeismicLoad,
}
