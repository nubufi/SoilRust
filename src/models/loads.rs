// File: loads.rs

/// Load severity
///
/// # Variants
/// * `Min` - Minimum case
/// * `Avg` - Average case
/// * `Max` - Maximum case
#[derive(Debug, Clone, Copy)]
pub enum LoadSeverity {
    Min, // Minimum case
    Avg, // Average case
    Max, // Maximum case
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

/// Stress values in ton/m^2
///
/// # Fields
/// * `min` - Minimum vertical stress in ton/m^2
/// * `avg` - Average vertical stress in ton/m^2
/// * `max` - Maximum vertical stress in ton/m^2
#[derive(Debug, Clone, Copy)]
pub struct Stress {
    pub min: Option<f64>,
    pub avg: Option<f64>,
    pub max: Option<f64>,
}

impl Default for Stress {
    fn default() -> Self {
        Stress {
            min: None,
            avg: None,
            max: None,
        }
    }
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
#[derive(Debug, Clone)]
pub struct Loads {
    pub service_load: Stress,
    pub ultimate_load: Stress,
    pub seismic_load: Stress,
    pub horizontal_load_x: Option<f64>,
    pub horizontal_load_y: Option<f64>,
    pub moment_x: Option<f64>,
    pub moment_y: Option<f64>,
}

impl Default for Loads {
    fn default() -> Self {
        Loads {
            service_load: Default::default(),
            ultimate_load: Default::default(),
            seismic_load: Default::default(),
            horizontal_load_x: None,
            horizontal_load_y: None,
            moment_x: None,
            moment_y: None,
        }
    }
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
    pub fn get_vertical_stress(&self, load_case: LoadCase, load_severity: LoadSeverity) -> f64 {
        match load_case {
            LoadCase::ServiceLoad => match load_severity {
                LoadSeverity::Min => self.service_load.min.unwrap_or(0.),
                LoadSeverity::Avg => self.service_load.avg.unwrap_or(0.),
                LoadSeverity::Max => self.service_load.max.unwrap_or(0.),
            },
            LoadCase::UltimateLoad => match load_severity {
                LoadSeverity::Min => self.ultimate_load.min.unwrap_or(0.),
                LoadSeverity::Avg => self.ultimate_load.avg.unwrap_or(0.),
                LoadSeverity::Max => self.ultimate_load.max.unwrap_or(0.),
            },
            LoadCase::SeismicLoad => match load_severity {
                LoadSeverity::Min => self.seismic_load.min.unwrap_or(0.),
                LoadSeverity::Avg => self.seismic_load.avg.unwrap_or(0.),
                LoadSeverity::Max => self.seismic_load.max.unwrap_or(0.),
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
    pub fn calc_eccentricity(&self, vertical_load: f64) -> (f64, f64) {
        if vertical_load == 0.0 {
            return (0.0, 0.0);
        }
        if let (Some(mx), Some(my)) = (self.moment_x, self.moment_y) {
            let ex = mx / vertical_load;
            let ey = my / vertical_load;
            (ex, ey)
        } else {
            (0.0, 0.0)
        }
    }
}
