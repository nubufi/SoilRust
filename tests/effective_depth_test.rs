use approx::assert_abs_diff_eq;
use soilrust::{
    effective_depth::calc_effective_depth,
    models::{
        foundation::Foundation,
        soil_profile::{SoilLayer, SoilProfile},
    },
};

fn create_soil_profile() -> SoilProfile {
    SoilProfile {
        ground_water_level: 5.,
        layers: vec![
            SoilLayer {
                thickness: 3.0,
                dry_unit_weight: Some(1.8),
                saturated_unit_weight: Some(1.9),
                depth: Some(3.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 5.0,
                dry_unit_weight: Some(1.9),
                saturated_unit_weight: Some(2.),
                depth: Some(8.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 50.0,
                dry_unit_weight: Some(2.),
                saturated_unit_weight: Some(2.1),
                depth: Some(58.0),
                ..Default::default()
            },
        ],
    }
}
fn create_foundation_data() -> Foundation {
    Foundation {
        foundation_width: 10.0,
        foundation_length: 20.0,
        foundation_depth: 2.0,
        ..Default::default()
    }
}
#[test]
fn test_effective_depth() {
    let soil_profile = create_soil_profile();
    let foundation_data = create_foundation_data();
    let foundation_pressure = 50.;

    let effective_depth =
        calc_effective_depth(&soil_profile, &foundation_data, foundation_pressure);
    let expected_depth = 34.41;
    assert_abs_diff_eq!(effective_depth, expected_depth, epsilon = 1e-2);
}
