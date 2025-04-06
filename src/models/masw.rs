use crate::enums::SelectionMethod;
use ordered_float::OrderedFloat;
use std::collections::BTreeSet;

/// Represents an individual MASW (Multichannel Analysis of Surface Waves) experiment layer.
///
/// # Fields
///
/// * `thickness` - The thickness of the layer in meters.
/// * `vs` - The shear wave velocity of the layer in meters per second.
/// * `vp` - The compressional wave velocity of the layer in meters per second.
/// * `depth` - The depth of the layer in meters.
#[derive(Debug, Clone)]
pub struct MaswLayer {
    pub thickness: f64,
    pub vs: f64,
    pub vp: f64,
    pub depth: Option<f64>,
}

impl MaswLayer {
    pub fn new(thickness: f64, vs: f64, vp: f64) -> Self {
        Self {
            thickness,
            vs,
            vp,
            depth: None,
        }
    }
}

/// Represents a MASW (Multichannel Analysis of Surface Waves) experiment.
///
/// # Fields
/// * `exps` - A vector of `MaswExp` instances representing the individual layers of the experiment.
/// * `depths` - A vector of the depths of the layers in the experiment.
/// * `vs` - A vector of the shear wave velocities of the layers in the experiment.
#[derive(Debug, Clone)]
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
            if exp.thickness <= 0.0 {
                panic!("Thickness of MASW experiment must be greater than zero.");
            }

            exp.depth = Some(bottom + exp.thickness);
            bottom += exp.thickness;
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
}

/// Represents a MASW (Multichannel Analysis of Surface Waves) model.
///
/// # Fields
/// * `exps` - A vector of `MaswExp` instances representing the individual experiments in the model.
#[derive(Debug, Clone)]
pub struct Masw {
    pub exps: Vec<MaswExp>,
}

impl Masw {
    pub fn new(mut exps: Vec<MaswExp>) -> Self {
        for exp in &mut exps {
            exp.calc_depths();
        }
        Self { exps }
    }

    pub fn add_exp(&mut self, exp: MaswExp) {
        self.exps.push(exp);
    }

    /// Creates an idealized MASW experiment based on the given mode.
    /// The idealized experiment is created by combining the corresponding layers from each individual experiment in the model.
    ///
    /// # Arguments
    /// * `mode` - The idealized mode to use when combining the layers.
    /// * `name` - The name of the idealized experiment.
    ///
    /// # Returns
    /// A new `MaswExp` instance representing the idealized experiment.
    pub fn get_idealized_exp(&self, mode: SelectionMethod, name: String) -> MaswExp {
        if self.exps.is_empty() {
            return MaswExp::new(vec![], name);
        }

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
                vs_at_depth.push(layer.vs);
                vp_at_depth.push(layer.vp);
            }

            let vs = get_mode_value(mode, vs_at_depth);
            let vp = get_mode_value(mode, vp_at_depth);

            layers.push(MaswLayer::new(thickness, vs, vp));
        }

        MaswExp::new(layers, name)
    }
}
