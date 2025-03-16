/// Represents a single CPT (Cone Penetration Test) data point.
///
/// Each `CPTExp` instance holds a `depth` value (in meters) and a `cone_resistance` value (in MPa).
#[derive(Debug,Clone)]
pub struct CPTExp {
    pub depth: f64,
    pub cone_resistance: f64,
}

impl CPTExp {
    pub fn new(depth: f64, cone_resistance: f64) -> Self {
        Self { depth, cone_resistance }
    }
}

/// Represents a collection of CPT data points.
///
/// A `CPT` struct contains multiple `CPTExp` instances, forming a complete CPT profile.
pub struct CPT {
    pub exps : Vec<CPTExp>
}

impl CPT {
    pub fn new(exps: Vec<CPTExp>) -> Self {
        Self { exps }
    }
}