use serde::{Deserialize, Serialize};

use crate::validation::{validate_field, ValidationError};

/// Represents a single soil layer in a geotechnical engineering model.
///
/// This struct contains essential soil properties used for analysis, such as
/// shear strength, stiffness, and classification parameters. The parameters are
/// divided into **total stress** (undrained) and **effective stress** (drained)
/// conditions for comprehensive modeling.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SoilLayer {
    pub thickness: Option<f64>,                 // meter
    pub natural_unit_weight: Option<f64>,       // t/m³
    pub dry_unit_weight: Option<f64>,           // t/m³
    pub saturated_unit_weight: Option<f64>,     // t/m³
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

impl SoilLayer {
    pub fn new(thickness: f64) -> Self {
        Self {
            thickness: Some(thickness),
            ..Default::default()
        }
    }
    /// Validate based on a list of required fields by name.
    ///
    /// # Arguments
    /// * `fields` - A slice of field names to validate.
    ///
    /// # Returns
    /// * `Ok(())` if all required fields are valid.
    pub fn validate_fields(&self, fields: &[&str]) -> Result<(), ValidationError> {
        for &field in fields {
            let result = match field {
                "thickness" => validate_field(
                    "thickness",
                    self.thickness,
                    Some(0.0001),
                    None,
                    "soil_profile",
                ),
                "natural_unit_weight" => validate_field(
                    "natural_unit_weight",
                    self.natural_unit_weight,
                    Some(0.1),
                    Some(10.0),
                    "soil_profile",
                ),
                "dry_unit_weight" => validate_field(
                    "dry_unit_weight",
                    self.dry_unit_weight,
                    Some(0.1),
                    Some(10.0),
                    "soil_profile",
                ),
                "saturated_unit_weight" => validate_field(
                    "saturated_unit_weight",
                    self.saturated_unit_weight,
                    Some(0.1),
                    Some(10.0),
                    "soil_profile",
                ),
                "damping_ratio" => validate_field(
                    "damping_ratio",
                    self.damping_ratio,
                    Some(0.1),
                    Some(100.0),
                    "soil_profile",
                ),
                "fine_content" => validate_field(
                    "fine_content",
                    self.fine_content,
                    Some(0.0),
                    Some(100.),
                    "soil_profile",
                ),
                "liquid_limit" => validate_field(
                    "liquid_limit",
                    self.liquid_limit,
                    Some(0.0),
                    Some(100.),
                    "soil_profile",
                ),
                "plastic_limit" => validate_field(
                    "plastic_limit",
                    self.plastic_limit,
                    Some(0.0),
                    Some(100.),
                    "soil_profile",
                ),
                "plasticity_index" => validate_field(
                    "plasticity_index",
                    self.plasticity_index,
                    Some(0.0),
                    Some(100.),
                    "soil_profile",
                ),
                "cu" => validate_field("cu", self.cu, Some(0.0), None, "soil_profile"),
                "c_prime" => {
                    validate_field("c_prime", self.c_prime, Some(0.0), None, "soil_profile")
                }
                "phi_u" => {
                    validate_field("phi_u", self.phi_u, Some(0.0), Some(90.), "soil_profile")
                }
                "phi_prime" => validate_field(
                    "phi_prime",
                    self.phi_prime,
                    Some(0.0),
                    Some(90.),
                    "soil_profile",
                ),
                "water_content" => validate_field(
                    "water_content",
                    self.water_content,
                    Some(0.),
                    Some(100.),
                    "soil_profile",
                ),
                "poissons_ratio" => validate_field(
                    "poissons_ratio",
                    self.poissons_ratio,
                    Some(0.0001),
                    Some(0.5),
                    "soil_profile",
                ),
                "elastic_modulus" => validate_field(
                    "elastic_modulus",
                    self.elastic_modulus,
                    Some(0.0001),
                    None,
                    "soil_profile",
                ),
                "void_ratio" => validate_field(
                    "void_ratio",
                    self.void_ratio,
                    Some(0.0),
                    None,
                    "soil_profile",
                ),
                "compression_index" => validate_field(
                    "compression_index",
                    self.compression_index,
                    Some(0.0),
                    None,
                    "soil_profile",
                ),
                "recompression_index" => validate_field(
                    "recompression_index",
                    self.recompression_index,
                    Some(0.0),
                    None,
                    "soil_profile",
                ),
                "preconsolidation_pressure" => validate_field(
                    "preconsolidation_pressure",
                    self.preconsolidation_pressure,
                    Some(0.0),
                    None,
                    "soil_profile",
                ),
                "mv" => validate_field("mv", self.mv, Some(0.0), None, "soil_profile"),
                "shear_wave_velocity" => validate_field(
                    "shear_wave_velocity",
                    self.shear_wave_velocity,
                    Some(0.0),
                    None,
                    "soil_profile",
                ),
                other => Err(ValidationError {
                    code: "soil_profile.invalid_field".to_string(),
                    message: format!("Field '{}' is not valid for SoilLayer.", other),
                }),
            };

            result?;
        }

        Ok(())
    }
}

