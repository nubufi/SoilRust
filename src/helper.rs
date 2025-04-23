use std::fmt::Display;

/// Performs linear interpolation for a given x value based on provided x and y vectors.
///
/// # Arguments
/// * `x_values` - Array of x-axis values (must be sorted)
/// * `y_values` - Array of y-axis values
/// * `x` - The x value for which to interpolate
///
/// # Returns
/// * Interpolated y value as f64
///
/// # Panics
/// If x_values and y_values lengths are not equal or x is out of range.
pub fn interp1d(x_values: &[f64], y_values: &[f64], x: f64) -> f64 {
    assert_eq!(
        x_values.len(),
        y_values.len(),
        "x_values and y_values must have the same length"
    );

    if x <= x_values[0] {
        return y_values[0];
    }
    if x >= x_values[x_values.len() - 1] {
        return y_values[y_values.len() - 1];
    }

    for i in 0..x_values.len() - 1 {
        let x0 = x_values[i];
        let x1 = x_values[i + 1];
        let y0 = y_values[i];
        let y1 = y_values[i + 1];

        if x >= x0 && x <= x1 {
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0);
        }
    }

    panic!("Interpolation error: x-value out of interpolation range");
}

/// Validates a single optional numeric field against optional bounds.
///
/// # Arguments
/// * `field_name` - Field name for use in the error message.
/// * `value` - Option<T> where T: numeric type (e.g., f64, i32, etc.).
/// * `min` - Optional minimum bound (inclusive).
/// * `max` - Optional maximum bound (inclusive).
///
/// # Returns
/// * `Ok(())` if the value is present and within bounds, otherwise `Err(String)`
pub fn validate_field<T>(
    field_name: &str,
    value: Option<T>,
    min: Option<T>,
    max: Option<T>,
) -> Result<(), String>
where
    T: PartialOrd + Display + Copy,
{
    let val = value.ok_or_else(|| format!("{} must be provided.", field_name))?;

    if let Some(min_val) = min {
        if val < min_val {
            return Err(format!("{} must be ≥ {}.", field_name, min_val));
        }
    }

    if let Some(max_val) = max {
        if val > max_val {
            return Err(format!("{} must be ≤ {}.", field_name, max_val));
        }
    }

    Ok(())
}
