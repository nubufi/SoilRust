use approx::assert_abs_diff_eq;
use soilrust::{
    enums::SelectionMethod,
    local_soil_class::by_vs::calc_lsc_by_vs,
    models::masw::{Masw, MaswExp, MaswLayer},
};

fn create_layer(thickness: f64, vs: f64) -> MaswLayer {
    MaswLayer {
        thickness,
        vs,
        vp: 0.0,
        depth: None,
    }
}

/// Case 1: All vs > 0 & depth < 30
#[test]
fn test_case_1() {
    let exp = MaswExp {
        name: "Test exp".to_string(),
        layers: vec![create_layer(5.0, 1000.0), create_layer(10.0, 1500.0)], // total depth = 15
    };

    let mut masw = Masw {
        exps: vec![exp],
        idealization_method: SelectionMethod::Min,
    };

    let result = calc_lsc_by_vs(&mut masw);
    assert_eq!(result.layers.len(), 2);
    assert_abs_diff_eq!(result.vs_30, 1285.71, epsilon = 1e-2); // harmonic average
    assert_eq!(result.soil_class, "ZB"); // low vs_30 leads to ZB
}

/// Case 2: One vs = 0 & depth = 30
#[test]
fn test_case_2() {
    let exp = MaswExp {
        name: "Test Exp".to_string(),
        layers: vec![
            create_layer(10.0, 1500.0),
            create_layer(10.0, 0.0), // should be skipped
            create_layer(10.0, 3000.0),
        ],
    };

    let mut masw = Masw {
        exps: vec![exp],
        idealization_method: SelectionMethod::Min,
    };

    let result = calc_lsc_by_vs(&mut masw);

    assert_eq!(result.layers.len(), 2);
    assert_eq!(result.vs_30, 3000.);
    assert_eq!(result.soil_class, "ZA"); // low vs_30 leads to ZE
}

/// Case 3: All vs > 0 & depth > 30
#[test]
fn test_case_3() {
    let exp = MaswExp {
        name: "Test Exp".to_string(),
        layers: vec![
            create_layer(10.0, 1000.0),
            create_layer(10.0, 2000.0),
            create_layer(20.0, 4000.0), // only 10 m of this will be used
        ],
    };

    let mut masw = Masw {
        exps: vec![exp],
        idealization_method: SelectionMethod::Min,
    };

    let result = calc_lsc_by_vs(&mut masw);

    assert_eq!(result.layers.len(), 3);
    assert_abs_diff_eq!(result.vs_30, 1714.28, epsilon = 1e-2); // harmonic average
    assert_eq!(result.soil_class, "ZA");
}
