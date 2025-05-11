use crate::{
    enums::SelectionMethod,
    validation::{validate_field, ValidationError},
};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Represents a single CPT (Cone Penetration Test) data point.
///
/// Each `CPTLayer` instance holds a `depth` value (in meters) and a `cone_resistance` value (in MPa).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPTLayer {
    pub depth: Option<f64>,           // Depth in meters
    pub cone_resistance: Option<f64>, // Cone resistance (qc) in MPa
    pub sleeve_friction: Option<f64>, // Sleeve friction (fs) in MPa
    pub pore_pressure: Option<f64>,   // Pore pressure (u2) in MPa
    pub friction_ratio: Option<f64>,  // Friction ratio (Rf) in percentage
}

impl Default for CPTLayer {
    fn default() -> Self {
        Self {
            depth: Some(0.0),
            cone_resistance: Some(0.0),
            sleeve_friction: Some(0.0),
            pore_pressure: None,
            friction_ratio: None,
        }
    }
}
impl CPTLayer {
    /// Creates a new `CPTLayer` instance.
    ///
    /// # Arguments
    /// * `depth` - The depth of the CPT data point in meters.
    /// * `cone_resistance` - The cone resistance of the CPT data point in MPa.
    pub fn new(depth: f64, qc: f64, fs: f64, u2: Option<f64>) -> Self {
        Self {
            depth: Some(depth),
            cone_resistance: Some(qc),
            sleeve_friction: Some(fs),
            pore_pressure: u2,
            friction_ratio: None,
        }
    }

    /// Calculates the friction ratio (Rf) for the CPT data point.
    /// The friction ratio is calculated as the ratio of sleeve friction to cone resistance.
    /// If the sleeve friction is not available, the function returns `None`.
    /// If the cone resistance is zero, the function returns `None`.
    /// If the friction ratio is calculated, it is stored in the `friction_ratio` field of the `CPTLayer` instance.
    /// The friction ratio is expressed as a percentage.
    /// The formula for calculating the friction ratio is:
    /// ```text
    /// Rf = (fs / qc) * 100
    /// ```
    /// where:
    /// - `Rf` is the friction ratio in percentage.
    /// - `fs` is the sleeve friction in MPa.
    /// - `qc` is the cone resistance in MPa.
    pub fn calc_friction_ratio(&mut self) {
        if self.cone_resistance.unwrap() != 0.0 {
            self.friction_ratio =
                Some((self.sleeve_friction.unwrap() / self.cone_resistance.unwrap()) * 100.0);
        }
    }

    /// Validates specific fields of the CPTLayer using field names.
    ///
    /// # Arguments
    /// * `fields` - A slice of field names to validate.
    ///
    /// # Returns
    /// Ok(()) if all fields are valid, or an error if any field is invalid.
    pub fn validate(&self, fields: &[&str]) -> Result<(), ValidationError> {
        for &field in fields {
            let result = match field {
                "depth" => validate_field("depth", self.depth, Some(0.0), None, "cpt"),
                "cone_resistance" => validate_field(
                    "cone_resistance",
                    self.cone_resistance,
                    Some(0.0),
                    None,
                    "cpt",
                ),
                "sleeve_friction" => validate_field(
                    "sleeve_friction",
                    self.sleeve_friction,
                    Some(0.0),
                    None,
                    "cpt",
                ),
                "pore_pressure" => {
                    validate_field("pore_pressure", self.pore_pressure, Some(0.0), None, "cpt")
                }
                "friction_ratio" => validate_field(
                    "friction_ratio",
                    self.friction_ratio,
                    Some(0.0),
                    None,
                    "cpt",
                ),
                unknown => Err(ValidationError {
                    code: "cpt.invalid_field".into(),
                    message: format!("Field '{}' is not valid for CPT.", unknown),
                }),
            };

            result?; // propagate error if any field fails
        }

        Ok(())
    }
}
// ------------------------------------------------------------------------------------------------

/// Represents a collection of CPT data points.
///
/// A `CPTExp` struct contains multiple `CPTLayer` instances, forming a complete CPT profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPTExp {
    pub layers: Vec<CPTLayer>,
    pub name: String,
}

impl CPTExp {
    /// Creates a new `CPT` instance.
    ///
    /// # Arguments
    /// * `layers` - A vector of `CPTLayer` instances.
    /// * `name` - The name of the CPT profile.
    pub fn new(layers: Vec<CPTLayer>, name: String) -> Self {
        Self { layers, name }
    }

    /// Adds a new `CPTLayer` instance to the `CPTExp` collection.
    ///
    /// # Arguments
    /// * `layer` - The `CPTLayer` instance to add to the collection.
    pub fn add_layer(&mut self, layer: CPTLayer) {
        self.layers.push(layer);
    }

