use soilrust::{
    bearing_capacity::{model::BearingCapacityFactors, vesic::*},
    models::{foundation::Foundation, loads::Loads},
};

/// Case 1: φ = 0°, pure cohesive soil — should return Nc = 5.14, Nq = 1.0, Ng = 0.0
#[test]
fn test_calc_bearing_capacity_factors_1() {
    let phi = 0.0;
    let expected = (5.14, 1.0, 0.0);
    let result = calc_bearing_capacity_factors(phi);

    assert!((result.nc - expected.0).abs() < 1e-3);
    assert!((result.nq - expected.1).abs() < 1e-3);
    assert!((result.ng - expected.2).abs() < 1e-3);
}

/// Case 2: φ = 10° — soft granular soil
#[test]
fn test_calc_bearing_capacity_factors_2() {
    let phi = 10.0;
    let expected = (8.345, 2.471, 0.519);
    let result = calc_bearing_capacity_factors(phi);

    assert!((result.nc - expected.0).abs() < 1e-3);
    assert!((result.nq - expected.1).abs() < 1e-3);
    assert!((result.ng - expected.2).abs() < 1e-3);
}

/// Case 3: φ = 30° — typical for dense sand
#[test]
fn test_calc_bearing_capacity_factors_3() {
    let phi = 30.0;
    let expected = (30.140, 18.401, 20.093);
    let result = calc_bearing_capacity_factors(phi);

    assert!((result.nc - expected.0).abs() < 1e-3);
    assert!((result.nq - expected.1).abs() < 1e-3);
    assert!((result.ng - expected.2).abs() < 1e-3);
}

/// Case 4: φ = 45° — extremely dense or crushed rock fill
#[test]
fn test_calc_bearing_capacity_factors_4() {
    let phi = 45.0;
    let expected = (133.874, 134.874, 267.748);
    let result = calc_bearing_capacity_factors(phi);

    assert!((result.nc - expected.0).abs() < 1e-3);
    assert!((result.nq - expected.1).abs() < 1e-3);
    assert!((result.ng - expected.2).abs() < 1e-3);
}
// --------------------------------------------------------------
// Case 1: φ = 30°, B/L = 2/4 = 0.5, Nq = 18.4, Nc = 30.1
#[test]
fn test_calc_shape_factors_1() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        effective_width: Some(2.0),
        effective_length: Some(4.0),
        ..Foundation::default()
    };

    let bc_factors = BearingCapacityFactors {
        nc: 30.1,
        nq: 18.4,
        ng: 0.0,
    };

    let result = calc_shape_factors(&foundation, bc_factors, 30.0);
    assert!((result.sc - 1.306).abs() < 1e-3);
    assert!((result.sq - 1.289).abs() < 1e-3);
    assert!((result.sg - 0.8).abs() < 1e-3);
}

/// Case 2: φ = 20°, B/L = 3/6 = 0.5, Nq = 10.0, Nc = 20.0
#[test]
fn test_calc_shape_factors_2() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        effective_width: Some(3.0),
        effective_length: Some(6.0),
        ..Foundation::default()
    };

    let bc_factors = BearingCapacityFactors {
        nc: 20.0,
        nq: 10.0,
        ng: 0.0,
    };

    let result = calc_shape_factors(&foundation, bc_factors, 20.0);
    assert!((result.sc - 1.25).abs() < 1e-3);
    assert!((result.sq - 1.182).abs() < 1e-3);
    assert!((result.sg - 0.8).abs() < 1e-3);
}

/// Case 3: φ = 45°, square footing (B = L = 5), should limit Sg to 0.6
#[test]
fn test_calc_shape_factors_3() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        effective_width: Some(5.0),
        effective_length: Some(5.0),
        ..Foundation::default()
    };

    let bc_factors = BearingCapacityFactors {
        nc: 40.0,
        nq: 25.0,
        ng: 0.0,
    };

    let result = calc_shape_factors(&foundation, bc_factors, 45.0);
    assert!((result.sc - 1.625).abs() < 1e-3);
    assert!((result.sq - 2.0).abs() < 1e-3);
    assert!((result.sg - 0.6).abs() < 1e-3);
}

