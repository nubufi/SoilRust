use crate::{
    enums::SelectionMethod,
    validation::{validate_field, ValidationError},
};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Represents an individual MASW (Multichannel Analysis of Surface Waves) experiment layer.
///
/// # Fields
///
/// * `thickness` - The thickness of the layer in meters.
/// * `vs` - The shear wave velocity of the layer in meters per second.
/// * `vp` - The compressional wave velocity of the layer in meters per second.
/// * `depth` - The depth of the layer in meters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaswLayer {
    pub thickness: Option<f64>,
    pub vs: Option<f64>,
    pub vp: Option<f64>,
    pub depth: Option<f64>,
}

impl MaswLayer {
    pub fn new(thickness: f64, vs: f64, vp: f64) -> Self {
        Self {
            thickness: Some(thickness),
            vs: Some(vs),
            vp: Some(vp),
            depth: None,
        }
    }
    /// Validates specific fields of the MaswLayer using field names.
    ///
    /// # Arguments
    /// * `fields` - A slice of field names to validate.
    ///
    /// # Returns
    /// Ok(()) if all fields are valid, or an error if any field is invalid.
    pub fn validate(&self, fields: &[&str]) -> Result<(), ValidationError> {
        for &field in fields {
            let result = match field {
                "depth" => validate_field("depth", self.depth, Some(0.0), None, "masw"),
                "thickness" => {
                    validate_field("thickness", self.thickness, Some(0.0001), None, "masw")
                }
                "vs" => validate_field("vs", self.vs, Some(0.0), None, "masw"),
                "vp" => validate_field("vp", self.vp, Some(0.0), None, "masw"),
                unknown => Err(ValidationError {
                    code: "masw.invalid_field".into(),
                    message: format!("Field '{}' is not valid for Loads.", unknown),
                }),
            };

            result?; // propagate error if any field fails
        }

        Ok(())
    }
}

/// Represents a MASW (Multichannel Analysis of Surface Waves) experiment.
///
/// # Fields
/// * `exps` - A vector of `MaswExp` instances representing the individual layers of the experiment.
/// * `depths` - A vector of the depths of the layers in the experiment.
/// * `vs` - A vector of the shear wave velocities of the layers in the experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaswExp {
    pub layers: Vec<MaswLayer>,
    pub name: String,
}

impl MaswExp {
    pub fn new(layers: Vec<MaswLayer>, name: String) -> Self {
        let mut instance = Self { layers, name }; // Create a mutable instance
        instance.calc_depths(); // Call calc_depths to update depths
        instance // Return the modified instance
    }

    /// Calculates and updates the depth of each MASW experiment layer.
    ///
    /// Depth is calculated as a cumulative sum of layer thicknesses.
    /// - The first layer's depth is equal to its thickness.
    /// - Each subsequent layer's depth is the sum of all previous layers' thicknesses.
    ///
    /// # Panics
    ///
    /// This function panics if any layer has a `thickness` value of `0.0` or less.
    pub fn calc_depths(&mut self) {
        if self.layers.is_empty() {
            return;
        }

        let mut bottom = 0.0;

        for exp in &mut self.layers {
            let thickness = exp.thickness.unwrap();
            if thickness <= 0.0 {
                panic!("Thickness of MASW experiment must be greater than zero.");
            }

            exp.depth = Some(bottom + thickness);
            bottom += thickness;
        }
    }

    /// Retrieves the MASW experiment layer corresponding to a given depth.
    ///
    /// This function finds the first layer whose depth is greater than or equal to the given `depth`.
    /// If no such layer is found, it returns the last layer in the list.
    ///
    /// # Arguments
    ///
    /// * `depth` - The depth at which to search for an experiment layer.
    ///
    /// # Returns
    ///
    /// A reference to the matching `MaswExp` layer.
    pub fn get_layer_at_depth(&self, depth: f64) -> &MaswLayer {
        self.layers
            .iter()
            .find(|exp| exp.depth.unwrap() >= depth)
            .unwrap_or_else(|| self.layers.last().unwrap())
    }

