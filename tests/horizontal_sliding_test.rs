use approx::assert_abs_diff_eq;
use soilrust::{
    horizontal_sliding::calc_horizontal_sliding,
    models::{
        foundation::Foundation,
        loads::Loads,
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
                c_prime: Some(1.),
                phi_prime: Some(21.),
                phi_u: Some(0.),
                cu: Some(3.),
                depth: Some(3.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 5.0,
                dry_unit_weight: Some(1.9),
                saturated_unit_weight: Some(2.),
                c_prime: Some(0.5),
                phi_prime: Some(28.),
                phi_u: Some(20.),
                cu: Some(0.),
                depth: Some(8.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 50.0,
                dry_unit_weight: Some(2.),
                saturated_unit_weight: Some(2.1),
                c_prime: Some(1.),
                phi_prime: Some(24.),
                phi_u: Some(0.),
                cu: Some(5.),
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
        surface_friction_coefficient: Some(0.6),
        ..Default::default()
    }
}

fn create_load_data() -> Loads {
    Loads {
        horizontal_load_x: Some(10.0),
        horizontal_load_y: Some(20.0),
        ..Default::default()
    }
}

#[test]
fn test_horizontal_sliding() {
    let soil_profile = create_soil_profile();
    let foundation_data = create_foundation_data();
    let load_data = create_load_data();
    let foundation_pressure = 50.;

    let result = calc_horizontal_sliding(
        &soil_profile,
        &foundation_data,
        &load_data,
        foundation_pressure,
    );
    assert_abs_diff_eq!(result.rth, 5454.55, epsilon = 1e-2);
    assert_abs_diff_eq!(result.rpk_x, 76.21, epsilon = 1e-2);
    assert_abs_diff_eq!(result.rpk_y, 152.43, epsilon = 1e-2);
    assert_abs_diff_eq!(result.rpt_x, 54.44, epsilon = 1e-2);
    assert_abs_diff_eq!(result.rpt_y, 108.88, epsilon = 1e-2);
    assert_abs_diff_eq!(result.sum_x, 5470.88, epsilon = 1e-2);
    assert_abs_diff_eq!(result.sum_y, 5487.21, epsilon = 1e-2);
}