/// Case 4: φ = 35°, B/L = 4/10 = 0.4
#[test]
fn test_calc_shape_factors_4() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        effective_width: Some(4.0),
        effective_length: Some(10.0),
        ..Foundation::default()
    };

    let bc_factors = BearingCapacityFactors {
        nc: 25.0,
        nq: 15.0,
        ng: 0.0,
    };

    let result = calc_shape_factors(&foundation, bc_factors, 35.0);
    assert!((result.sc - 1.24).abs() < 1e-3);
    assert!((result.sq - 1.28).abs() < 1e-3);
    assert!((result.sg - 0.84).abs() < 1e-3);
}
// --------------------------------------------------------------
/// Case 1: φ = 0°, cohesive soil, inclined load — ic < 1, iq = ig = 1
#[test]
fn test_calc_inclination_factors_1() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 4.0,
        foundation_length: 6.0,
        foundation_angle: Some(10.0),
        effective_width: Some(4.0),
        effective_length: Some(6.0),
        ..Default::default()
    };

    let loads = Loads {
        vertical_load: Some(200.0),
        horizontal_load_x: Some(15.0),
        horizontal_load_y: Some(20.0),
        ..Default::default()
    };

    let result = calc_inclination_factors(0.0, 25.0, &foundation, &loads);
    assert!((result.ic - 0.986).abs() < 1e-3);
    assert!((result.iq - 1.0).abs() < 1e-3);
    assert!((result.ig - 1.0).abs() < 1e-3);
}

/// Case 2: φ = 30°, frictional soil, moderate load inclination
#[test]
fn test_calc_inclination_factors_2() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 5.0,
        foundation_length: 10.0,
        foundation_angle: Some(15.0),
        effective_width: Some(5.0),
        effective_length: Some(10.0),
        ..Default::default()
    };

    let loads = Loads {
        vertical_load: Some(300.0),
        horizontal_load_x: Some(20.0),
        horizontal_load_y: Some(25.0),
        ..Default::default()
    };

    let result = calc_inclination_factors(30.0, 30.0, &foundation, &loads);
    assert!((result.ic - 0.980).abs() < 1e-3);
    assert!((result.iq - 0.982).abs() < 1e-3);
    assert!((result.ig - 0.971).abs() < 1e-3);
}

/// Case 3: φ = 45°, steep inclination and high H/V
#[test]
fn test_calc_inclination_factors_3() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 3.0,
        foundation_length: 8.0,
        foundation_angle: Some(20.0),
        effective_width: Some(3.0),
        effective_length: Some(8.0),
        ..Default::default()
    };

    let loads = Loads {
        vertical_load: Some(250.0),
        horizontal_load_x: Some(30.0),
        horizontal_load_y: Some(35.0),
        ..Default::default()
    };

    let result = calc_inclination_factors(45.0, 20.0, &foundation, &loads);
    assert!((result.ic - 0.902).abs() < 1e-3);
    assert!((result.iq - 0.903).abs() < 1e-3);
    assert!((result.ig - 0.851).abs() < 1e-3);
}

/// Case 4: φ = 0°, base angle = 0 → all inclination factors = 1
#[test]
fn test_calc_inclination_factors_4() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 3.0,
        foundation_length: 6.0,
        foundation_angle: Some(0.0),
        effective_width: Some(3.0),
        effective_length: Some(6.0),
        ..Default::default()
    };

    let loads = Loads {
        vertical_load: Some(100.0),
        horizontal_load_x: Some(5.0),
        horizontal_load_y: Some(5.0),
        ..Default::default()
    };

    let result = calc_inclination_factors(0.0, 10.0, &foundation, &loads);
    assert!((result.ic - 1.0).abs() < 1e-3);
    assert!((result.iq - 1.0).abs() < 1e-3);
    assert!((result.ig - 1.0).abs() < 1e-3);
}
// --------------------------------------------------------------
/// Case 1: φ = 0°, slope = 10°, base = 5° → bc < 1, bq = bg = 1
#[test]
fn test_calc_base_factors_1() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        foundation_length: 2.0,
        foundation_angle: Some(5.0),
        slope_angle: Some(10.0),
        ..Default::default()
    };

    let result = calc_base_factors(0.0, &foundation);

    assert!((result.bc - 0.966).abs() < 1e-3);
    assert!((result.bq - 1.0).abs() < 1e-3);
    assert!((result.bg - 1.0).abs() < 1e-3);
}

/// Case 2: φ = 30°, slope = 10°, base = 5°
#[test]
fn test_calc_base_factors_2() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        foundation_length: 2.0,
        foundation_angle: Some(5.0),
        slope_angle: Some(10.0),
        ..Default::default()
    };

    let result = calc_base_factors(30.0, &foundation);

    assert!((result.bc - 0.882).abs() < 1e-3);
    assert!((result.bq - 0.902).abs() < 1e-3);
    assert!((result.bg - 0.902).abs() < 1e-3);
}

/// Case 3: φ = 45°, slope = 20°, base = 10°
#[test]
fn test_calc_base_factors_3() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        foundation_length: 2.0,
        foundation_angle: Some(10.0),
        slope_angle: Some(20.0),
        ..Default::default()
    };

    let result = calc_base_factors(45.0, &foundation);

    assert!((result.bc - 0.864).abs() < 1e-3);
    assert!((result.bq - 0.681).abs() < 1e-3);
    assert!((result.bg - 0.681).abs() < 1e-3);
}

