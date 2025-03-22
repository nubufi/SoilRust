use crate::enums::SelectionMethod;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NValue {
    Value(i32),
    Refusal,
}
impl Default for NValue {
    fn default() -> Self {
        NValue::Value(0)
    }
}
impl NValue {
    /// Converts from `i32` to `NValue`
    pub fn from_i32(n: i32) -> Self {
        if n <= 0 {
            panic!("n value must be greater than 0")
        } else {
            NValue::Value(n)
        }
    }

    /// Converts to `Option<i32>` (50 for refusals)
    pub fn to_i32(self) -> i32 {
        match self {
            NValue::Value(n) => n,
            NValue::Refusal => 50,
        }
    }
    /// Converts to `Option<i32>`, treating Refusal as 50
    pub fn to_option(self) -> Option<i32> {
        match self {
            NValue::Value(n) => Some(n),
            NValue::Refusal => Some(50),
        }
    }

    /// Multiply by a factor
    pub fn mul_by_f64(self, factor: f64) -> Self {
        match self {
            NValue::Value(n) => NValue::Value((n as f64 * factor) as i32),
            NValue::Refusal => NValue::Refusal,
        }
    }

    /// Sum up with another NValue
    pub fn sum_with(self, other: Self) -> Self {
        match (self, other) {
            (NValue::Value(n1), NValue::Value(n2)) => NValue::Value(n1 + n2),
            _ => NValue::Refusal,
        }
    }

    /// Sum up with a f64
    pub fn add_f64(self, other: f64) -> Self {
        match self {
            NValue::Value(n) => NValue::Value((n as f64 + other) as i32),
            NValue::Refusal => NValue::Refusal,
        }
    }
}
// Implement `Display` for printing values
impl fmt::Display for NValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NValue::Value(n) => write!(f, "{}", n),
            NValue::Refusal => write!(f, "R"),
        }
    }
}
// Implement ordering so that Refusal is the BEST case (highest value)
impl PartialOrd for NValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for NValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (NValue::Refusal, NValue::Refusal) => std::cmp::Ordering::Equal,
            (NValue::Refusal, _) => std::cmp::Ordering::Greater,
            (_, NValue::Refusal) => std::cmp::Ordering::Less,
            (NValue::Value(a), NValue::Value(b)) => {
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            }
        }
    }
}
// -------------------------------------------------------------------------------------------
#[derive(Debug, Clone, Default)]
pub struct SPTBlow {
    pub depth: f64,
    pub n1: NValue,
    pub n2: NValue,
    pub n3: NValue,
    pub n: NValue,
    pub n60: Option<NValue>,
    pub n90: Option<NValue>,
    pub n1_60: Option<NValue>,
    pub n1_60f: Option<NValue>,
    pub cn: Option<f64>,
    pub alpha: Option<f64>,
    pub beta: Option<f64>,
}

impl SPTBlow {
    /// Create a new SPTBlow
    ///
    /// # Arguments
    /// * `depth` - Depth of the blow
    /// * `n1` - N-value of the first blow
    /// * `n2` - N-value of the second blow
    /// * `n3` - N-value of the third blow
    pub fn new(depth: f64, n1: NValue, n2: NValue, n3: NValue) -> Self {
        Self {
            depth,
            n1,
            n2,
            n3,
            n: n2.sum_with(n3),
            ..Default::default()
        }
    }

    /// Apply energy correction
    ///
    /// # Arguments
    /// * `energy_correction_factor` - Energy correction factor to convert N value to N60
    pub fn apply_energy_correction(&mut self, energy_correction_factor: f64) {
        let n60 = self.n.mul_by_f64(energy_correction_factor);
        self.n60 = Some(n60);
        self.n90 = Some(n60.mul_by_f64(1.5));
    }

    /// Set overburden correction factor
    ///
    /// # Arguments
    /// * `sigma_effective` - Effective overburden pressure in ton
    pub fn set_cn(&mut self, sigma_effective: f64) {
        self.cn = Some(f64::min(f64::sqrt(9.81 / sigma_effective) * 9.78, 1.7))
    }

    /// Set alpha and beta factors
    ///
    /// # Arguments
    /// * `fine_content` - Percentage of fine content in soil in percentage
    pub fn set_alpha_beta(&mut self, fine_content: f64) {
        if fine_content <= 5. {
            self.alpha = Some(0.);
            self.beta = Some(1.);
        } else if fine_content <= 35. {
            self.alpha = Some(f64::exp(1.76 - (190. / fine_content.powi(2))));
            self.beta = Some(0.99 + fine_content.powf(1.5) / 1000.);
        } else {
            self.alpha = Some(5.);
            self.beta = Some(1.2);
        }
    }
    /// Apply corrections
    ///
    /// # Arguments
    /// * `sigma_effective` - Effective overburden pressure in ton
    /// * `fine_content` - Percentage of fine content in soil in percentage
    /// * `cr` - rod length correction factor
    /// * `cs` - sampler correction factor
    /// * `cb` - borehole diameter correction factor
    /// * `ce` - energy correction factor
    pub fn apply_corrections(
        &mut self,
        sigma_effective: f64,
        fine_content: f64,
        cr: f64,
        cs: f64,
        cb: f64,
        ce: f64,
    ) {
        self.apply_energy_correction(ce);
        self.set_cn(sigma_effective);
        self.set_alpha_beta(fine_content);

        if let (Some(n60), Some(cn), Some(alpha), Some(beta)) =
            (self.n60, self.cn, self.alpha, self.beta)
        {
            let n1_60 = n60.mul_by_f64(cn * cr * cs * cb);
            self.n1_60 = Some(n1_60);
            self.n1_60f = Some(n1_60.mul_by_f64(beta).add_f64(alpha));
        }
    }
}
// -------------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct SPTExp {
    pub blows: Vec<SPTBlow>,
    pub name: String,
}

