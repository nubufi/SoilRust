use soilrust::models::soil_profile::{SoilLayer, SoilProfile};

/// Creates a reusable soil profile for testing.
pub fn setup_soil_profile() -> SoilProfile {
    SoilProfile::new(
        vec![
            SoilLayer {
                thickness: 2.0,
                dry_unit_weight: Some(1.8),
                saturated_unit_weight: Some(2.0),
                soil_class: "Clay".to_string(),
                ..Default::default()
            },
            SoilLayer {
                thickness: 3.0,
                dry_unit_weight: Some(1.6),
                saturated_unit_weight: Some(1.9),
                soil_class: "Sand".to_string(),
                ..Default::default()
            },
        ],
        2.5, // Groundwater level at 2.5m
    )
}

#[test]
fn test_calc_layer_depths() {
    let profile = setup_soil_profile();
    assert_eq!(profile.layers[0].depth.unwrap(), 2.0);
    assert_eq!(profile.layers[1].depth.unwrap(), 5.0);
    assert_eq!(profile.layers[0].center.unwrap(), 1.0);
    assert_eq!(profile.layers[1].center.unwrap(), 3.5);
}

#[test]
fn test_get_layer_index() {
    let profile = setup_soil_profile();
    assert_eq!(profile.get_layer_index(1.0), 0);
    assert_eq!(profile.get_layer_index(3.0), 1);
    assert_eq!(profile.get_layer_index(5.0), 1);
}

#[test]
fn test_calc_normal_stress() {
    let profile = setup_soil_profile();

    assert!((profile.calc_normal_stress(1.0) - 1.8).abs() < 1e-3);
    assert!((profile.calc_normal_stress(2.0) - 3.6).abs() < 1e-3);
    assert!((profile.calc_normal_stress(3.0) - 5.35).abs() < 1e-3);
}

#[test]
fn test_calc_effective_stress() {
    let profile = setup_soil_profile();

    assert!((profile.calc_effective_stress(1.0) - 1.8).abs() < 1e-3);
    assert!((profile.calc_effective_stress(2.0) - 3.6).abs() < 1e-3);
    assert!((profile.calc_effective_stress(3.0) - 4.8595).abs() < 1e-3);
}
