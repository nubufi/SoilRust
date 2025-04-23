use serde::{Deserialize, Serialize};

use crate::validation::{validate_field, ValidationError};

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
/// * `base_tilt_angle` - Foundation inclination angle (degrees).
/// * `slope_angle` - Slope angle of the ground (degrees).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Foundation {
    /// Depth of the foundation (m).
    pub foundation_depth: Option<f64>,
    /// Length of the foundation (m).
    pub foundation_length: Option<f64>,
    /// Width of the foundation (m).
    pub foundation_width: Option<f64>,
    /// Area of the foundation (m²).
    pub foundation_area: Option<f64>,
    /// Foundation inclination angle (degrees).
    pub base_tilt_angle: Option<f64>,
    /// Slope angle of the ground (degrees).
    pub slope_angle: Option<f64>,
    /// Effective length of the foundation after load effects (m).
    pub effective_length: Option<f64>,
    /// Effective width of the foundation after load effects (m).
    pub effective_width: Option<f64>,
    /// Friction coefficient for horizontal sliding (unitless).
    pub surface_friction_coefficient: Option<f64>,
}

impl Foundation {
    /// Creates a new `Foundation` instance.
    ///
    /// # Arguments
    /// * `depth` - Depth of the foundation (m).
    /// * `length` - Length of the foundation (m).
    /// * `width` - Width of the foundation (m).
    /// * `angle` - Foundation inclination angle (degrees).
    /// * `slope` - Slope angle of the ground (degrees).
    /// * `area` - Area of the foundation (m²).
    ///
    /// # Returns
    /// A new `Foundation` instance.
    pub fn new(
        depth: Option<f64>,
        length: Option<f64>,
        width: Option<f64>,
        angle: Option<f64>,
        slope: Option<f64>,
        area: Option<f64>,
        surface_friction_coefficient: Option<f64>,
    ) -> Self {
        Self {
            foundation_depth: depth,
            foundation_length: length,
            foundation_width: width,
            foundation_area: area,
            base_tilt_angle: angle,
            slope_angle: slope,
            effective_length: None,
            effective_width: None,
            surface_friction_coefficient,
        }
    }
    /// Calculates effective lengths based on applied loads.
    ///
    /// # Arguments
    ///
    /// * `ex` - Eccentricity in x-direction (m).
    /// * `ey` - Eccentricity in y-direction (m).
    pub fn calc_effective_lengths(&mut self, ex: f64, ey: f64) {
        let b_ = self.foundation_width.unwrap() - 2.0 * ex;
        let l_ = self.foundation_length.unwrap() - 2.0 * ey;

        self.effective_width = Some(f64::min(b_, l_).max(0.0));
        self.effective_length = Some(f64::max(b_, l_).max(0.0));
    }

    /// Validates specific fields of the Foundation using field names.
    /// This enables context-specific validation like `["foundation_depth", "effective_width"]`
    ///
    /// # Arguments
    /// * `fields` - A slice of field names to validate.
    ///
    /// # Returns
    /// Ok(()) if all fields are valid, or an error if any field is invalid.
    pub fn validate(&self, fields: &[&str]) -> Result<(), ValidationError> {
        for &field in fields {
            let result = match field {
                "foundation_depth" => validate_field(
                    "foundation_depth",
                    self.foundation_depth,
                    Some(0.0),
                    None,
                    "foundation",
                ),

                "foundation_length" => validate_field(
                    "foundation_length",
                    self.foundation_length,
                    Some(0.0001),
                    None,
                    "foundation",
                ),

                "foundation_width" => validate_field(
                    "foundation_width",
                    self.foundation_width,
                    Some(0.001),
                    self.foundation_length,
                    "foundation",
                ),

                "foundation_area" => validate_field(
                    "foundation_area",
                    self.foundation_area,
                    Some(0.001),
                    None,
                    "foundation",
                ),

                "base_tilt_angle" => validate_field(
                    "base_tilt_angle",
                    self.base_tilt_angle,
                    Some(0.0),
                    Some(45.0),
                    "foundation",
                ),

                "slope_angle" => validate_field(
                    "slope_angle",
                    self.slope_angle,
                    Some(0.0),
                    Some(90.0),
                    "foundation",
                ),

                "effective_width" => validate_field(
                    "effective_width",
                    self.effective_width,
                    Some(0.0),
                    None,
                    "foundation",
                ),

                "effective_length" => validate_field(
                    "effective_length",
                    self.effective_length,
                    Some(0.0),
                    None,
                    "foundation",
                ),

                "surface_friction_coefficient" => validate_field(
                    "surface_friction_coefficient",
                    self.surface_friction_coefficient,
                    Some(0.0),
                    Some(1.0),
                    "foundation",
                ),

                unknown => Err(ValidationError {
                    code: "foundation.invalid_field".into(),
                    message: format!("Field '{}' is not valid for Foundation.", unknown),
                }),
            };

            result?; // propagate error if any field fails
        }

        Ok(())
    }
}
