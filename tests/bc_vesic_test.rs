use approx::assert_abs_diff_eq;
use soilrust::{
    bearing_capacity::{model::BearingCapacityFactors, vesic::*},
    models::{foundation::Foundation, loads::Loads},
};

/// Case 1: φ = 0°, pure cohesive soil — should return Nc = 5.14, Nq = 1.0, Ng = 0.0
#[test]
fn test_calc_bearing_capacity_factors_1() {
    let phi = 0.0;
    let result = calc_bearing_capacity_factors(phi);

    assert_abs_diff_eq!(result.nc, 5.14, epsilon = 1e-3);
    assert_abs_diff_eq!(result.nq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.ng, 0., epsilon = 1e-3);
}

/// Case 2: φ = 10° — soft granular soil
#[test]
fn test_calc_bearing_capacity_factors_2() {
    let phi = 10.0;
    let result = calc_bearing_capacity_factors(phi);

    assert_abs_diff_eq!(result.nc, 8.345, epsilon = 1e-3);
    assert_abs_diff_eq!(result.nq, 2.471, epsilon = 1e-3);
    assert_abs_diff_eq!(result.ng, 0.519, epsilon = 1e-3);
}

/// Case 3: φ = 30° — typical for dense sand
#[test]
fn test_calc_bearing_capacity_factors_3() {
    let phi = 30.0;
    let result = calc_bearing_capacity_factors(phi);

    assert_abs_diff_eq!(result.nc, 30.14, epsilon = 1e-3);
    assert_abs_diff_eq!(result.nq, 18.401, epsilon = 1e-3);
    assert_abs_diff_eq!(result.ng, 20.093, epsilon = 1e-3);
}
// --------------------------------------------------------------
// Case 1: φ = 00°
#[test]
fn test_calc_shape_factors_1() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 1.0,
        foundation_length: 1.5,
        ..Foundation::default()
    };
    let phi = 0.0;

    let bc_factors = BearingCapacityFactors {
        nc: 5.14,
        nq: 1.,
        ng: 0.0,
    };

    let result = calc_shape_factors(&foundation, bc_factors, phi);
    assert_abs_diff_eq!(result.sc, 0.133, epsilon = 1e-3);
    assert_abs_diff_eq!(result.sq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.sg, 0.733, epsilon = 1e-3);
}

