use approx::assert_abs_diff_eq;
use soilrust::{
    bearing_capacity::point_load_test::{calc_bearing_capacity, get_generalized_c_value},
    models::point_load_test::{PointLoadExp, PointLoadSample},
};

#[test]
fn test_get_generalized_c_value() {
    // Test cases for the generalized size correction factor `C`
    let test_cases = [(10.0, 17.5), (30.0, 19.0), (45.0, 22.0), (65.0, 24.5)];

    for &(d, expected_c) in &test_cases {
        let c = get_generalized_c_value(d);
        assert!(
            (c - expected_c).abs() < f64::EPSILON,
            "Failed for d = {}",
            d
        );
    }
}

#[test]
fn test_calc_bearing_capacity() {
    let exp = PointLoadExp::new("Test".to_string(), vec![PointLoadSample::new(20., 2., 50.)]);

    let df = 20.0;
    let foundation_pressure = 100.0;
    let safety_factor = 2.0;

    let result = calc_bearing_capacity(exp, df, foundation_pressure, safety_factor);

    assert_eq!(result.c, 23.0);
    assert_abs_diff_eq!(result.ucs, 46.0, epsilon = 1e-5);
    assert_abs_diff_eq!(result.allowable_bearing_capacity, 23.0, epsilon = 1e-5);
}
