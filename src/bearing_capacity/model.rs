use serde::Serialize;
/// Bearing capacity factors according to Terzaghi, Meyerhof, Hansen, etc.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct BearingCapacityFactors {
    pub nc: f64,
    pub nq: f64,
    pub ng: f64, // sometimes denoted as Nγ
}

/// Shape modification factors used in bearing capacity equations.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct ShapeFactors {
    pub sc: f64,
    pub sq: f64,
    pub sg: f64,
}

/// Inclination modification factors for inclined load conditions.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct InclinationFactors {
    pub ic: f64,
    pub iq: f64,
    pub ig: f64,
}

/// Base inclination factors depending on foundation base angle.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct BaseFactors {
    pub bc: f64,
    pub bq: f64,
    pub bg: f64,
}

/// Ground slope modification factors affecting bearing capacity.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct GroundFactors {
    pub gc: f64,
    pub gq: f64,
    pub gg: f64,
}

/// Depth modification factors for accounting foundation embedment.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct DepthFactors {
    pub dc: f64,
    pub dq: f64,
    pub dg: f64,
}

/// Soil parameters used in bearing capacity calculations.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct SoilParams {
    pub friction_angle: f64,
    pub cohesion: f64,
    pub unit_weight: f64,
}

#[derive(Debug, Serialize)]
pub struct BearingCapacityResult {
    pub bearing_capacity_factors: BearingCapacityFactors,
    pub shape_factors: ShapeFactors,
    pub depth_factors: DepthFactors,
    pub load_inclination_factors: InclinationFactors,
    pub ground_factors: GroundFactors,
    pub base_factors: BaseFactors,
    pub soil_params: SoilParams,
    pub ultimate_bearing_capacity: f64,
    pub allowable_bearing_capacity: f64,
    pub is_safe: bool,
}
