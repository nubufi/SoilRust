use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::enums::SelectionMethod;

/// Represents an individual Point Load Test sample for determining rock strength.
///
/// # Fields
/// * `depth` - Depth of the sample in meters.
/// * `sample_no` - Optional identifier number for the tested sample.
/// * `p` - Optional applied load at failure in kiloNewtons (kN).
/// * `is` - Optional point load strength index in MegaPascals (MPa).
/// * `f` - Optional size correction factor.
/// * `is50` - Corrected point load strength index to 50 mm diameter in MegaPascals (MPa).
/// * `l` - Optional distance between load application points in millimeters (mm).
/// * `d` - Equivalent core diameter in millimeters (mm).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointLoadSample {
    pub depth: f64,
    pub sample_no: Option<u32>,
    pub p: Option<f64>,
    pub is: Option<f64>,
    pub f: Option<f64>,
    pub is50: f64,
    pub l: Option<f64>,
    pub d: f64,
}

impl PointLoadSample {
    pub fn new(depth: f64, is50: f64, d: f64) -> Self {
        Self {
            depth,
            sample_no: None,
            p: None,
            is: None,
            f: None,
            is50,
            l: None,
            d,
        }
    }
}

/// Represents a single borehole containing multiple Point Load Test samples.
///
/// # Fields
/// * `borehole_id` - Identifier for the borehole.
/// * `samples` - Collection of Point Load Test samples taken from the borehole.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointLoadExp {
    pub borehole_id: String,
    pub samples: Vec<PointLoadSample>,
}

impl PointLoadExp {
    pub fn new(borehole_id: String, samples: Vec<PointLoadSample>) -> Self {
        Self {
            borehole_id,
            samples,
        }
    }

    pub fn add_sample(&mut self, sample: PointLoadSample) {
        self.samples.push(sample);
    }

    /// Retrieves the sample at the specified depth.
    ///
    /// This function finds the first sample whose depth is greater than or equal to the given `depth`.
    /// If no such sample is found, it returns the last sample in the list.
    ///
    /// # Arguments
    ///
    /// * `depth` - The depth at which to search for an experiment sample.
    ///
    /// # Returns
    ///
    /// A reference to the matching `PointLoadSample`.
    pub fn get_sample_at_depth(&self, depth: f64) -> &PointLoadSample {
        self.samples
            .iter()
            .find(|exp| exp.depth >= depth)
            .unwrap_or_else(|| self.samples.last().unwrap())
    }
}

/// Represents the entire Point Load Test comprising multiple boreholes.
///
/// # Fields
/// * `exps` - Collection of borehole tests included in the overall test campaign.
/// * `idealization_method` - Method used for idealizing the test results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointLoadTest {
    pub exps: Vec<PointLoadExp>,
    pub idealization_method: SelectionMethod,
}

impl PointLoadTest {
    pub fn new(exps: Vec<PointLoadExp>, idealization_method: SelectionMethod) -> Self {
        Self {
            exps,
            idealization_method,
        }
    }

    pub fn add_borehole(&mut self, exp: PointLoadExp) {
        self.exps.push(exp);
    }

    /// Get the idealized experiment
    ///
    /// # Arguments
    /// * `mode` - Idealized mode to use when combining the layers
    /// * `name` - Name of the idealized experiment
    ///
    /// # Returns
    /// * `PointLoadExp` - Idealized experiment
    pub fn get_idealized_exp(&self, name: String) -> PointLoadExp {
        if self.exps.is_empty() {
            return PointLoadExp::new(name, vec![]);
        }

        let mode = self.idealization_method;

        let mut depth_map: BTreeMap<
            OrderedFloat<f64>,
            Vec<(OrderedFloat<f64>, OrderedFloat<f64>)>,
        > = BTreeMap::new();

        // Collect all unique depths and corresponding (is50, d) values
        for exp in &self.exps {
            for sample in &exp.samples {
                depth_map
                    .entry(OrderedFloat(sample.depth))
                    .or_default()
                    .push((OrderedFloat(sample.is50), OrderedFloat(sample.d)));
            }
        }

        // Create a new PointLoadExp with selected values
        let mut idealized_samples = Vec::new();

        for (&depth, is50_d_pairs) in &depth_map {
            let selected_is50 = match mode {
                SelectionMethod::Min => is50_d_pairs.iter().min_by_key(|&(is50, _)| is50).unwrap(),
                SelectionMethod::Max => is50_d_pairs.iter().max_by_key(|&(is50, _)| is50).unwrap(),
                SelectionMethod::Avg => {
                    let sum_is50: f64 =
                        is50_d_pairs.iter().map(|(is50, _)| is50.into_inner()).sum();
                    let sum_d: f64 = is50_d_pairs.iter().map(|(_, d)| d.into_inner()).sum();
                    let count = is50_d_pairs.len() as f64;
                    &(OrderedFloat(sum_is50 / count), OrderedFloat(sum_d / count))
                }
            };

            // Add to new PointLoadExp
            idealized_samples.push(PointLoadSample::new(
                depth.into_inner(),
                selected_is50.0.into_inner(),
                selected_is50.1.into_inner(),
            ));
        }

        PointLoadExp::new(name, idealized_samples)
    }
}
