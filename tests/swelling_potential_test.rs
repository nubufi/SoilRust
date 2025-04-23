use approx::assert_abs_diff_eq;
use soilrust::{
    models::{
        foundation::Foundation,
        soil_profile::{SoilLayer, SoilProfile},
    },
    swelling_potential::calc_swelling_potential,
};

fn create_soil_profile() -> SoilProfile {
    SoilProfile {
        ground_water_level: Some(5.),
        layers: vec![
            SoilLayer {
                thickness: Some(3.0),
                dry_unit_weight: Some(1.8),
                saturated_unit_weight: Some(1.9),
                depth: Some(3.0),
                liquid_limit: Some(43.9),
                plastic_limit: Some(21.3),
                water_content: Some(23.7),
                ..Default::default()
            },
            SoilLayer {
                thickness: Some(5.0),
                dry_unit_weight: Some(1.9),
                saturated_unit_weight: Some(2.),
                depth: Some(8.0),
                liquid_limit: Some(58.85),
                plastic_limit: Some(37.4),
                water_content: Some(75.4),
                ..Default::default()
            },
            SoilLayer {
                thickness: Some(50.0),
                dry_unit_weight: Some(2.),
                saturated_unit_weight: Some(2.1),
                depth: Some(58.0),
                liquid_limit: Some(2.3),
                plastic_limit: Some(0.),
                water_content: Some(22.5),
                ..Default::default()
            },
        ],
    }
}
fn create_foundation_data() -> Foundation {
    Foundation {
        foundation_width: Some(10.0),
        foundation_length: Some(20.0),
        foundation_depth: Some(2.0),
        ..Default::default()
    }
}

#[test]
fn test_calc_swelling_potential() {
    let mut soil_profile = create_soil_profile();
    let foundation_data = create_foundation_data();
    let foundation_pressure = 50.;

    let result =
        calc_swelling_potential(&mut soil_profile, &foundation_data, foundation_pressure).unwrap();
    let expected_pressure = 8.89;
    assert_abs_diff_eq!(
        result.data[0].swelling_pressure,
        expected_pressure,
        epsilon = 0.01
    );
}