    /// Validates specific fields of the MaswExp using field names.
    ///
    /// # Arguments
    /// * `fields` - A slice of field names to validate.
    ///
    /// # Returns
    /// Ok(()) if all fields are valid, or an error if any field is invalid.
    pub fn validate(&self, fields: &[&str]) -> Result<(), ValidationError> {
        if self.layers.is_empty() {
            return Err(ValidationError {
                code: "masw.empty_layers".into(),
                message: "No layers provided for MaswExp.".into(),
            });
        }
        for layer in &self.layers {
            layer.validate(fields)?;
        }
        Ok(())
    }
}

/// Represents a MASW (Multichannel Analysis of Surface Waves) model.
///
/// # Fields
/// * `exps` - A vector of `MaswExp` instances representing the individual experiments in the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Masw {
    pub exps: Vec<MaswExp>,
    pub idealization_method: SelectionMethod,
}

impl Masw {
    /// Creates a new `Masw` instance.
    ///
    /// # Arguments
    /// * `exps` - A vector of `MaswExp` instances.
    /// * `idealization_method` - The method used for idealization.
    ///
    /// # Returns
    /// A new `Masw` instance.
    pub fn new(mut exps: Vec<MaswExp>, idealization_method: SelectionMethod) -> Self {
        for exp in &mut exps {
            exp.calc_depths();
        }
        Self {
            exps,
            idealization_method,
        }
    }

    /// Adds a new `MaswExp` instance to the `Masw` collection.
    ///
    /// # Arguments
    /// * `exp` - The `MaswExp` instance to add to the collection.
    pub fn add_exp(&mut self, exp: MaswExp) {
        self.exps.push(exp);
    }

    /// Calculates and updates the depth of each MASW experiment layer in the model.
    pub fn calc_depths(&mut self) {
        for exp in &mut self.exps {
            exp.calc_depths();
        }
    }

    /// Creates an idealized MASW experiment based on the given mode.
    /// The idealized experiment is created by combining the corresponding layers from each individual experiment in the model.
    ///
    /// # Arguments
    /// * `name` - The name of the idealized experiment.
    ///
    /// # Returns
    /// A new `MaswExp` instance representing the idealized experiment.
    pub fn get_idealized_exp(&mut self, name: String) -> MaswExp {
        if self.exps.is_empty() {
            return MaswExp::new(vec![], name);
        }

        let mode = self.idealization_method;

        self.calc_depths();

        // 1. Collect unique depths across all experiments
        let mut unique_depths = BTreeSet::new();
        unique_depths.insert(OrderedFloat(0.0)); // Add the surface depth
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
        for depth_pair in sorted_depths.windows(2) {
            let top = depth_pair[0];
            let bottom = depth_pair[1];
            let thickness = bottom - top;

            let mut vs_at_depth = Vec::new();
            let mut vp_at_depth = Vec::new();

            for exp in &self.exps {
                let layer = exp.get_layer_at_depth((top + bottom) / 2.0);
                vs_at_depth.push(layer.vs.unwrap());
                vp_at_depth.push(layer.vp.unwrap());
            }

            let vs = get_mode_value(mode, vs_at_depth);
            let vp = get_mode_value(mode, vp_at_depth);

            layers.push(MaswLayer::new(thickness, vs, vp));
        }

        MaswExp::new(layers, name)
    }
    /// Validates specific fields of the Masw using field names.
    ///
    /// # Arguments
    /// * `fields` - A slice of field names to validate.
    ///
    /// # Returns
    /// Ok(()) if all fields are valid, or an error if any field is invalid.
    pub fn validate(&self, fields: &[&str]) -> Result<(), ValidationError> {
        if self.exps.is_empty() {
            return Err(ValidationError {
                code: "masw.empty_exps".into(),
                message: "No experiments provided for Masw.".into(),
            });
        }
        for exp in &self.exps {
            exp.validate(fields)?;
        }
        Ok(())
    }
}
