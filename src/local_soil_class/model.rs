#[derive(Debug, Clone)]
pub struct LocalSoilClassCommonParams {
    pub depth: Vec<f64>,
    pub h: Vec<f64>,
    pub n: Vec<f64>,
    pub h_n: Vec<f64>,
    pub sum_h_n: f64,
    pub n_30: f64,
    pub soil_class: String,
}
