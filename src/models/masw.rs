/// Represents an individual MASW (Multichannel Analysis of Surface Waves) experiment layer.
///
/// # Fields
///
/// * `thickness` - The thickness of the layer in meters.
/// * `vs` - The shear wave velocity of the layer in meters per second.
/// * `depth` - The depth of the layer in meters.
#[derive(Debug, Clone)]
pub struct MaswExp {
    pub thickness: f64,
    pub vs: f64,
    pub depth: f64,
}

impl MaswExp {
    pub fn new(thickness: f64, vs: f64) -> Self {
        Self {
            thickness,
            vs,
            depth: 0.0,
        }
    }
}

impl Default for MaswExp {
    fn default() -> Self {
        Self {
            thickness: 0.0,
            vs: 0.0,
            depth: 0.0,
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
pub struct Masw {
    pub exps: Vec<MaswExp>,
}

impl Masw {
    pub fn new(exps: Vec<MaswExp>) -> Self {
        let mut instance = Self { exps }; // Create a mutable instance
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
        if self.exps.is_empty() {
            return;
        }

        let mut bottom = 0.0;

        for exp in &mut self.exps {
            if exp.thickness <= 0.0 {
                panic!("Thickness of MASW experiment must be greater than zero.");
            }

            exp.depth = bottom + exp.thickness;
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
    pub fn get_exp_at_depth(&self, depth: f64) -> &MaswExp {
        self.exps
            .iter()
            .find(|exp| exp.depth >= depth)
            .unwrap_or_else(|| self.exps.last().unwrap())
    }
}
