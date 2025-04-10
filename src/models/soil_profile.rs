use serde::{Deserialize, Serialize};
/// Represents a single soil layer in a geotechnical engineering model.
///
/// This struct contains essential soil properties used for analysis, such as
/// shear strength, stiffness, and classification parameters. The parameters are
/// divided into **total stress** (undrained) and **effective stress** (drained)
/// conditions for comprehensive modeling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilLayer {
    pub thickness: f64,                         // meter
    pub natural_unit_weight: Option<f64>,       // t/m³
    pub dry_unit_weight: Option<f64>,           // t/m³
    pub saturated_unit_weight: Option<f64>,     // t/m³
    pub soil_class: String,                     // Soil classification
    pub depth: Option<f64>,                     // meter
    pub center: Option<f64>,                    // meter
    pub damping_ratio: Option<f64>,             // percentage
    pub fine_content: Option<f64>,              // percentage
    pub liquid_limit: Option<f64>,              // percentage
    pub plastic_limit: Option<f64>,             // percentage
    pub plasticity_index: Option<f64>,          // percentage
    pub cu: Option<f64>,                        // Undrained shear strength in t/m²
    pub c_prime: Option<f64>,                   // Effective cohesion in t/m²
    pub phi_u: Option<f64>,                     // Undrained internal friction angle in degrees
    pub phi_prime: Option<f64>,                 // Effective internal friction angle in degrees
    pub water_content: Option<f64>,             // percentage
    pub poissons_ratio: Option<f64>,            // Poisson's ratio
    pub elastic_modulus: Option<f64>,           // t/m²
    pub void_ratio: Option<f64>,                // Void ratio
    pub recompression_index: Option<f64>,       // Recompression index
    pub compression_index: Option<f64>,         // Compression index
    pub preconsolidation_pressure: Option<f64>, // t/m²
    pub mv: Option<f64>,                        // volume compressibility coefficient in m²/t
    pub shear_wave_velocity: Option<f64>,       // m/s
}
impl Default for SoilLayer {
    fn default() -> Self {
        Self {
            thickness: 1.0,
            natural_unit_weight: None,
            dry_unit_weight: None,
            saturated_unit_weight: None,
            soil_class: String::new(),
            depth: None,
            center: None,
            damping_ratio: None,
            fine_content: None,
            liquid_limit: None,
            plastic_limit: None,
            plasticity_index: None,
            cu: None,
            c_prime: None,
            phi_u: None,
            phi_prime: None,
            water_content: None,
            poissons_ratio: None,
            elastic_modulus: None,
            void_ratio: None,
            recompression_index: None,
            compression_index: None,
            preconsolidation_pressure: None,
            mv: None,
            shear_wave_velocity: None,
        }
    }
}
impl SoilLayer {
    pub fn new(thickness: f64) -> Self {
        Self {
            thickness,
            ..Default::default()
        }
    }
}

/// Represents a soil profile consisting of multiple soil layers.
/// This structure stores soil layers and calculates normal and effective stresses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilProfile {
    /// A list of soil layers in the profile.
    pub layers: Vec<SoilLayer>,
    /// Depth of the groundwater table (meters).
    pub ground_water_level: f64,
}

impl SoilProfile {
    /// Creates a new soil profile and initializes layer depths.
    ///
    /// # Arguments
    /// * `layers` - A vector of `SoilLayer` objects.
    /// * `ground_water_level` - Depth of the groundwater table in meters.
    ///
    /// # Panics
    /// * If no layers are provided.
    pub fn new(layers: Vec<SoilLayer>, ground_water_level: f64) -> Self {
        if layers.is_empty() {
            panic!("Soil profile must contain at least one layer.");
        }

        let mut profile = Self {
            layers,
            ground_water_level,
        };
        profile.calc_layer_depths();
        profile
    }

    /// Calculates center and bottom depth for each soil layer.
    pub fn calc_layer_depths(&mut self) {
        if self.layers.is_empty() {
            return;
        }

        let mut bottom = 0.0;

        for layer in &mut self.layers {
            if layer.thickness <= 0.0 {
                panic!("Thickness of soil layer must be greater than zero.");
            }

            layer.center = Some(bottom + layer.thickness / 2.0);
            bottom += layer.thickness;
            layer.depth = Some(bottom);
        }
    }

    /// Returns the index of the soil layer at a specified depth.
    ///
    /// # Arguments
    /// * `depth` - The depth at which to find the layer.
    ///
    /// # Returns
    /// * The index of the layer containing the specified depth.
    pub fn get_layer_index(&self, depth: f64) -> usize {
        for (i, layer) in self.layers.iter().enumerate() {
            if let Some(layer_depth) = layer.depth {
                if layer_depth >= depth {
                    return i;
                }
            }
        }
        self.layers.len() - 1
    }

    /// Returns a reference to the soil layer at a specified depth.
    ///
    /// # Arguments
    /// * `depth` - The depth at which to find the layer.
    ///
    /// # Returns
    /// * A reference to the `SoilLayer` at the specified depth.
    pub fn get_layer_at_depth(&self, depth: f64) -> &SoilLayer {
        let index = self.get_layer_index(depth);
        &self.layers[index]
    }

    /// Calculates the total (normal) stress at a given depth.
    ///
    /// # Arguments
    /// * `depth` - The depth at which to calculate total stress.
    ///
    /// # Returns
    /// * The total normal stress (t/m²) at the specified depth.
    pub fn calc_normal_stress(&self, depth: f64) -> f64 {
        let layer_index = self.get_layer_index(depth);

        let mut total_stress = 0.0;
        let mut previous_depth = 0.0;

        for (i, layer) in self.layers.iter().take(layer_index + 1).enumerate() {
            let layer_thickness = if i == layer_index {
                depth - previous_depth // Partial thickness for last layer
            } else {
                layer.thickness // Full thickness for earlier layers
            };
            let dry_unit_weight = layer.dry_unit_weight.unwrap_or(0.0);
            let saturated_unit_weight = layer.saturated_unit_weight.unwrap_or(0.0);
            if dry_unit_weight <= 1.0 && saturated_unit_weight <= 1.0 {
                panic!("Dry or saturated unit weight must be greater then 1 for each layer.");
            }
            if self.ground_water_level >= previous_depth + layer_thickness {
                // Entirely above groundwater table (dry unit weight applies)
                total_stress += dry_unit_weight * layer_thickness;
            } else if self.ground_water_level <= previous_depth {
                // Entirely below groundwater table (saturated unit weight applies)
                total_stress += saturated_unit_weight * layer_thickness;
            } else {
                // Partially submerged (both dry and saturated weights apply)
                let dry_thickness = self.ground_water_level - previous_depth;
                let submerged_thickness = layer_thickness - dry_thickness;
                total_stress +=
                    dry_unit_weight * dry_thickness + saturated_unit_weight * submerged_thickness;
            }

            previous_depth += layer_thickness;
        }

        total_stress
    }

    /// Calculates the effective stress at a given depth.
    ///
    /// # Arguments
    /// * `depth` - The depth at which to calculate effective stress.
    ///
    /// # Returns
    /// * The effective stress (t/m²) at the specified depth.
    pub fn calc_effective_stress(&self, depth: f64) -> f64 {
        let normal_stress = self.calc_normal_stress(depth);

        if self.ground_water_level >= depth {
            normal_stress // Effective stress equals total stress above water table
        } else {
            let pore_pressure = (depth - self.ground_water_level) * 0.981; // t/m³ for water
            normal_stress - pore_pressure
        }
    }
}