/// Case 4: φ = 15°, slope = 5°, base = 2°
#[test]
fn test_calc_base_factors_4() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        foundation_length: 2.0,
        foundation_angle: Some(2.0),
        slope_angle: Some(5.0),
        ..Default::default()
    };

    let result = calc_base_factors(15.0, &foundation);

    assert!((result.bc - 0.873).abs() < 1e-3);
    assert!((result.bq - 0.981).abs() < 1e-3);
    assert!((result.bg - 0.981).abs() < 1e-3);
}
// --------------------------------------------------------------
/// Case 1: φ = 0°, iq = 1.0, slope = 10° → basic cohesive slope condition
#[test]
fn test_calc_ground_factors_1() {
    let result = calc_ground_factors(1.0, 10.0, 0.0);
    assert!((result.gc - 0.966).abs() < 1e-3);
    assert!((result.gq - 0.678).abs() < 1e-3);
    assert!((result.gg - 0.678).abs() < 1e-3);
}

/// Case 2: φ = 30°, iq = 0.9, slope = 15°
#[test]
fn test_calc_ground_factors_2() {
    let result = calc_ground_factors(0.9, 15.0, 30.0);
    assert!((result.gc - 0.866).abs() < 1e-3);
    assert!((result.gq - 0.536).abs() < 1e-3);
    assert!((result.gg - 0.536).abs() < 1e-3);
}

/// Case 3: φ = 45°, iq = 0.8, slope = 20°
#[test]
fn test_calc_ground_factors_3() {
    let result = calc_ground_factors(0.8, 20.0, 45.0);
    assert!((result.gc - 0.7611).abs() < 1e-3);
    assert!((result.gq - 0.4045).abs() < 1e-3);
    assert!((result.gg - 0.4045).abs() < 1e-3);
}

/// Case 4: φ = 15°, iq = 0.95, slope = 5°
#[test]
fn test_calc_ground_factors_4() {
    let result = calc_ground_factors(0.95, 5.0, 15.0);
    assert!((result.gc - 0.914).abs() < 1e-3);
    assert!((result.gq - 0.833).abs() < 1e-3);
    assert!((result.gg - 0.833).abs() < 1e-3);
}
// --------------------------------------------------------------
/// Case 1: φ = 0°, Df/B = 0.5 → dq = 1.0, dc = 1.2, dg = 1.0
#[test]
fn test_calc_depth_factors_1() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        effective_width: Some(2.0),
        ..Default::default()
    };

    let result = calc_depth_factors(&foundation, 0.0);
    assert!((result.dc - 1.2).abs() < 1e-3);
    assert!((result.dq - 1.0).abs() < 1e-3);
    assert!((result.dg - 1.0).abs() < 1e-3);
}

/// Case 2: φ = 30°, Df/B = 0.5 → tan(φ) effect increases dq
#[test]
fn test_calc_depth_factors_2() {
    let foundation = Foundation {
        foundation_depth: 1.0,
        foundation_width: 2.0,
        effective_width: Some(2.0),
        ..Default::default()
    };

    let result = calc_depth_factors(&foundation, 30.0);
    assert!((result.dc - 1.2).abs() < 1e-3);
    assert!((result.dq - 1.144).abs() < 1e-3);
    assert!((result.dg - 1.0).abs() < 1e-3);
}

/// Case 3: φ = 45°, Df/B = 1.5 → db = atan(Df/B) is used
#[test]
fn test_calc_depth_factors_3() {
    let foundation = Foundation {
        foundation_depth: 3.0,
        foundation_width: 2.0,
        effective_width: Some(2.0),
        ..Default::default()
    };

    let result = calc_depth_factors(&foundation, 45.0);
    assert!((result.dc - 1.393).abs() < 1e-3);
    assert!((result.dq - 1.169).abs() < 1e-3);
    assert!((result.dg - 1.0).abs() < 1e-3);
}

/// Case 4: φ = 15°, Df = 0 → depth factors default to 1.0
#[test]
fn test_calc_depth_factors_4() {
    let foundation = Foundation {
        foundation_depth: 0.0,
        foundation_width: 2.0,
        effective_width: Some(2.0),
        ..Default::default()
    };

    let result = calc_depth_factors(&foundation, 15.0);
    assert!((result.dc - 1.0).abs() < 1e-3);
    assert!((result.dq - 1.0).abs() < 1e-3);
    assert!((result.dg - 1.0).abs() < 1e-3);
}
// --------------------------------------------------------------
