use approx::assert_abs_diff_eq;
use soilrust::{
    elastic_settlement::boussinesq::*,
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
                elastic_modulus: Some(1500.),
                poissons_ratio: Some(0.4),
                depth: Some(3.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 5.0,
                dry_unit_weight: Some(1.9),
                saturated_unit_weight: Some(2.),
                elastic_modulus: Some(6000.),
                poissons_ratio: Some(0.4),
                depth: Some(8.0),
                ..Default::default()
            },
            SoilLayer {
                thickness: 50.0,
                dry_unit_weight: Some(2.),
                saturated_unit_weight: Some(2.1),
                elastic_modulus: Some(7500.),
                poissons_ratio: Some(0.4),
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
fn test_calc_ip() {
    let h = 5.0;
    let b = 10.0;
    let l = 20.0;
    let u = 0.1;

    let result = calc_ip(h, b, l, u);
    let expected = 0.222;

    assert_abs_diff_eq!(result, expected, epsilon = 1e-3);
}

#[test]
fn test_calc_single_layer_settlement() {
    let h = 2.0;
    let u = 0.4;
    let e = 6000.0;
    let l = 20.0;
    let b = 10.0;
    let df = 6.0;
    let q_net = 88.3;

    let result = single_layer_settlement(h, u, e, l, b, df, q_net);
    let expected = 1.05;

    assert_abs_diff_eq!(result, expected, epsilon = 1e-3);
}

#[test]
fn test_calc_elastic_settlement() {
    let soil_profile = create_soil_profile();
    let foundation_data = create_foundation_data();
    let foundation_pressure = 50.;

    let settlements = calc_elastic_settlement(&soil_profile, &foundation_data, foundation_pressure);
    let expected_settlements = &[1.058, 2.195, 4.613];

    for (settlement, expected) in settlements.iter().zip(expected_settlements.iter()) {
        assert_abs_diff_eq!(settlement, expected, epsilon = 1e-3);
    }
}