/// Case 2: φ = 20°, B/L = 3/6 = 0.5, Nq = 10.0, Nc = 20.0
#[test]
fn test_calc_shape_factors_2() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 1.0,
        foundation_length: 1.5,
        ..Foundation::default()
    };
    let phi = 30.0;

    let bc_factors = BearingCapacityFactors {
        nc: 30.140,
        nq: 18.401,
        ng: 20.093,
    };

    let result = calc_shape_factors(&foundation, bc_factors, phi);
    assert_abs_diff_eq!(result.sc, 1.407, epsilon = 1e-3);
    assert_abs_diff_eq!(result.sq, 1.333, epsilon = 1e-3);
    assert_abs_diff_eq!(result.sg, 0.733, epsilon = 1e-3);
}
// --------------------------------------------------------------
/// Case 1: φ = 0°, c = 10, HL = 0, HB = 0, V = 200
#[test]
fn test_calc_inclination_factors_1() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 4.0,
        foundation_length: 6.0,
        effective_width: Some(1.0),
        effective_length: Some(1.5),
        ..Default::default()
    };

    let loads = Loads {
        vertical_load: Some(200.0),
        horizontal_load_x: Some(0.),
        horizontal_load_y: Some(0.),
        ..Default::default()
    };

    let bc_factors = BearingCapacityFactors {
        nc: 1.,
        nq: 1.,
        ng: 1.,
    };
    let phi = 0.0;
    let cohesion = 10.0;

    let result = calc_inclination_factors(phi, cohesion, bc_factors, &foundation, &loads);
    assert_abs_diff_eq!(result.ic, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.iq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.ig, 1., epsilon = 1e-3);
}
/// Case 2: φ = 30°, c = 0, HL = 0, HB = 10, V = 200
#[test]
fn test_calc_inclination_factors_2() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 1.0,
        foundation_length: 1.5,
        effective_width: Some(1.0),
        effective_length: Some(1.5),
        ..Default::default()
    };

    let loads = Loads {
        vertical_load: Some(200.0),
        horizontal_load_x: Some(00.),
        horizontal_load_y: Some(0.),
        ..Default::default()
    };
    let bc_factors = BearingCapacityFactors {
        nc: 1.,
        nq: 18.401,
        ng: 1.,
    };
    let phi = 30.0;
    let cohesion = 0.0;

    let result = calc_inclination_factors(phi, cohesion, bc_factors, &foundation, &loads);
    assert_abs_diff_eq!(result.ic, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.iq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.ig, 1., epsilon = 1e-3);
}
/// Case 3: φ = 30°, c = 10, HL = 0, HB = 10, V = 200
#[test]
fn test_calc_inclination_factors_3() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 1.0,
        foundation_length: 1.5,
        effective_width: Some(1.0),
        effective_length: Some(1.5),
        ..Default::default()
    };

    let loads = Loads {
        vertical_load: Some(200.0),
        horizontal_load_x: Some(10.),
        horizontal_load_y: Some(0.),
        ..Default::default()
    };
    let bc_factors = BearingCapacityFactors {
        nc: 1.,
        nq: 18.401,
        ng: 1.,
    };
    let phi = 30.0;
    let cohesion = 10.0;

    let result = calc_inclination_factors(phi, cohesion, bc_factors, &foundation, &loads);
    assert_abs_diff_eq!(result.ic, 0.924, epsilon = 1e-3);
    assert_abs_diff_eq!(result.iq, 0.928, epsilon = 1e-3);
    assert_abs_diff_eq!(result.ig, 0.886, epsilon = 1e-3);
}
/// Case 4: φ = 30°, c = 10, HL = 10, HB = 0, V = 200
#[test]
fn test_calc_inclination_factors_4() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 1.0,
        foundation_length: 1.5,
        effective_width: Some(1.0),
        effective_length: Some(1.5),
        ..Default::default()
    };

    let loads = Loads {
        vertical_load: Some(200.0),
        horizontal_load_x: Some(0.),
        horizontal_load_y: Some(10.),
        ..Default::default()
    };
    let bc_factors = BearingCapacityFactors {
        nc: 1.,
        nq: 18.401,
        ng: 1.,
    };
    let phi = 30.0;
    let cohesion = 10.0;

    let result = calc_inclination_factors(phi, cohesion, bc_factors, &foundation, &loads);
    assert_abs_diff_eq!(result.ic, 0.933, epsilon = 1e-3);
    assert_abs_diff_eq!(result.iq, 0.937, epsilon = 1e-3);
    assert_abs_diff_eq!(result.ig, 0.894, epsilon = 1e-3);
}
/// Case 5: φ = 30°, c = 10, HL = 10, HB = 10, V = 200
#[test]
fn test_calc_inclination_factors_5() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 1.0,
        foundation_length: 1.5,
        effective_width: Some(1.0),
        effective_length: Some(1.5),
        ..Default::default()
    };

    let loads = Loads {
        vertical_load: Some(200.0),
        horizontal_load_x: Some(10.),
        horizontal_load_y: Some(10.),
        ..Default::default()
    };
    let bc_factors = BearingCapacityFactors {
        nc: 1.,
        nq: 18.401,
        ng: 1.,
    };
    let phi = 30.0;
    let cohesion = 10.0;

    let result = calc_inclination_factors(phi, cohesion, bc_factors, &foundation, &loads);
    assert_abs_diff_eq!(result.ic, 0.805, epsilon = 1e-3);
    assert_abs_diff_eq!(result.iq, 0.817, epsilon = 1e-3);
    assert_abs_diff_eq!(result.ig, 0.742, epsilon = 1e-3);
}
// --------------------------------------------------------------
/// Case 1: φ = 0°, Df/B = 1
#[test]
fn test_calc_depth_factors_1() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 1.0,
        ..Default::default()
    };

    let phi = 0.0;
    let result = calc_depth_factors(&foundation, phi);
    assert_abs_diff_eq!(result.dc, 0.4, epsilon = 1e-3);
    assert_abs_diff_eq!(result.dq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.dg, 1., epsilon = 1e-3);
}
/// Case 2: φ = 30°, Df/B = 1
#[test]
fn test_calc_depth_factors_2() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 1.0,
        ..Default::default()
    };

    let phi = 30.0;
    let result = calc_depth_factors(&foundation, phi);
    assert_abs_diff_eq!(result.dc, 1.4, epsilon = 1e-3);
    assert_abs_diff_eq!(result.dq, 1.289, epsilon = 1e-3);
    assert_abs_diff_eq!(result.dg, 1., epsilon = 1e-3);
}
/// Case 3: φ = 0°, Df/B > 1
#[test]
fn test_calc_depth_factors_3() {
    let foundation = Foundation {
        foundation_depth: 2.0,
        foundation_width: 1.0,
        ..Default::default()
    };

    let phi = 0.0;
    let result = calc_depth_factors(&foundation, phi);
    assert_abs_diff_eq!(result.dc, 0.0139, epsilon = 1e-3);
    assert_abs_diff_eq!(result.dq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.dg, 1., epsilon = 1e-3);
}
/// Case 4: φ = 30°, Df/B > 1
#[test]
fn test_calc_depth_factors_4() {
    let foundation = Foundation {
        foundation_depth: 2.0,
        foundation_width: 1.0,
        ..Default::default()
    };

    let phi = 30.0;
    let result = calc_depth_factors(&foundation, phi);
    assert_abs_diff_eq!(result.dc, 1.0139, epsilon = 1e-3);
    assert_abs_diff_eq!(result.dq, 1.01, epsilon = 1e-3);
    assert_abs_diff_eq!(result.dg, 1., epsilon = 1e-3);
}
// --------------------------------------------------------------
/// Case 1: φ = 0°, slope = 0°, base = 0°
#[test]
fn test_calc_base_factors_1() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        foundation_length: 2.0,
        base_tilt_angle: Some(0.0),
        slope_angle: Some(0.0),
        ..Default::default()
    };
    let phi = 0.0;
    let result = calc_base_factors(phi, &foundation);

    assert_abs_diff_eq!(result.bc, 0., epsilon = 1e-3);
    assert_abs_diff_eq!(result.bq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.bg, 1., epsilon = 1e-3);
}
/// Case 2: φ = 30°, slope = 0°, base = 0°
#[test]
fn test_calc_base_factors_2() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        foundation_length: 2.0,
        base_tilt_angle: Some(0.0),
        slope_angle: Some(0.0),
        ..Default::default()
    };
    let phi = 30.0;
    let result = calc_base_factors(phi, &foundation);

    assert_abs_diff_eq!(result.bc, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.bq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.bg, 1., epsilon = 1e-3);
}
/// Case 3: φ = 0°, slope = 10°, base = 0°
#[test]
fn test_calc_base_factors_3() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        foundation_length: 2.0,
        base_tilt_angle: Some(0.0),
        slope_angle: Some(10.0),
        ..Default::default()
    };
    let phi = 0.0;
    let result = calc_base_factors(phi, &foundation);

    assert_abs_diff_eq!(result.bc, 0.034, epsilon = 1e-3);
    assert_abs_diff_eq!(result.bq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.bg, 1., epsilon = 1e-3);
}
/// Case 4: φ = 0°, slope = 0°, base = 10°
#[test]
fn test_calc_base_factors_4() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        foundation_length: 2.0,
        base_tilt_angle: Some(10.0),
        slope_angle: Some(0.0),
        ..Default::default()
    };
    let phi = 0.0;
    let result = calc_base_factors(phi, &foundation);

    assert_abs_diff_eq!(result.bc, 0., epsilon = 1e-3);
    assert_abs_diff_eq!(result.bq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.bg, 1., epsilon = 1e-3);
}
/// Case 5: φ = 0°, slope = 10°, base = 10°
#[test]
fn test_calc_base_factors_5() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        foundation_length: 2.0,
        base_tilt_angle: Some(10.0),
        slope_angle: Some(10.0),
        ..Default::default()
    };
    let phi = 0.0;
    let result = calc_base_factors(phi, &foundation);

    assert_abs_diff_eq!(result.bc, 0.034, epsilon = 1e-3);
    assert_abs_diff_eq!(result.bq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.bg, 1., epsilon = 1e-3);
}
/// Case 6: φ = 30°, slope = 10°, base = 10°
#[test]
fn test_calc_base_factors_6() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        foundation_length: 2.0,
        base_tilt_angle: Some(10.0),
        slope_angle: Some(10.0),
        ..Default::default()
    };
    let phi = 30.0;
    let result = calc_base_factors(phi, &foundation);

    assert_abs_diff_eq!(result.bc, 0.882, epsilon = 1e-3);
    assert_abs_diff_eq!(result.bq, 0.809, epsilon = 1e-3);
    assert_abs_diff_eq!(result.bg, 0.809, epsilon = 1e-3);
}
// --------------------------------------------------------------
/// Case 1: φ = 0°, slope = 0°
#[test]
fn test_calc_ground_factors_1() {
    let phi = 0.0;
    let slope = 0.0;
    let iq = 1.0;
    let result = calc_ground_factors(iq, slope, phi);
    assert_abs_diff_eq!(result.gc, 0., epsilon = 1e-3);
    assert_abs_diff_eq!(result.gq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.gg, 1., epsilon = 1e-3);
}
/// Case 2: φ = 30°, slope = 0°
#[test]
fn test_calc_ground_factors_2() {
    let phi = 30.0;
    let slope = 0.0;
    let iq = 0.861;
    let result = calc_ground_factors(iq, slope, phi);
    assert_abs_diff_eq!(result.gc, 0.814, epsilon = 1e-3);
    assert_abs_diff_eq!(result.gq, 1., epsilon = 1e-3);
    assert_abs_diff_eq!(result.gg, 1., epsilon = 1e-3);
}
/// Case 3: φ = 0°, slope = 5°
#[test]
fn test_calc_ground_factors_3() {
    let phi = 0.0;
    let slope = 5.0;
    let iq = 1.;
    let result = calc_ground_factors(iq, slope, phi);
    assert_abs_diff_eq!(result.gc, 0.017, epsilon = 1e-3);
    assert_abs_diff_eq!(result.gq, 0.833, epsilon = 1e-3);
    assert_abs_diff_eq!(result.gg, 0.833, epsilon = 1e-3);
}
/// Case 4: φ = 30°, slope = 5°
#[test]
fn test_calc_ground_factors_4() {
    let phi = 30.0;
    let slope = 5.0;
    let iq = 0.861;
    let result = calc_ground_factors(iq, slope, phi);
    assert_abs_diff_eq!(result.gc, 0.814, epsilon = 1e-3);
    assert_abs_diff_eq!(result.gq, 0.833, epsilon = 1e-3);
    assert_abs_diff_eq!(result.gg, 0.833, epsilon = 1e-3);
}
