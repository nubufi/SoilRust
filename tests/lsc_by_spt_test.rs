use approx::assert_abs_diff_eq;
use soilrust::{
    local_soil_class::by_spt::calc_lsc_by_spt,
    models::spt::{NValue, SPTBlow, SPTExp},
};

fn create_blow(depth: f64, n60: i32) -> SPTBlow {
    SPTBlow {
        depth,
        n60: if n60 == 50 {
            Some(NValue::Refusal)
        } else {
            Some(NValue::from_i32(n60))
        },
        ..Default::default()
    }
}

/// Case 1: All spt > 0 & no refusal & depth < 30
#[test]
fn test_case_1() {
    let exp = SPTExp {
        name: "Test exp".to_string(),
        blows: vec![
            create_blow(5.0, 10),
            create_blow(10.0, 15),
            create_blow(15.0, 20),
        ], // total depth = 15
    };

    let result = calc_lsc_by_spt(&mut exp.clone());
    assert_eq!(result.layers.len(), 3);
    assert_abs_diff_eq!(result.n_30, 13.84, epsilon = 1e-2); // harmonic average
    assert_eq!(result.soil_class, "ZE");
}

/// Case 2: One spt = R & depth = 30
#[test]
fn test_case_2() {
    let exp = SPTExp {
        name: "Test Exp".to_string(),
        blows: vec![
            create_blow(10.0, 15),
            create_blow(20.0, 50),
            create_blow(30.0, 30),
        ],
    };

    let result = calc_lsc_by_spt(&mut exp.clone());

    assert_eq!(result.layers.len(), 3);
    assert_eq!(result.n_30, 25.);
    assert_eq!(result.soil_class, "ZD"); // low vs_30 leads to ZE
}

/// Case 3: All spt > 0 & no refusal & depth > 30
#[test]
fn test_case_3() {
    let exp = SPTExp {
        name: "Test Exp".to_string(),
        blows: vec![
            create_blow(10.0, 10),
            create_blow(20.0, 20),
            create_blow(40.0, 40), // only 10 m of this will be used
        ],
    };

    let result = calc_lsc_by_spt(&mut exp.clone());

    assert_eq!(result.layers.len(), 3);
    assert_abs_diff_eq!(result.n_30, 17.14, epsilon = 1e-2); // harmonic average
    assert_eq!(result.soil_class, "ZD");
}
