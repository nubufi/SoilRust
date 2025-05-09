use approx::assert_abs_diff_eq;
use soilrust::{
    enums::SelectionMethod,
    models::{
        masw::{Masw, MaswExp, MaswLayer},
        soil_profile::{SoilLayer, SoilProfile},
    },
};

fn create_soil_profile() -> SoilProfile {
    SoilProfile {
        ground_water_level: Some(0.),
        layers: vec![SoilLayer {
            thickness: Some(5.0),
            dry_unit_weight: Some(1.8),
            saturated_unit_weight: Some(2.0),
            depth: Some(5.0),
            ..Default::default()
        }],
    }
}

fn create_masw_exp(vs: f64) -> Masw {
    Masw {
        exps: vec![MaswExp {
            layers: vec![MaswLayer {
                thickness: Some(5.0),
                depth: Some(5.0),
                vs: Some(vs),
                vp: Some(0.0),
            }],
            name: "Test".to_string(),
        }],
        idealization_method: SelectionMethod::Min,
    }
}

// Test for VS >= 4000
#[test]
fn test_bc_tezcan_ozdemir_1() {
    let soil_profile: SoilProfile = create_soil_profile();
    let mut masw_exp = create_masw_exp(4001.0);
    let foundation = soilrust::models::foundation::Foundation::new(
        Some(5.0),
        Some(1.0),
        Some(1.0),
        None,
        None,
        None,
        None,
    );

    let foundation_pressure = 100.0;

    let result = soilrust::bearing_capacity::tezcan_ozdemir::calc_bearing_capacity(
        soil_profile,
        &mut masw_exp,
        foundation,
        foundation_pressure,
    )
    .unwrap();

    assert!(result.is_safe);
    assert_abs_diff_eq!(result.allowable_bearing_capacity, 568.142, epsilon = 1e-5);
    assert_abs_diff_eq!(result.safety_factor, 1.4, epsilon = 1e-5);
}

// Test for VS = 3000
#[test]
fn test_bc_tezcan_ozdemir_2() {
    let soil_profile: SoilProfile = create_soil_profile();
    let mut masw_exp = create_masw_exp(3000.0);
    let foundation = soilrust::models::foundation::Foundation::new(
        Some(5.0),
        Some(1.0),
        Some(1.0),
        None,
        None,
        None,
        None,
    );

    let foundation_pressure = 100.0;

    let result = soilrust::bearing_capacity::tezcan_ozdemir::calc_bearing_capacity(
        soil_profile,
        &mut masw_exp,
        foundation,
        foundation_pressure,
    )
    .unwrap();

    assert!(result.is_safe);
    assert_abs_diff_eq!(result.allowable_bearing_capacity, 272.72727, epsilon = 1e-5);
    assert_abs_diff_eq!(result.safety_factor, 2.2, epsilon = 1e-5);
}

// Test for VS = 400
#[test]
fn test_bc_tezcan_ozdemir_3() {
    let soil_profile: SoilProfile = create_soil_profile();
    let mut masw_exp = create_masw_exp(400.0);
    let foundation = soilrust::models::foundation::Foundation::new(
        Some(5.0),
        Some(1.0),
        Some(1.0),
        None,
        None,
        None,
        None,
    );

    let foundation_pressure = 100.0;

    let result = soilrust::bearing_capacity::tezcan_ozdemir::calc_bearing_capacity(
        soil_profile,
        &mut masw_exp,
        foundation,
        foundation_pressure,
    )
    .unwrap();

    assert!(!result.is_safe);
    assert_abs_diff_eq!(result.allowable_bearing_capacity, 20., epsilon = 1e-5);
    assert_abs_diff_eq!(result.safety_factor, 4., epsilon = 1e-5);
}
