use serde::Serialize;
use std::fmt::{self, Display};

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub code: String,
    pub message: String, // English fallback (optional but helpful for debugging)
}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}
impl From<ValidationError> for String {
    fn from(err: ValidationError) -> Self {
        format!("[{}] {}", err.code, err.message)
    }
}

/// Validates a single optional numeric field against optional bounds, returning a structured error.
///
/// # Arguments
/// * `field_name` - A name for the field (e.g. "cu")
/// * `value` - Option<T> to validate
/// * `min` - Optional minimum value (inclusive)
/// * `max` - Optional maximum value (inclusive)
/// * `error_code_prefix` - A short prefix for generating the error code, e.g., "layer"
///
/// # Returns
/// Ok(()) if valid, Err(ValidationError) otherwise
pub fn validate_field<T>(
    field_name: &str,
    value: Option<T>,
    min: Option<T>,
    max: Option<T>,
    error_code_prefix: &str,
) -> Result<(), ValidationError>
where
    T: PartialOrd + Display + Copy,
{
    let val = value.ok_or(ValidationError {
        code: format!("{}.{}.missing", error_code_prefix, field_name),
        message: format!("{} must be provided.", field_name),
    })?;

    if let Some(min_val) = min {
        if val < min_val {
            return Err(ValidationError {
                code: format!("{}.{}.too_small.{}", error_code_prefix, field_name, min_val),
                message: format!(
                    "{} must be greater than or equal to {}.",
                    field_name, min_val
                ),
            });
        }
    }

    if let Some(max_val) = max {
        if val > max_val {
            return Err(ValidationError {
                code: format!("{}.{}.too_large.{}", error_code_prefix, field_name, max_val),
                message: format!("{} must be less than or equal to {}.", field_name, max_val),
            });
        }
    }

    Ok(())
}
