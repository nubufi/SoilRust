/// Calculates stress reduction factor (rd) based on depth
///
/// # Arguments
/// * `depth` - Depth in meters
///
/// # Returns
/// * `rd` - Stress reduction coefficient
pub fn calc_rd(depth: f64) -> f64 {
    match depth {
        z if z <= 9.15 => 1.0 - 0.00765 * z,
        z if z > 9.15 && z < 23.0 => 1.174 - 0.0267 * z,
        z if (23.0..30.0).contains(&z) => 0.744 - 0.008 * z,
        _ => 0.5,
    }
}

/// Calculates cyclic stress ratio (CSR) based on PGA, normal stress, and rd
///
/// # Arguments
/// * `pga` - Peak Ground Acceleration
/// * `normal_stress` - Normal stress in ton/m²
pub fn calc_csr(pga: f64, normal_stress: f64, rd: f64) -> f64 {
    0.65 * pga * normal_stress * rd
}

/// Calculates magnitude scaling factor (MSF) based on moment magnitude
///
/// # Arguments
/// * `mw` - Moment magnitude
///
/// # Returns
/// * `msf` - Magnitude scaling factor
pub fn calc_msf(mw: f64) -> f64 {
    10.0_f64.powf(2.24) / mw.powf(2.56)
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn test_calc_rd_shallow_depth() {
        // Test values for depth <= 9.15
        let depth = 5.0;
        let expected = 1.0 - 0.00765 * depth;
        let result = calc_rd(depth);
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    #[test]
    fn test_calc_rd_transition_depth_9_15() {
        // Edge case at depth = 9.15
        let depth = 9.15;
        let expected = 1.0 - 0.00765 * depth;
        let result = calc_rd(depth);
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    #[test]
    fn test_calc_rd_intermediate_depth() {
        // Test value between 9.15 and 23.0
        let depth = 15.0;
        let expected = 1.174 - 0.0267 * depth;
        let result = calc_rd(depth);
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    #[test]
    fn test_calc_rd_range_23_to_30() {
        // Test value in 23.0 <= z < 30.0
        let depth = 25.0;
        let expected = 0.744 - 0.008 * depth;
        let result = calc_rd(depth);
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    #[test]
    fn test_calc_rd_exact_30() {
        // Edge case at 30.0, should fall into default case
        let depth = 30.0;
        let expected = 0.5;
        let result = calc_rd(depth);
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    #[test]
    fn test_calc_rd_greater_than_30() {
        // Test depth > 30.0
        let depth = 35.0;
        let expected = 0.5;
        let result = calc_rd(depth);
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }
    #[test]
    fn test_calc_csr() {
        let pga = 0.3; // g
        let normal_stress = 10.0; // ton/m²
        let rd = 0.9;
        let expected = 0.65 * pga * normal_stress * rd;
        let result = calc_csr(pga, normal_stress, rd);
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    #[test]
    fn test_calc_msf_typical_magnitude() {
        let mw: f64 = 7.5;
        let expected = 10.0_f64.powf(2.24) / mw.powf(2.56);
        let result = calc_msf(mw);
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }
}