/// Represents a soil profile consisting of multiple soil layers.
/// This structure stores soil layers and calculates normal and effective stresses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilProfile {
    /// A list of soil layers in the profile.
    pub layers: Vec<SoilLayer>,
    /// Depth of the groundwater table (meters).
    pub ground_water_level: Option<f64>, // meters
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
            ground_water_level: Some(ground_water_level),
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
            let thickness = layer.thickness.unwrap();
            layer.center = Some(bottom + thickness / 2.0);
            bottom += thickness;
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
        let gwt = self.ground_water_level.unwrap();

        for (i, layer) in self.layers.iter().take(layer_index + 1).enumerate() {
            let layer_thickness = if i == layer_index {
                depth - previous_depth // Partial thickness for last layer
            } else {
                layer.thickness.unwrap() // Full thickness for earlier layers
            };
            let dry_unit_weight = layer.dry_unit_weight.unwrap_or(0.0);
            let saturated_unit_weight = layer.saturated_unit_weight.unwrap_or(0.0);
            if dry_unit_weight <= 1.0 && saturated_unit_weight <= 1.0 {
                panic!("Dry or saturated unit weight must be greater then 1 for each layer.");
            }
            if gwt >= previous_depth + layer_thickness {
                // Entirely above groundwater table (dry unit weight applies)
                total_stress += dry_unit_weight * layer_thickness;
            } else if gwt <= previous_depth {
                // Entirely below groundwater table (saturated unit weight applies)
                total_stress += saturated_unit_weight * layer_thickness;
            } else {
                // Partially submerged (both dry and saturated weights apply)
                let dry_thickness = gwt - previous_depth;
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

        if self.ground_water_level.unwrap() >= depth {
            normal_stress // Effective stress equals total stress above water table
        } else {
            let pore_pressure = (depth - self.ground_water_level.unwrap()) * 0.981; // t/m³ for water
            normal_stress - pore_pressure
        }
    }

    /// Validates the soil profile and its layers.
    ///
    /// # Arguments
    /// * `fields` - A slice of field names to validate.
    ///
    /// # Returns
    /// * `Ok(())` if the profile is valid.
    pub fn validate(&self, fields: &[&str]) -> Result<(), ValidationError> {
        if self.layers.is_empty() {
            return Err(ValidationError {
                code: "soil_profile.empty".to_string(),
                message: "Soil profile must contain at least one layer.".to_string(),
            });
        }

        for layer in &self.layers {
            layer.validate_fields(fields)?;
        }

        validate_field(
            "ground_water_level",
            self.ground_water_level,
            Some(0.0),
            None,
            "soil_profile",
        )?;

        Ok(())
    }
}
