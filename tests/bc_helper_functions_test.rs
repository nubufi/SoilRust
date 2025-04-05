use approx::assert_abs_diff_eq;
use soilrust::bearing_capacity::helper_functions::*;
use soilrust::enums::AnalysisTerm;
use soilrust::models::foundation::Foundation;
use soilrust::models::soil_profile::{SoilLayer, SoilProfile};
// ------------------------------------------------------------------------------------------------
// Test for single layer
#[test]
fn test_compute_equivalent_unit_weights_1() {
    let profile = SoilProfile {
        ground_water_level: 0.,
        layers: vec![SoilLayer {
            thickness: 5.0,
            dry_unit_weight: Some(1.8),
            saturated_unit_weight: Some(2.0),
            depth: Some(5.0),
            ..Default::default()
        }],
    };
    let (gamma_1, gamma_2) = compute_equivalent_unit_weights(&profile, 5.0);
    assert_abs_diff_eq!(gamma_1, 1.8, epsilon = 1e-3);
    assert_abs_diff_eq!(gamma_2, 2.0, epsilon = 1e-3);
}

// Test for 2 layers
#[test]
fn test_compute_equivalent_unit_weights_2() {
    let profile = SoilProfile {
        ground_water_level: 0.0,
        layers: vec![
            SoilLayer {
                thickness: 3.0,
                dry_unit_weight: Some(1.7),
                saturated_unit_weight: Some(1.9),
                depth: Some(3.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 2.0,
                dry_unit_weight: Some(1.9),
                saturated_unit_weight: Some(2.1),
                depth: Some(5.0),
                ..Default::default()
            },
        ],
    };
    let (gamma_1, gamma_2) = compute_equivalent_unit_weights(&profile, 5.0);
    assert!((gamma_1 - 1.78).abs() < 1e-3);
    assert!((gamma_2 - 1.98).abs() < 1e-3);
}

// Test for 3 layers
#[test]
fn test_compute_equivalent_unit_weights_3() {
    let profile = SoilProfile {
        ground_water_level: 0.0,
        layers: vec![
            SoilLayer {
                thickness: 2.0,
                dry_unit_weight: Some(1.6),
                saturated_unit_weight: Some(1.8),
                depth: Some(2.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 3.0,
                dry_unit_weight: Some(1.8),
                saturated_unit_weight: Some(2.0),
                depth: Some(5.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 4.0,
                dry_unit_weight: Some(2.0),
                saturated_unit_weight: Some(2.2),
                depth: Some(9.0),
                ..Default::default()
            },
        ],
    };
    let (gamma_1, gamma_2) = compute_equivalent_unit_weights(&profile, 7.0);
    assert!((gamma_1 - 1.8).abs() < 1e-3);
    assert!((gamma_2 - 2.0).abs() < 1e-3);
}

// Test for depth limit at layer boundary
#[test]
fn test_compute_equivalent_unit_weights_4() {
    let profile = SoilProfile {
        ground_water_level: 0.0,
        layers: vec![
            SoilLayer {
                thickness: 3.0,
                dry_unit_weight: Some(1.7),
                saturated_unit_weight: Some(1.9),
                depth: Some(3.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 2.0,
                dry_unit_weight: Some(1.8),
                saturated_unit_weight: Some(2.0),
                depth: Some(5.0),
                ..Default::default()
            },
        ],
    };
    let (gamma_1, gamma_2) = compute_equivalent_unit_weights(&profile, 3.0);
    assert!((gamma_1 - 1.7).abs() < 1e-3);
    assert!((gamma_2 - 1.9).abs() < 1e-3);
}

// Test for depth limit inside layer
#[test]
fn test_compute_equivalent_unit_weights_5() {
    let profile = SoilProfile {
        ground_water_level: 0.0,
        layers: vec![
            SoilLayer {
                thickness: 3.0,
                dry_unit_weight: Some(1.7),
                saturated_unit_weight: Some(1.9),
                depth: Some(3.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 3.0,
                dry_unit_weight: Some(1.8),
                saturated_unit_weight: Some(2.0),
                depth: Some(6.0),
                ..Default::default()
            },
        ],
    };
    let (gamma_1, gamma_2) = compute_equivalent_unit_weights(&profile, 4.0);
    assert!((gamma_1 - 1.725).abs() < 1e-3);
    assert!((gamma_2 - 1.925).abs() < 1e-3);
}

// Test for depth limit outside profile
#[test]
fn test_compute_equivalent_unit_weights_6() {
    let profile = SoilProfile {
        ground_water_level: 0.0,
        layers: vec![
            SoilLayer {
                thickness: 3.0,
                dry_unit_weight: Some(1.7),
                saturated_unit_weight: Some(1.9),
                depth: Some(3.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 3.0,
                dry_unit_weight: Some(1.8),
                saturated_unit_weight: Some(2.0),
                depth: Some(6.0),
                ..Default::default()
            },
        ],
    };
    let (gamma_1, gamma_2) = compute_equivalent_unit_weights(&profile, 10.0);
    assert!((gamma_1 - 1.75).abs() < 1e-3);
    assert!((gamma_2 - 1.95).abs() < 1e-3);
}
// ------------------------------------------------------------------------------------------------
/// Case 1: Foundation above groundwater (gwt > Df + B)
#[test]
fn test_calc_effective_surcharge_1() {
    let profile = SoilProfile {
        ground_water_level: 10.0,
        layers: vec![SoilLayer {
            thickness: 5.0,
            dry_unit_weight: Some(1.8),
            saturated_unit_weight: Some(2.0),
            depth: Some(5.0),
            ..Default::default()
        }],
    };
    let building = Foundation {
        foundation_depth: 3.0,
        effective_width: Some(2.0),
        ..Default::default()
    };
    let pressure = calc_effective_surcharge(&profile, &building, AnalysisTerm::Short);
    assert!(
        (pressure - 5.4).abs() < 1e-3,
        "Expected 5.4, got {}",
        pressure
    );
}

/// Case 2: Foundation below groundwater (0 < gwt <= Df)
#[test]
fn test_calc_effective_surcharge_2() {
    let profile = SoilProfile {
        ground_water_level: 2.0,
        layers: vec![SoilLayer {
            thickness: 5.0,
            dry_unit_weight: Some(1.8),
            saturated_unit_weight: Some(2.0),
            depth: Some(5.0),
            ..Default::default()
        }],
    };
    let building = Foundation {
        foundation_depth: 5.0,
        effective_width: Some(2.0),
        ..Default::default()
    };
    let pressure = calc_effective_surcharge(&profile, &building, AnalysisTerm::Short);
    assert!(
        (pressure - 6.657).abs() < 1e-3,
        "Expected 6.657, got {}",
        pressure
    );
}

/// Case 3: Groundwater at surface (gwt = 0) with short term
#[test]
fn test_calc_effective_surcharge_3() {
    let profile = SoilProfile {
        ground_water_level: 0.0,
        layers: vec![SoilLayer {
            thickness: 5.0,
            dry_unit_weight: Some(1.8),
            saturated_unit_weight: Some(2.0),
            depth: Some(5.0),
            ..Default::default()
        }],
    };
    let building = Foundation {
        foundation_depth: 7.0,
        effective_width: Some(3.0),
        ..Default::default()
    };
    let pressure = calc_effective_surcharge(&profile, &building, AnalysisTerm::Short);
    assert!(
        (pressure - 7.133).abs() < 1e-3,
        "Expected 7.133, got {}",
        pressure
    );
}

/// Case 4: Groundwater at surface (gwt = 0) with long term
#[test]
fn test_calc_effective_surcharge_4() {
    let profile = SoilProfile {
        ground_water_level: 0.0,
        layers: vec![SoilLayer {
            thickness: 5.0,
            dry_unit_weight: Some(1.8),
            saturated_unit_weight: Some(2.0),
            depth: Some(5.0),
            ..Default::default()
        }],
    };
    let building = Foundation {
        foundation_depth: 7.0,
        effective_width: Some(3.0),
        ..Default::default()
    };
    let pressure = calc_effective_surcharge(&profile, &building, AnalysisTerm::Long);
    assert!(
        (pressure - 12.6).abs() < 1e-3,
        "Expected 12.6, got {}",
        pressure
    );
}
// ------------------------------------------------------------------------------------------------
/// Case 1: Entire foundation is below groundwater level (gwt <= Df)
#[test]
fn test_calc_effective_unit_weight_1() {
    let profile = SoilProfile {
        ground_water_level: 2.0,
        layers: vec![SoilLayer {
            thickness: 5.0,
            dry_unit_weight: Some(1.8),
            saturated_unit_weight: Some(2.0),
            depth: Some(5.0),
            ..Default::default()
        }],
    };

    let foundation = Foundation {
        foundation_depth: 5.0,
        effective_width: Some(2.0),
        ..Default::default()
    };

    let gamma = calc_effective_unit_weight(&profile, &foundation, AnalysisTerm::Short);
    assert!(
        (gamma - 1.019).abs() < 1e-3,
        "Expected 1.019, got {}",
        gamma
    );
}

/// Case 2: Groundwater is between Df and Df + B (partially submerged zone)
#[test]
fn test_calc_effective_unit_weight_2() {
    let profile = SoilProfile {
        ground_water_level: 6.0,
        layers: vec![SoilLayer {
            thickness: 4.0,
            dry_unit_weight: Some(1.7),
            saturated_unit_weight: Some(2.1),
            depth: Some(4.0),
            ..Default::default()
        }],
    };

    let foundation = Foundation {
        foundation_depth: 5.0,
        effective_width: Some(2.0),
        ..Default::default()
    };

    let gamma = calc_effective_unit_weight(&profile, &foundation, AnalysisTerm::Short);
    assert!(
        (gamma - 1.409).abs() < 1e-3,
        "Expected 1.409, got {}",
        gamma
    );
}

/// Case 3: Foundation and entire zone above groundwater (gwt > Df + B)
#[test]
fn test_calc_effective_unit_weight_3() {
    let profile = SoilProfile {
        ground_water_level: 10.0,
        layers: vec![SoilLayer {
            thickness: 4.0,
            dry_unit_weight: Some(1.9),
            saturated_unit_weight: Some(2.3),
            depth: Some(4.0),
            ..Default::default()
        }],
    };

    let foundation = Foundation {
        foundation_depth: 6.0,
        effective_width: Some(2.0),
        ..Default::default()
    };

    let gamma = calc_effective_unit_weight(&profile, &foundation, AnalysisTerm::Short);
    assert!((gamma - 1.9).abs() < 1e-3, "Expected 1.9, got {}", gamma);
}

/// Case 4: Short-term vs. Long-term — long-term makes gwt = Df + B
#[test]
fn test_calc_effective_unit_weight_4() {
    let profile = SoilProfile {
        ground_water_level: 3.0,
        layers: vec![SoilLayer {
            thickness: 4.0,
            dry_unit_weight: Some(1.7),
            saturated_unit_weight: Some(2.1),
            depth: Some(4.0),
            ..Default::default()
        }],
    };

    let foundation = Foundation {
        foundation_depth: 6.0,
        effective_width: Some(2.0),
        ..Default::default()
    };

    let gamma = calc_effective_unit_weight(&profile, &foundation, AnalysisTerm::Long);
    assert!((gamma - 1.7).abs() < 1e-3, "Expected 1.7, got {}", gamma);
}
// ------------------------------------------------------------------------------------------------
/// Case 1: Short-term loading — returns undrained cohesion and undrained friction angle
#[test]
fn test_get_soil_params_1() {
    let profile = SoilProfile {
        ground_water_level: 2.0,
        layers: vec![SoilLayer {
            thickness: 5.0,
            depth: Some(5.0),
            cu: Some(25.0),
            phi_u: Some(20.0),
            c_prime: Some(5.0),
            phi_prime: Some(30.0),
            dry_unit_weight: Some(1.8),
            saturated_unit_weight: Some(2.0),
            ..Default::default()
        }],
    };

    let foundation = Foundation {
        foundation_depth: 3.0,
        effective_width: Some(2.0),
        ..Default::default()
    };

    let params = get_soil_params(&profile, &foundation, AnalysisTerm::Short);

    assert_eq!(params.friction_angle, 20.0);
    assert_eq!(params.cohesion, 25.0);
    assert!(
        (params.unit_weight - 1.019).abs() < 1e-3,
        "Expected 1.019, got {}",
        params.unit_weight
    );
}

/// Case 2: Long-term loading — returns effective parameters
#[test]
fn test_get_soil_params_2() {
    let profile = SoilProfile {
        ground_water_level: 0.0,
        layers: vec![SoilLayer {
            thickness: 4.0,
            depth: Some(4.0),
            cu: Some(18.0),
            phi_u: Some(25.0),
            c_prime: Some(8.0),
            phi_prime: Some(32.0),
            dry_unit_weight: Some(1.9),
            saturated_unit_weight: Some(2.1),
            ..Default::default()
        }],
    };

    let foundation = Foundation {
        foundation_depth: 3.5,
        effective_width: Some(2.0),
        ..Default::default()
    };

    let params = get_soil_params(&profile, &foundation, AnalysisTerm::Long);

    assert_eq!(params.friction_angle, 32.0);
    assert_eq!(params.cohesion, 8.0);
    assert!(
        (params.unit_weight - 1.9).abs() < 1e-3,
        "Expected 1.9, got {}",
        params.unit_weight
    );
}
