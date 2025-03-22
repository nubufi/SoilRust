use crate::enums::SelectionMethod;
use ordered_float::OrderedFloat;
use std::collections::BTreeSet;

/// Represents a single CPT (Cone Penetration Test) data point.
///
/// Each `CPTLayer` instance holds a `depth` value (in meters) and a `cone_resistance` value (in MPa).
#[derive(Debug, Clone)]
pub struct CPTLayer {
    pub depth: f64,                  // Depth in meters
    pub cone_resistance: f64,        // Cone resistance (qc) in MPa
    pub sleeve_friction: f64,        // Sleeve friction (fs) in MPa
    pub pore_pressure: Option<f64>,  // Pore pressure (u2) in MPa
    pub friction_ratio: Option<f64>, // Friction ratio (Rf) in percentage
}

impl Default for CPTLayer {
    fn default() -> Self {
        Self {
            depth: 0.0,
            cone_resistance: 0.0,
            sleeve_friction: 0.0,
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
            depth,
            cone_resistance: qc,
            sleeve_friction: fs,
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
        if self.cone_resistance != 0.0 {
            self.friction_ratio = Some((self.sleeve_friction / self.cone_resistance) * 100.0);
        }
    }
}
// ------------------------------------------------------------------------------------------------

/// Represents a collection of CPT data points.
///
/// A `CPTExp` struct contains multiple `CPTLayer` instances, forming a complete CPT profile.
#[derive(Debug, Clone)]
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
            .find(|exp| exp.depth >= depth)
            .unwrap_or_else(|| self.layers.last().unwrap())
    }
}
// ------------------------------------------------------------------------------------------------

/// Represents a collection of CPT tests.
///
/// A `CPT` struct contains multiple `CPTExp` instances, each representing a single CPT profile.
#[derive(Debug, Clone)]
pub struct CPT {
    pub exps: Vec<CPTExp>,
}

impl CPT {
    /// Creates a new `CPT` instance.
    ///
    /// # Arguments
    /// * `exps` - A vector of `CPTExp` instances.
    pub fn new(exps: Vec<CPTExp>) -> Self {
        Self { exps }
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
    /// * `mode` - The idealized mode to use when combining the layers.
    /// * `name` - The name of the idealized experiment.
    ///
    /// # Returns
    /// A new `CPTExp` instance representing the idealized experiment.
    pub fn get_idealized_exp(&self, mode: SelectionMethod, name: String) -> CPTExp {
        if self.exps.is_empty() {
            return CPTExp::new(vec![], name);
        }

        // 1. Collect unique depths across all experiments
        let mut unique_depths = BTreeSet::new();
        for exp in &self.exps {
            for layer in &exp.layers {
                unique_depths.insert(OrderedFloat(layer.depth));
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
                qc_at_depth.push(layer.cone_resistance);
                fs_at_depth.push(layer.sleeve_friction);
                u2_at_depth.push(layer.pore_pressure.unwrap_or(0.0));
            }

            let qc = get_mode_value(mode, qc_at_depth);
            let fs = get_mode_value(mode, fs_at_depth);
            let u2 = get_mode_value(mode, u2_at_depth);

            layers.push(CPTLayer::new(depth, qc, fs, Some(u2)));
        }

        CPTExp::new(layers, name)
    }
}
