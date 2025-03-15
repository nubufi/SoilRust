/// Represents a foundation with geometry and load effects.
///
/// # Fields
///
/// * `foundation_depth` - Depth of the foundation (m).
/// * `foundation_length` - Length of the foundation (m).
/// * `foundation_width` - Width of the foundation (m).
/// * `foundation_area` - Area of the foundation (m²).
/// * `effective_length` - Effective length of the foundation after load effects (m).
/// * `effective_width` - Effective width of the foundation after load effects (m).
/// * `foundation_angle` - Foundation inclination angle (degrees).
/// * `slope_angle` - Slope angle of the ground (degrees).
#[derive(Debug, Clone)]
pub struct Foundation {
    /// Depth of the foundation (m).
    pub foundation_depth: f64,
    /// Length of the foundation (m).
    pub foundation_length: f64,
    /// Width of the foundation (m).
    pub foundation_width: f64,
    /// Area of the foundation (m²).
    pub foundation_area: Option<f64>,
    /// Effective length of the foundation after load effects (m).
    pub effective_length: Option<f64>,
    /// Effective width of the foundation after load effects (m).
    pub effective_width: Option<f64>,
    /// Foundation inclination angle (degrees).
    pub foundation_angle: Option<f64>,
    /// Slope angle of the ground (degrees).
    pub slope_angle: Option<f64>,
}

impl Default for Foundation {
    fn default() -> Self {
        Self {
            foundation_depth: 0.,
            foundation_length: 0.,
            foundation_width: 0.,
            foundation_area: None,
            effective_length: None,
            effective_width: None,
            foundation_angle: None,
            slope_angle: None,
        }
    }
}

impl Foundation {
    /// Calculates effective lengths based on applied loads.
    ///
    /// # Arguments
    ///
    /// * `ex` - Eccentricity in x-direction (m).
    /// * `ey` - Eccentricity in y-direction (m).
    pub fn calc_effective_lengths(&mut self, ex: f64, ey: f64) {
        let b_ = self.foundation_width - 2.0 * ex;
        let l_ = self.foundation_length - 2.0 * ey;

        self.effective_width = Some(f64::min(b_, l_).max(0.0));
        self.effective_length = Some(f64::max(b_, l_).max(0.0));
    }
}
