use soilrust::soil_coefficient::{calc_by_bearing_capacity, calc_by_settlement};

#[test]
fn test_calc_soil_coefficient_by_settlement_positive() {
    let settlement = 2.; // in meters
    let foundation_load = 1000.0;
    let result = calc_by_settlement(settlement, foundation_load);
    assert!((result - 50_000.0).abs() < 1e-6);
}

#[test]
fn test_calc_soil_coefficient_by_settlement_zero() {
    let settlement = 0.0; // in meters
    let foundation_load = 1000.0;
    let result = calc_by_settlement(settlement, foundation_load);
    assert_eq!(result, 999_999.0);
}

#[test]
fn test_calc_soil_coefficient_by_bearing_capacity() {
    let bearing_capacity = 250.0;
    let result = calc_by_bearing_capacity(bearing_capacity);
    assert!((result - 100_000.0).abs() < 1e-6);
}
