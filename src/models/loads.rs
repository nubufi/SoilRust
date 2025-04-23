use crate::{
    enums::{LoadCase, SelectionMethod},
    validation::{validate_field, ValidationError},
};
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

impl Stress {
    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_field("min", self.min, None, None, "loads")?;
        validate_field("avg", self.avg, None, None, "loads")?;
        validate_field("max", self.max, None, None, "loads")?;
        Ok(())
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
/// * `vertical_load` - Vertical load in ton
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Loads {
    pub service_load: Option<Stress>,
    pub ultimate_load: Option<Stress>,
    pub seismic_load: Option<Stress>,
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
                SelectionMethod::Min => self.service_load.unwrap().min.unwrap_or(0.),
                SelectionMethod::Avg => self.service_load.unwrap().avg.unwrap_or(0.),
                SelectionMethod::Max => self.service_load.unwrap().max.unwrap_or(0.),
            },
            LoadCase::UltimateLoad => match load_severity {
                SelectionMethod::Min => self.ultimate_load.unwrap().min.unwrap_or(0.),
                SelectionMethod::Avg => self.ultimate_load.unwrap().avg.unwrap_or(0.),
                SelectionMethod::Max => self.ultimate_load.unwrap().max.unwrap_or(0.),
            },
            LoadCase::SeismicLoad => match load_severity {
                SelectionMethod::Min => self.seismic_load.unwrap().min.unwrap_or(0.),
                SelectionMethod::Avg => self.seismic_load.unwrap().avg.unwrap_or(0.),
                SelectionMethod::Max => self.seismic_load.unwrap().max.unwrap_or(0.),
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
    /// Validates specific fields of the Loads using field names.
    /// This enables context-specific validation like `["vertical_load", "moment_x"]`.
    ///
    /// # Arguments
    /// * `fields` - A slice of field names to validate.
    ///
    /// # Returns
    /// Ok(()) if all fields are valid, or an error if any field is invalid.
    pub fn validate(&self, fields: &[&str]) -> Result<(), ValidationError> {
        for &field in fields {
            let result = match field {
                "horizontal_load_x" => validate_field(
                    "horizontal_load_x",
                    self.horizontal_load_x,
                    Some(0.0),
                    None,
                    "loads",
                ),
                "horizontal_load_y" => validate_field(
                    "horizontal_load_y",
                    self.horizontal_load_y,
                    Some(0.0),
                    None,
                    "loads",
                ),
                "moment_x" => validate_field("moment_x", self.moment_x, Some(0.0), None, "loads"),
                "moment_y" => validate_field("moment_y", self.moment_y, Some(0.0), None, "loads"),
                "vertical_load" => validate_field(
                    "vertical_load",
                    self.vertical_load,
                    Some(0.0),
                    None,
                    "loads",
                ),
                "service_load" => {
                    if let Some(service_load) = &self.service_load {
                        service_load.validate()
                    } else {
                        Err(ValidationError {
                            code: "loads.service_load_not_set".into(),
                            message: "Service load is not set.".into(),
                        })
                    }
                }
                "ultimate_load" => {
                    if let Some(ultimate_load) = &self.ultimate_load {
                        ultimate_load.validate()
                    } else {
                        Err(ValidationError {
                            code: "loads.ultimate_load_not_set".into(),
                            message: "Ultimate load is not set.".into(),
                        })
                    }
                }
                "seismic_load" => {
                    if let Some(seismic_load) = &self.seismic_load {
                        seismic_load.validate()
                    } else {
                        Err(ValidationError {
                            code: "loads.seismic_load_not_set".into(),
                            message: "Seismic load is not set.".into(),
                        })
                    }
                }

                unknown => Err(ValidationError {
                    code: "loads.invalid_field".into(),
                    message: format!("Field '{}' is not valid for Loads.", unknown),
                }),
            };

            result?; // propagate error if any field fails
        }

        Ok(())
    }
}
