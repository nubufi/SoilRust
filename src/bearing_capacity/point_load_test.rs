use serde::Serialize;

use crate::models::point_load_test::PointLoadExp;

/// Represents the bearing capacity result for a given soil and foundation setup.
#[derive(Debug, Serialize)]
pub struct Output {
    /// Is50 value in MPa.
    pub is50: f64,

    /// Uniaxial compressive strength (UCS) in MPa.
    pub ucs: f64,

    /// Generalized value of C.
    pub c: f64,

    /// Equivalent core diameter in mm.
    pub d: f64,

    /// Allowable bearing capacity in ton/m2.
    pub allowable_bearing_capacity: f64,

    /// The pressure exerted by the foundation in ton/m2.
    pub foundation_pressure: f64,

    /// Indicates the depth at which the bearing capacity is calculated in meters.
    pub df: f64,

    /// Indicates whether the bearing capacity is safe.
    pub is_safe: bool,

    /// Safety factor used in the design.
    pub safety_factor: f64,
}

/// Calculates the generalized size correction factor `C` based on the given equivalent core diameter `D`.
///
/// This follows the standard chart provided by ASTM and ISRM guidelines for point load tests, interpolating intermediate values.
///
/// # Arguments
/// * `d` - Sample diameter in millimeters (mm).
///
/// # Returns
/// * `f64` - The generalized correction factor `C`.
pub fn get_generalized_c_value(d: f64) -> f64 {
    // Diameter (mm) to C values mapping
    let diameters = [
        (20., 17.5),
        (30., 19.),
        (40., 21.),
        (50., 23.),
        (54., 24.),
        (60., 24.5),
    ];

    if d <= diameters[0].0 {
        return diameters[0].1;
    }

    if d >= diameters.last().unwrap().0 {
        return diameters.last().unwrap().1;
    }

    // Interpolate intermediate values
    for i in 0..diameters.len() - 1 {
        let (d_lower, c_lower) = diameters[i];
        let (d_upper, c_upper) = diameters[i + 1];

        if d >= d_lower && d <= d_upper {
            let fraction = (d - d_lower) / (d_upper - d_lower);
            return c_lower + fraction * (c_upper - c_lower);
        }
    }
    unreachable!()
}

/// Calculates the bearing capacity of a foundation based on point load test results.
///
/// # Arguments
/// * `point_load_test_exp` - The point load test experiment data.
/// * `df` - Depth at which to calculate the bearing capacity.
/// * `foundation_pressure` - The pressure exerted by the foundation.
/// * `safety_factor` - The safety factor for the design.
///
/// # Returns
/// * `Output` - The bearing capacity result containing various parameters.
pub fn calc_bearing_capacity(
    point_load_test_exp: PointLoadExp,
    df: f64,
    foundation_pressure: f64,
    safety_factor: f64,
) -> Output {
    let sample = point_load_test_exp.get_sample_at_depth(df);

    let is50 = sample.is50;
    let d = sample.d;
    let c = get_generalized_c_value(d);

    let ucs = is50 * c;

    let allowable_bearing_capacity = ucs / safety_factor;
    let is_safe = allowable_bearing_capacity >= foundation_pressure;

    Output {
        is50,
        ucs,
        c,
        d,
        allowable_bearing_capacity,
        is_safe,
        safety_factor,
        foundation_pressure,
        df,
    }
}
