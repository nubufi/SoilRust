use crate::{
    models::{foundation::Foundation, soil_profile::SoilProfile},
    validation::{validate_field, ValidationError},
};

/// Validates the input data for elastic settlement calculations.
///
/// # Arguments
/// * `soil_profile` - The soil profile data.
/// * `foundation` - The foundation data.
/// * `foundation_pressure` - The foundation pressure (q) [t/m²].
///
/// # Returns
/// * `Result<(), &'static str>`: Ok if valid, Err with a message if invalid.
pub fn validate_input(
    soil_profile: &SoilProfile,
    foundation: &Foundation,
    foundation_pressure: f64,
) -> Result<(), ValidationError> {
    soil_profile.validate(&["thickness", "dry_unit_weight", "saturated_unit_weight"])?;
    foundation.validate(&["foundation_depth", "foundation_width", "foundation_length"])?;

    validate_field(
        "foundation_pressure",
        Some(foundation_pressure),
        Some(0.0),
        None,
        "loads",
    )?;

    Ok(())
}
/// Calculates the difference between the stress increment (Δσ) and 10% of effective stress at depth `z`.
fn get_difference(z: f64, f: f64, b: f64, df: f64, l: f64, sp: &SoilProfile) -> f64 {
    let dg = f / ((b + z - df) * (l + z - df));
    let effective_stress = sp.calc_effective_stress(z);
    dg - 0.1 * effective_stress
}

/// Finds the effective depth where the stress increment equals 10% of effective stress using the bisection method.
fn find_effective_depth(f: f64, b: f64, df: f64, l: f64, sp: &SoilProfile) -> f64 {
    let mut boundary1 = df;
    let mut boundary2 = df + 1.5 * b;
    let mut middle = (boundary1 + boundary2) / 2.0;
    let mut n = 0;

    // Check if both ends have same sign, then widen the boundary
    if get_difference(boundary1, f, b, df, l, sp) * get_difference(boundary2, f, b, df, l, sp) > 0.0
    {
        boundary2 = 100.0 * b;
    }

    // Bisection loop
    while get_difference(middle, f, b, df, l, sp).abs() > 0.01 && n < 100 {
        n += 1;
        if boundary1 == boundary2 && boundary1 == middle && n > 10 {
            return 0.0;
        }

        if get_difference(middle, f, b, df, l, sp) > 0.0 {
            boundary1 = middle;
        } else {
            boundary2 = middle;
        }

        middle = (boundary1 + boundary2) / 2.0;
    }

    middle
}

/// Public function to calculate effective depth based on foundation and soil data.
///
/// # Arguments
/// * `soil_profile` - A reference to a `SoilProfile` object.
/// * `foundation_data` - A reference to a `Foundation` object.
/// * `foundation_pressure` - The pressure applied by the foundation in ton/m2.
///
/// # Returns
/// * The effective depth as a `f64` value in meters.
pub fn calc_effective_depth(
    soil_profile: &SoilProfile,
    foundation_data: &Foundation,
    foundation_pressure: f64,
) -> Result<f64, ValidationError> {
    validate_input(soil_profile, foundation_data, foundation_pressure)?;

    let df = foundation_data.foundation_depth.unwrap();
    let b = foundation_data.foundation_width.unwrap();
    let l = foundation_data.foundation_length.unwrap();

    let q_net = foundation_pressure - soil_profile.calc_normal_stress(df);
    let f = q_net * b * l;

    let result = find_effective_depth(f, b, df, l, soil_profile);

    Ok(result)
}
