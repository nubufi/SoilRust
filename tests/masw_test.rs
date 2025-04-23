use soilrust::{
    enums::SelectionMethod,
    models::masw::{Masw, MaswExp, MaswLayer},
};

#[test]
fn test_calc_depths() {
    let layers = vec![
        MaswLayer::new(1.5, 1., 1.),
        MaswLayer::new(2.5, 1., 1.),
        MaswLayer::new(4.0, 1., 1.),
    ];

    let mut masw_exp = MaswExp {
        layers,
        name: "Test".to_string(),
    };
    masw_exp.calc_depths();

    assert_eq!(masw_exp.layers[0].depth, Some(1.5));
    assert_eq!(masw_exp.layers[1].depth, Some(4.0));
    assert_eq!(masw_exp.layers[2].depth, Some(8.0));
}

#[test]
#[should_panic(expected = "Thickness of MASW experiment must be greater than zero.")]
fn test_calc_depths_invalid_thickness() {
    let layers = vec![
        MaswLayer::new(3.0, 1., 1.),
        MaswLayer::new(0.0, 1., 1.), // This should trigger a panic
    ];

    let _masw_exp = MaswExp::new(layers, "Test".to_string());
}

#[test]
fn test_get_layer_at_depth() {
    let layers = vec![
        MaswLayer::new(2.0, 1., 1.),
        MaswLayer::new(3.0, 2., 2.),
        MaswLayer::new(5.0, 3., 3.),
    ];

    let masw_exp = MaswExp::new(layers, "Test".to_string());

    let layer = masw_exp.get_layer_at_depth(4.0);
    assert_eq!(layer.vs.unwrap(), 2.0); // The second layer should be returned

    let layer = masw_exp.get_layer_at_depth(15.0);
    assert_eq!(layer.vs.unwrap(), 3.0);
}

fn create_test_maws() -> Masw {
    let exp1 = MaswExp::new(
        vec![
            MaswLayer::new(2.0, 180.0, 400.0), // depth: 2.0
            MaswLayer::new(3.0, 200.0, 450.0), // depth: 5.0
        ],
        "Exp1".into(),
    );

    let exp2 = MaswExp::new(
        vec![
            MaswLayer::new(1.5, 170.0, 390.0), // depth: 1.5
            MaswLayer::new(4.0, 190.0, 430.0), // depth: 5.5
        ],
        "Exp2".into(),
    );

    let exp3 = MaswExp::new(
        vec![
            MaswLayer::new(3.0, 160.0, 395.0), // depth: 3.0
            MaswLayer::new(3.0, 180.0, 420.0), // depth: 6.0
        ],
        "Exp3".into(),
    );

    Masw::new(vec![exp1, exp2, exp3], SelectionMethod::Min)
}

#[test]
fn test_get_idealized_exp_min_mode() {
    let mut masw = create_test_maws();

    let ideal = masw.get_idealized_exp("Ideal_Min".into());

    println!("{:?}", ideal);
    // Sanity checks
    assert_eq!(ideal.name, "Ideal_Min");

    // Should be based on union of depths: [1.5, 2.0, 3.0, 5.0, 5.5, 6.0]
    assert_eq!(ideal.layers.len(), 6);

    // Check first layer values
    let layer1 = &ideal.layers[0];
    assert_eq!(layer1.thickness.unwrap(), 1.5);
    assert_eq!(layer1.vs.unwrap(), 160.0);
    assert_eq!(layer1.vp.unwrap(), 390.0);

    // Check last layer depth
    let last_layer = ideal.layers.last().unwrap();
    assert_eq!(last_layer.depth, Some(6.0));
}

#[test]
fn test_get_idealized_exp_avg_mode() {
    let mut masw = create_test_maws();

    masw.idealization_method = SelectionMethod::Avg;
    let ideal = masw.get_idealized_exp("Ideal_Avg".into());

    println!("{:?}", ideal);
    // Sanity checks
    assert_eq!(ideal.name, "Ideal_Avg");

    // Should be based on union of depths: [1.5, 2.0, 3.0, 5.0, 5.5, 6.0]
    assert_eq!(ideal.layers.len(), 6);

    // Check first layer values
    let layer1 = &ideal.layers[0];
    assert_eq!(layer1.thickness.unwrap(), 1.5);
    assert_eq!(layer1.vs.unwrap(), 170.0);
    assert_eq!(layer1.vp.unwrap(), 395.0);

    // Check last layer depth
    let last_layer = ideal.layers.last().unwrap();
    assert_eq!(last_layer.depth, Some(6.0));
}

#[test]
fn test_get_idealized_exp_max_mode() {
    let mut masw = create_test_maws();

    masw.idealization_method = SelectionMethod::Max;
    let ideal = masw.get_idealized_exp("Ideal_Max".into());

    println!("{:?}", ideal);
    // Sanity checks
    assert_eq!(ideal.name, "Ideal_Max");

    // Should be based on union of depths: [1.5, 2.0, 3.0, 5.0, 5.5, 6.0]
    assert_eq!(ideal.layers.len(), 6);

    // Check first layer values
    let layer1 = &ideal.layers[0];
    assert_eq!(layer1.thickness.unwrap(), 1.5);
    assert_eq!(layer1.vs.unwrap(), 180.0);
    assert_eq!(layer1.vp.unwrap(), 400.0);

    // Check last layer depth
    let last_layer = ideal.layers.last().unwrap();
    assert_eq!(last_layer.depth, Some(6.0));
}
