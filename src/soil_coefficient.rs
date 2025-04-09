/// Calculates the soil coefficient based on settlement and foundation load.
/// Returns a high value (999_999.0) if settlement is zero or negative to avoid division by zero.
///
/// # Arguments
///
/// * `settlement` - The settlement of the foundation in meters.
/// * `foundation_load` - The load on the foundation in tons.
///
/// # Returns
/// * The soil coefficient in tons per cubic meter (t/m³).
pub fn calc_by_settlement(settlement: f64, vertical_load: f64) -> f64 {
    if settlement <= 0.0 {
        return 999_999.0;
    }
    100.0 * vertical_load / settlement // units: t/m³
}

/// Calculates the soil coefficient based on bearing capacity.
/// Uses a factor of 400 as specified in empirical design practice.
///
/// # Arguments
///
/// * `bearing_capacity` - The bearing capacity of the soil in tons per square meter (t/m²).
///
/// # Returns
/// * The soil coefficient in tons per cubic meter (t/m³).
pub fn calc_by_bearing_capacity(bearing_capacity: f64) -> f64 {
    400.0 * bearing_capacity // units: t/m³
}