impl SPTExp {
    /// Create a new SPTExp
    ///
    /// # Arguments
    /// * `blows` - List of SPTBlow
    /// * `name` - Name of the experiment
    pub fn new(blows: Vec<SPTBlow>, name: String) -> Self {
        Self { blows, name }
    }

    /// Add a new blow to the experiment
    ///
    /// # Arguments
    /// * `depth` - Depth of the blow
    /// * `n1` - N-value of the first blow
    /// * `n2` - N-value of the second blow
    /// * `n3` - N-value of the third blow
    pub fn add_blow(&mut self, depth: f64, n1: NValue, n2: NValue, n3: NValue) {
        self.blows.push(SPTBlow::new(depth, n1, n2, n3));
    }

    /// Apply corrections
    ///
    /// # Arguments
    /// * `sigma_effective` - Effective overburden pressure in ton
    /// * `fine_content` - Percentage of fine content in soil in percentage
    /// * `cr` - rod length correction factor
    /// * `cs` - sampler correction factor
    /// * `cb` - borehole diameter correction factor
    /// * `ce` - energy correction factor
    pub fn apply_corrections(
        &mut self,
        sigma_effective: f64,
        fine_content: f64,
        cr: f64,
        cs: f64,
        cb: f64,
        ce: f64,
    ) {
        self.blows
            .iter_mut()
            .for_each(|blow| blow.apply_corrections(sigma_effective, fine_content, cr, cs, cb, ce));
    }
}

// -------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SPT {
    pub exps: Vec<SPTExp>,
    pub energy_correction_factor: f64,
    pub diameter_correction_factor: f64,
    pub sampler_correction_factor: f64,
    pub rod_length_correction_factor: f64,
}
impl SPT {
    /// Create a new SPT
    ///
    /// # Arguments
    /// * `energy_correction_factor` - Energy correction factor to convert N value to N60
    /// * `diameter_correction_factor` - Borehole diameter correction factor
    /// * `sampler_correction_factor` - Sampler correction factor
    /// * `rod_length_correction_factor` - Rod length correction factor
    pub fn new(
        energy_correction_factor: f64,
        diameter_correction_factor: f64,
        sampler_correction_factor: f64,
        rod_length_correction_factor: f64,
    ) -> Self {
        Self {
            exps: Vec::new(),
            energy_correction_factor,
            diameter_correction_factor,
            sampler_correction_factor,
            rod_length_correction_factor,
        }
    }

    /// Add a new experiment to the SPT
    ///
    /// # Arguments
    /// * `exp` - SPTExp
    pub fn add_exp(&mut self, exp: SPTExp) {
        self.exps.push(exp);
    }

    /// Get the idealized experiment
    ///
    /// # Arguments
    /// * `mode` - Idealized mode to use when combining the layers
    /// * `name` - Name of the idealized experiment
    ///
    /// # Returns
    /// * `SPTExp` - Idealized experiment
    pub fn get_idealized_exp(&self, mode: SelectionMethod, name: String) -> SPTExp {
        let mut depth_map: BTreeMap<OrderedFloat<f64>, Vec<NValue>> = BTreeMap::new();

        // Collect all unique depths and corresponding `n` values
        for exp in &self.exps {
            for blow in &exp.blows {
                depth_map
                    .entry(OrderedFloat(blow.depth))
                    .or_default()
                    .push(blow.n);
            }
        }

        // Create a new SPTExp with selected values
        let mut idealized_blows = Vec::new();

        for (&depth, n_values) in &depth_map {
            let selected_n = match mode {
                SelectionMethod::Min => *n_values.iter().min().unwrap(), // Refusal is best
                SelectionMethod::Max => *n_values.iter().max().unwrap(), // Refusal is best
                SelectionMethod::Avg => {
                    let sum: f64 = n_values
                        .iter()
                        .filter_map(|&n| n.to_option().map(|v| v as f64))
                        .sum();
                    let count = n_values.len();

                    NValue::from_i32((sum / count as f64).round() as i32)
                }
            };

            // Add to new SPTExp
            idealized_blows.push(SPTBlow {
                depth: depth.into_inner(),
                n1: NValue::Value(0), // Placeholder (could be refined if needed)
                n2: NValue::Value(0),
                n3: NValue::Value(0),
                n: selected_n,
                ..Default::default()
            });
        }

        SPTExp::new(idealized_blows, name)
    }
}
