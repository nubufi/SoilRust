// Representation of the IF table for the reduction factors
// for the elastic settlement calculation
// 1st dimension: poisson ratio [0,0.1,0.3,0.4,0.5]
// 2nd dimension: df/B [0.05,0.1,0.2,0.4,0.6,0.8,1,2]
// 3rd dimension: L/B [1,1.2,1.4,1.6,1.8,2,5]
pub struct IfTable {
    pub values: &'static [&'static [&'static [f64]]],
}

const IF_TABLE: IfTable = IfTable {
    values: &[
        &[
            &[0.950, 0.954, 0.957, 0.959, 0.961, 0.963, 0.973],
            &[0.904, 0.911, 0.917, 0.922, 0.925, 0.928, 0.948],
            &[0.825, 0.838, 0.847, 0.855, 0.862, 0.867, 0.903],
            &[0.710, 0.727, 0.740, 0.752, 0.761, 0.769, 0.827],
            &[0.635, 0.652, 0.666, 0.678, 0.689, 0.698, 0.769],
            &[0.585, 0.600, 0.614, 0.626, 0.637, 0.646, 0.723],
            &[0.549, 0.563, 0.576, 0.587, 0.598, 0.607, 0.686],
            &[0.468, 0.476, 0.484, 0.492, 0.499, 0.506, 0.577],
        ],
        &[
            &[0.958, 0.962, 0.965, 0.967, 0.968, 0.970, 0.978],
            &[0.919, 0.926, 0.930, 0.934, 0.938, 0.940, 0.957],
            &[0.848, 0.859, 0.868, 0.875, 0.881, 0.886, 0.917],
            &[0.739, 0.755, 0.768, 0.779, 0.788, 0.795, 0.848],
            &[0.665, 0.682, 0.696, 0.708, 0.718, 0.727, 0.793],
            &[0.615, 0.630, 0.644, 0.656, 0.667, 0.676, 0.749],
            &[0.579, 0.593, 0.606, 0.618, 0.628, 0.637, 0.714],
            &[0.496, 0.505, 0.513, 0.521, 0.528, 0.535, 0.606],
        ],
        &[
            &[0.979, 0.981, 0.982, 0.983, 0.984, 0.985, 0.990],
            &[0.954, 0.958, 0.962, 0.964, 0.966, 0.968, 0.977],
            &[0.902, 0.911, 0.917, 0.923, 0.927, 0.930, 0.951],
            &[0.808, 0.823, 0.834, 0.843, 0.851, 0.857, 0.899],
            &[0.738, 0.754, 0.767, 0.778, 0.788, 0.796, 0.852],
            &[0.687, 0.703, 0.716, 0.728, 0.738, 0.747, 0.813],
            &[0.650, 0.665, 0.678, 0.689, 0.700, 0.709, 0.780],
            &[0.562, 0.571, 0.580, 0.588, 0.596, 0.603, 0.675],
        ],
        &[
            &[0.989, 0.990, 0.991, 0.992, 0.992, 0.993, 0.995],
            &[0.973, 0.976, 0.978, 0.980, 0.981, 0.982, 0.988],
            &[0.932, 0.940, 0.945, 0.949, 0.952, 0.955, 0.970],
            &[0.848, 0.862, 0.872, 0.881, 0.887, 0.893, 0.927],
            &[0.779, 0.795, 0.808, 0.819, 0.828, 0.836, 0.886],
            &[0.727, 0.743, 0.757, 0.769, 0.779, 0.788, 0.849],
            &[0.689, 0.704, 0.718, 0.730, 0.740, 0.749, 0.818],
            &[0.596, 0.606, 0.615, 0.624, 0.632, 0.640, 0.714],
        ],
        &[
            &[0.997, 0.997, 0.998, 0.998, 0.998, 0.998, 0.999],
            &[0.988, 0.990, 0.991, 0.992, 0.993, 0.993, 0.996],
            &[0.960, 0.966, 0.969, 0.972, 0.974, 0.976, 0.985],
            &[0.886, 0.899, 0.908, 0.916, 0.922, 0.926, 0.953],
            &[0.818, 0.834, 0.847, 0.857, 0.866, 0.873, 0.917],
            &[0.764, 0.781, 0.795, 0.807, 0.817, 0.826, 0.883],
            &[0.723, 0.740, 0.754, 0.766, 0.777, 0.786, 0.852],
            &[0.622, 0.633, 0.643, 0.653, 0.662, 0.670, 0.747],
        ],
    ],
};

const NU_VALUES: [f64; 5] = [0.0, 0.1, 0.3, 0.4, 0.5];
const D_B_VALUES: [f64; 8] = [0.05, 0.1, 0.2, 0.4, 0.6, 0.8, 1.0, 2.0];
const L_B_VALUES: [f64; 7] = [1.0, 1.2, 1.4, 1.6, 1.8, 2.0, 5.0];

fn find_bounds(values: &[f64], target: f64) -> (usize, usize) {
    for i in 0..values.len() - 1 {
        if target >= values[i] && target <= values[i + 1] {
            return (i, i + 1);
        }
    }
    (values.len() - 2, values.len() - 1) // fallback to last bounds
}

/// Interpolates the IF value for the given parameters
///
/// # Arguments
///
/// * `nu` - Poisson ratio
/// * `d_b` - df/B ratio
/// * `l_b` - L/B ratio
///
/// # Returns
///
/// The interpolated IF value
pub fn interpolate_if(nu: f64, d_b: f64, l_b: f64) -> f64 {
    let nu = nu.clamp(0., 0.5);
    let d_b = d_b.clamp(0.05, 2.0);
    let l_b = l_b.clamp(1.0, 5.0);

    let (nu_i0, nu_i1) = find_bounds(&NU_VALUES, nu);
    let (d_b_i0, d_b_i1) = find_bounds(&D_B_VALUES, d_b);
    let (l_b_i0, l_b_i1) = find_bounds(&L_B_VALUES, l_b);

    let nu0 = NU_VALUES[nu_i0];
    let nu1 = NU_VALUES[nu_i1];
    let d_b0 = D_B_VALUES[d_b_i0];
    let d_b1 = D_B_VALUES[d_b_i1];
    let l_b0 = L_B_VALUES[l_b_i0];
    let l_b1 = L_B_VALUES[l_b_i1];

    let if000 = IF_TABLE.values[nu_i0][d_b_i0][l_b_i0];
    let if001 = IF_TABLE.values[nu_i0][d_b_i0][l_b_i1];
    let if010 = IF_TABLE.values[nu_i0][d_b_i1][l_b_i0];
    let if011 = IF_TABLE.values[nu_i0][d_b_i1][l_b_i1];

    let if100 = IF_TABLE.values[nu_i1][d_b_i0][l_b_i0];
    let if101 = IF_TABLE.values[nu_i1][d_b_i0][l_b_i1];
    let if110 = IF_TABLE.values[nu_i1][d_b_i1][l_b_i0];
    let if111 = IF_TABLE.values[nu_i1][d_b_i1][l_b_i1];

    // Linear interp function
    let lerp = |x0: f64, x1: f64, t: f64| x0 * (1.0 - t) + x1 * t;

    let tx = (nu - nu0) / (nu1 - nu0);
    let ty = (d_b - d_b0) / (d_b1 - d_b0);
    let tz = (l_b - l_b0) / (l_b1 - l_b0);

    let c00 = lerp(if000, if100, tx);
    let c01 = lerp(if001, if101, tx);
    let c10 = lerp(if010, if110, tx);
    let c11 = lerp(if011, if111, tx);

    let c0 = lerp(c00, c10, ty);
    let c1 = lerp(c01, c11, ty);

    lerp(c0, c1, tz)
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    /// Case 1: All exact match
    #[test]
    fn test_case_1() {
        let result = interpolate_if(0.0, 0.05, 1.0);
        let expected = 0.950;
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    /// Case 2: nu is interpolated, d_b and l_b are exact match
    #[test]
    fn test_case_2() {
        let result = interpolate_if(0.05, 0.05, 1.0);
        let expected = 0.954;
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    /// Case 3: d_b is interpolated, nu and l_b are exact match
    #[test]
    fn test_case_3() {
        let result = interpolate_if(0., 0.7, 1.0);
        let expected = 0.61;
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    /// Case 4: l_b is interpolated, nu and d_b are exact match
    #[test]
    fn test_case_4() {
        let result = interpolate_if(0., 0.05, 1.1);
        let expected = 0.952;
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    /// Case 5: d_b and l_b are exact match, nu > 0.5
    #[test]
    fn test_case_5() {
        let result = interpolate_if(0.6, 0.05, 1.);
        let expected = 0.997;
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    /// Case 6: d_b and l_b are interpolated, nu is exact match.
    #[test]
    fn test_case_6() {
        let result = interpolate_if(0., 0.3, 1.3);
        let expected = 0.788;
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }

    /// Case 7: All values are interpolated
    #[test]
    fn test_case_7() {
        let result = interpolate_if(0.05, 0.3, 1.3);
        let expected = 0.80025;
        assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
    }
}
