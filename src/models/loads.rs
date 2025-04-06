use crate::enums::{LoadCase, SelectionMethod};
use serde::{Deserialize, Serialize};

/// Stress values in ton/m^2
///
/// # Fields
/// * `min` - Minimum vertical stress in ton/m^2
/// * `avg` - Average vertical stress in ton/m^2
/// * `max` - Maximum vertical stress in ton/m^2
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Stress {
    pub min: Option<f64>,
    pub avg: Option<f64>,
    pub max: Option<f64>,
}

/// Loading conditions
///
/// # Fields
/// * `service_load` - Service load stress values
/// * `ultimate_load` - Ultimate load stress values
/// * `seismic_load` - Seismic load stress values
/// * `horizontal_load_x` - Horizontal load in x-direction in ton
/// * `horizontal_load_y` - Horizontal load in y-direction in ton
/// * `moment_x` - Moment in x-direction in ton.m
/// * `moment_y` - Moment in y-direction in ton.m
/// * `vertical_load` - Vertical load in ton
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Loads {
    pub service_load: Stress,
    pub ultimate_load: Stress,
    pub seismic_load: Stress,
    pub horizontal_load_x: Option<f64>,
    pub horizontal_load_y: Option<f64>,
    pub moment_x: Option<f64>,
    pub moment_y: Option<f64>,
    pub vertical_load: Option<f64>,
}

impl Loads {
    /// Get vertical stress value in ton/m^2 for specified load_case and load_severity.
    ///
    /// # Arguments
    /// * `load_case` - Load case
    /// * `load_severity` - Load severity
    ///
    /// # Returns
    /// * Vertical stress value in ton/m^2
    pub fn get_vertical_stress(&self, load_case: LoadCase, load_severity: SelectionMethod) -> f64 {
        match load_case {
            LoadCase::ServiceLoad => match load_severity {
                SelectionMethod::Min => self.service_load.min.unwrap_or(0.),
                SelectionMethod::Avg => self.service_load.avg.unwrap_or(0.),
                SelectionMethod::Max => self.service_load.max.unwrap_or(0.),
            },
            LoadCase::UltimateLoad => match load_severity {
                SelectionMethod::Min => self.ultimate_load.min.unwrap_or(0.),
                SelectionMethod::Avg => self.ultimate_load.avg.unwrap_or(0.),
                SelectionMethod::Max => self.ultimate_load.max.unwrap_or(0.),
            },
            LoadCase::SeismicLoad => match load_severity {
                SelectionMethod::Min => self.seismic_load.min.unwrap_or(0.),
                SelectionMethod::Avg => self.seismic_load.avg.unwrap_or(0.),
                SelectionMethod::Max => self.seismic_load.max.unwrap_or(0.),
            },
        }
    }
    /// Calculates the eccentricity of the loading.
    ///
    /// # Arguments
    /// * `vertical_load` - Vertical load in ton (or equivalent unit).
    ///
    /// # Returns
    /// * `(ex, ey)` - Eccentricities in meters (or equivalent unit).
    ///
    /// # Note
    /// If `vertical_load` is zero, it returns `(0.0, 0.0)` to prevent division by zero.
    pub fn calc_eccentricity(&self) -> (f64, f64) {
        if self.vertical_load.is_none() || self.vertical_load.unwrap() == 0.0 {
            return (0.0, 0.0);
        }
        if let (Some(mx), Some(my)) = (self.moment_x, self.moment_y) {
            let ex = mx / self.vertical_load.unwrap();
            let ey = my / self.vertical_load.unwrap();
            (ex, ey)
        } else {
            (0.0, 0.0)
        }
    }
}
