use approx::assert_abs_diff_eq;
use soilrust::{
    local_soil_class::by_cu::calc_lsc_by_cu,
    models::soil_profile::{SoilLayer, SoilProfile},
};

fn create_layer(thickness: f64, cu: f64) -> SoilLayer {
    SoilLayer {
        thickness,
        cu: Some(cu),
        ..Default::default()
    }
}

/// Case 1: All cu > 0 & depth < 30
#[test]
fn test_case_1() {
    let profile = SoilProfile {
        ground_water_level: 0.0,
        layers: vec![create_layer(5.0, 10.0), create_layer(10.0, 15.0)], // total depth = 15
    };

    let result = calc_lsc_by_cu(&mut profile.clone());
    assert_eq!(result.layers.len(), 2);
    assert_abs_diff_eq!(result.cu_30, 12.86, epsilon = 1e-2); // harmonic average
    assert_eq!(result.soil_class, "ZD"); // low cu_30 leads to ZD
}

/// Case 2: One cu = 0 & depth = 30
#[test]
fn test_case_2() {
    let profile = SoilProfile {
        ground_water_level: 0.0,
        layers: vec![
            create_layer(10.0, 15.0),
            create_layer(10.0, 0.0), // should be skipped
            create_layer(10.0, 30.0),
        ],
    };

    let result = calc_lsc_by_cu(&mut profile.clone());

    assert_eq!(result.layers.len(), 2);
    assert_eq!(result.cu_30, 30.);
    assert_eq!(result.soil_class, "ZC"); // low cu_30 leads to ZE
}

#[test]
fn test_case_3() {
    let profile = SoilProfile {
        ground_water_level: 0.0,
        layers: vec![
            create_layer(10.0, 10.0),
            create_layer(10.0, 20.0),
            create_layer(20.0, 40.0), // only 10 m of this will be used
        ],
    };

    let result = calc_lsc_by_cu(&mut profile.clone());

    assert_eq!(result.layers.len(), 3);
    assert_abs_diff_eq!(result.cu_30, 17.14, epsilon = 1e-2); // harmonic average
    assert_eq!(result.soil_class, "ZD");
}