    /// Retrieves the CPT layer corresponding to a given depth.
    ///
    /// This function finds the first layer whose depth is greater than or equal to the given `depth`.
    /// If no such layer is found, it returns the last layer in the list.
    ///
    /// # Arguments
    /// * `depth` - The depth at which to search for a CPT layer.
    ///
    /// # Returns
    /// A reference to the matching `CPTLayer`.
    pub fn get_layer_at_depth(&self, depth: f64) -> &CPTLayer {
        self.layers
            .iter()
            .find(|exp| exp.depth.unwrap() >= depth)
            .unwrap_or_else(|| self.layers.last().unwrap())
    }

    /// Validates specific fields of the CPTExp using field names.
    ///
    /// # Arguments
    /// * `fields` - A slice of field names to validate.
    ///
    /// # Returns
    /// Ok(()) if all fields are valid, or an error if any field is invalid.
    pub fn validate(&self, fields: &[&str]) -> Result<(), ValidationError> {
        if self.layers.is_empty() {
            return Err(ValidationError {
                code: "cpt.empty_layers".into(),
                message: "No layers provided for CPTExp.".into(),
            });
        }
        for layer in &self.layers {
            layer.validate(fields)?;
        }

        Ok(())
    }
}
// ------------------------------------------------------------------------------------------------

/// Represents a collection of CPT tests.
///
/// A `CPT` struct contains multiple `CPTExp` instances, each representing a single CPT profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPT {
    pub exps: Vec<CPTExp>,
    pub idealization_method: SelectionMethod,
}

impl CPT {
    /// Creates a new `CPT` instance.
    ///
    /// # Arguments
    /// * `exps` - A vector of `CPTExp` instances.
    /// * `idealization_method` - The method used for idealization.
    pub fn new(exps: Vec<CPTExp>, idealization_method: SelectionMethod) -> Self {
        Self {
            exps,
            idealization_method,
        }
    }

    /// Adds a new `CPTExp` instance to the `CPT` collection.
    ///
    /// # Arguments
    /// * `exp` - The `CPTExp` instance to add to the collection.
    pub fn add_exp(&mut self, exp: CPTExp) {
        self.exps.push(exp);
    }

    /// Creates an idealized CPT experiment based on the given mode.
    /// The idealized experiment is created by combining the corresponding layers from each individual experiment in the model.
    ///
    /// # Arguments
    /// * `name` - The name of the idealized experiment.
    ///
    /// # Returns
    /// A new `CPTExp` instance representing the idealized experiment.
    pub fn get_idealized_exp(&self, name: String) -> CPTExp {
        if self.exps.is_empty() {
            return CPTExp::new(vec![], name);
        }

        let mode = self.idealization_method;

        // 1. Collect unique depths across all experiments
        let mut unique_depths = BTreeSet::new();
        for exp in &self.exps {
            for layer in &exp.layers {
                unique_depths.insert(OrderedFloat(layer.depth.unwrap()));
            }
        }

        let sorted_depths: Vec<f64> = unique_depths.into_iter().map(|d| d.into_inner()).collect();

        let mut layers = Vec::new();

        let get_mode_value = |mode: SelectionMethod, values: Vec<f64>| -> f64 {
            match mode {
                SelectionMethod::Min => values.iter().cloned().fold(f64::INFINITY, f64::min),
                SelectionMethod::Avg => values.iter().sum::<f64>() / values.len() as f64,
                SelectionMethod::Max => values.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            }
        };
        for depth in sorted_depths {
            let mut qc_at_depth = Vec::new();
            let mut fs_at_depth = Vec::new();
            let mut u2_at_depth = Vec::new();

            for exp in &self.exps {
                let layer = exp.get_layer_at_depth(depth);
                qc_at_depth.push(layer.cone_resistance.unwrap());
                fs_at_depth.push(layer.sleeve_friction.unwrap());
                u2_at_depth.push(layer.pore_pressure.unwrap_or(0.0));
            }

            let qc = get_mode_value(mode, qc_at_depth);
            let fs = get_mode_value(mode, fs_at_depth);
            let u2 = get_mode_value(mode, u2_at_depth);

            layers.push(CPTLayer::new(depth, qc, fs, Some(u2)));
        }

        CPTExp::new(layers, name)
    }

    /// Validates specific fields of the CPT using field names.
    ///
    /// # Arguments
    /// * `fields` - A slice of field names to validate.
    ///
    /// # Returns
    /// Ok(()) if all fields are valid, or an error if any field is invalid.
    pub fn validate(&self, fields: &[&str]) -> Result<(), ValidationError> {
        if self.exps.is_empty() {
            return Err(ValidationError {
                code: "cpt.empty_exps".into(),
                message: "No experiments found in CPT.".into(),
            });
        }
        for exp in &self.exps {
            exp.validate(fields)?;
        }

        Ok(())
    }
}
