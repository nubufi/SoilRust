use approx::assert_abs_diff_eq;
use soilrust::enums::SelectionMethod;
use soilrust::models::cpt::*;

#[test]
fn test_calc_friction_ratio_valid() {
    let mut layer = CPTLayer::new(1.0, 10.0, 0.5, Some(0.2));
    layer.calc_friction_ratio();
    assert!(layer.friction_ratio.is_some());
    let rf = layer.friction_ratio.unwrap();
    assert_abs_diff_eq!(rf, 5.0, epsilon = 1e-6); // 0.5 / 10.0 * 100 = 5.0%
}

#[test]
fn test_calc_friction_ratio_zero_cone_resistance() {
    let mut layer = CPTLayer::new(1.0, 0.0, 0.5, Some(0.2));
    layer.calc_friction_ratio();
    assert!(layer.friction_ratio.is_none());
}

fn create_test_layers() -> Vec<CPTLayer> {
    vec![
        CPTLayer::new(1.0, 10.0, 0.5, Some(0.2)),
        CPTLayer::new(2.0, 11.0, 0.6, Some(0.3)),
        CPTLayer::new(3.0, 12.0, 0.7, Some(0.4)),
    ]
}

#[test]
fn test_get_layer_at_exact_depth() {
    let layers = create_test_layers();
    let cpt = CPTExp::new(layers.clone(), "Test CPT".to_string());

    let layer = cpt.get_layer_at_depth(2.0);
    assert_eq!(layer.depth.unwrap(), 2.0);
}

#[test]
fn test_get_layer_at_intermediate_depth() {
    let layers = create_test_layers();
    let cpt = CPTExp::new(layers.clone(), "Test CPT".to_string());

    let layer = cpt.get_layer_at_depth(2.5);
    assert_eq!(layer.depth.unwrap(), 3.0);
}

#[test]
fn test_get_layer_at_depth_exceeds_all_layers() {
    let layers = create_test_layers();
    let cpt = CPTExp::new(layers.clone(), "Test CPT".to_string());

    let layer = cpt.get_layer_at_depth(5.0);
    assert_eq!(layer.depth.unwrap(), 3.0); // last layer
}

#[test]
#[should_panic]
fn test_get_layer_with_empty_layers_should_panic() {
    let cpt = CPTExp::new(vec![], "Empty CPT".to_string());
    cpt.get_layer_at_depth(1.0); // Should panic because unwrap on empty
}

fn create_test_cpt() -> CPT {
    let exp1 = CPTExp::new(
        vec![
            CPTLayer::new(1.5, 160.0, 390.0, None),
            CPTLayer::new(2., 170.0, 395.0, None),
            CPTLayer::new(3., 180.0, 400.0, None),
        ],
        "Exp1".into(),
    );

    let exp2 = CPTExp::new(
        vec![
            CPTLayer::new(1.5, 150.0, 380.0, None),
            CPTLayer::new(3.0, 160.0, 390.0, None),
            CPTLayer::new(5.5, 170.0, 395.0, None),
            CPTLayer::new(6.5, 180.0, 400.0, None),
        ],
        "Exp2".into(),
    );

    CPT::new(vec![exp1, exp2], SelectionMethod::Min)
}

#[test]
fn test_get_idealized_exp_min_mode() {
    let cpt = create_test_cpt();

    let ideal = cpt.get_idealized_exp("Ideal_Min".into());

    // Sanity checks
    assert_eq!(ideal.name, "Ideal_Min");

    // Should be based on union of depths: [1.5, 2.0, 3.0,5.5, 6.0]
    assert_eq!(ideal.layers.len(), 5);

    // Check first layer values
    let layer1 = &ideal.layers[0];
    assert_abs_diff_eq!(layer1.depth.unwrap(), 1.5, epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.cone_resistance.unwrap(), 150., epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.sleeve_friction.unwrap(), 380., epsilon = 1e-6);

    // Check last layer depth
    let last_layer = ideal.layers.last().unwrap();
    assert_abs_diff_eq!(last_layer.depth.unwrap(), 6.5, epsilon = 1e-6);
}

#[test]
fn test_get_idealized_exp_avg_mode() {
    let mut cpt = create_test_cpt();

    cpt.idealization_method = SelectionMethod::Avg;
    let ideal = cpt.get_idealized_exp("Ideal_Avg".into());

    // Sanity checks
    assert_eq!(ideal.name, "Ideal_Avg");

    // Should be based on union of depths: [1.5, 2.0, 3.0,5.5, 6.0]
    assert_eq!(ideal.layers.len(), 5);

    // Check first layer values
    let layer1 = &ideal.layers[0];
    assert_abs_diff_eq!(layer1.depth.unwrap(), 1.5, epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.cone_resistance.unwrap(), 155., epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.sleeve_friction.unwrap(), 385., epsilon = 1e-6);

    // Check last layer depth
    let last_layer = ideal.layers.last().unwrap();
    assert_abs_diff_eq!(last_layer.depth.unwrap(), 6.5, epsilon = 1e-6);
}

#[test]
fn test_get_idealized_exp_max_mode() {
    let mut cpt = create_test_cpt();

    cpt.idealization_method = SelectionMethod::Max;
    let ideal = cpt.get_idealized_exp("Ideal_Max".into());

    println!("{:?}", ideal);
    // Sanity checks
    assert_eq!(ideal.name, "Ideal_Max");

    // Should be based on union of depths: [1.5, 2.0, 3.0,5.5, 6.0]
    assert_eq!(ideal.layers.len(), 5);

    // Check first layer values
    let layer1 = &ideal.layers[0];
    assert_abs_diff_eq!(layer1.depth.unwrap(), 1.5, epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.cone_resistance.unwrap(), 160., epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.sleeve_friction.unwrap(), 390., epsilon = 1e-6);

    // Check last layer depth
    let last_layer = ideal.layers.last().unwrap();
    assert_abs_diff_eq!(last_layer.depth.unwrap(), 6.5, epsilon = 1e-6);
}
