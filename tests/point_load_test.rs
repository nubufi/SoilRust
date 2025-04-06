use approx::assert_abs_diff_eq;
use soilrust::enums::SelectionMethod;
use soilrust::models::point_load_test::*;

fn create_test_data() -> PointLoadTest {
    let sk1 = PointLoadExp::new(
        "Borehole1".to_string(),
        vec![
            PointLoadSample::new(1.5, 2.67, 50.0),
            PointLoadSample::new(3., 2.38, 50.0),
        ],
    );
    let sk2 = PointLoadExp::new(
        "Borehole2".to_string(),
        vec![
            PointLoadSample::new(1.5, 2.66, 50.0),
            PointLoadSample::new(3., 2.96, 50.0),
        ],
    );
    let sk3 = PointLoadExp::new(
        "Borehole3".to_string(),
        vec![
            PointLoadSample::new(3., 2.53, 50.0),
            PointLoadSample::new(4.5, 2.84, 50.0),
        ],
    );

    PointLoadTest::new(vec![sk1, sk2, sk3])
}
/// -----------------------------------------------------------------------------------
#[test]
fn test_get_sample_at_depth_1() {
    let exp = create_test_data().exps[0].clone();

    let sample = exp.get_sample_at_depth(1.5);
    assert_eq!(sample.depth, 1.5);
    assert_eq!(sample.is50, 2.67);
}

#[test]
fn test_get_sample_at_depth_2() {
    let exp = create_test_data().exps[0].clone();

    let sample = exp.get_sample_at_depth(2.);
    assert_eq!(sample.depth, 3.);
    assert_eq!(sample.is50, 2.38);
}

#[test]
fn test_get_sample_at_depth_3() {
    let exp = create_test_data().exps[0].clone();

    let sample = exp.get_sample_at_depth(4.);
    assert_eq!(sample.depth, 3.);
    assert_eq!(sample.is50, 2.38);
}
/// -----------------------------------------------------------------------------------
#[test]
fn test_get_idealized_exp_min_mode() {
    let data = create_test_data();

    let ideal = data.get_idealized_exp(SelectionMethod::Min, "Ideal_Min".into());

    // Sanity checks
    assert_eq!(ideal.borehole_id, "Ideal_Min");

    // Should be based on union of depths: [1.5, 3.0, 4.5]
    assert_eq!(ideal.samples.len(), 3);

    // Check first layer values
    let layer1 = &ideal.samples[0];
    assert_abs_diff_eq!(layer1.depth, 1.5, epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.is50, 2.66, epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.d, 50., epsilon = 1e-6);

    // Check first layer values
    let layer2 = &ideal.samples[1];
    assert_abs_diff_eq!(layer2.depth, 3., epsilon = 1e-6);
    assert_abs_diff_eq!(layer2.is50, 2.38, epsilon = 1e-6);
    assert_abs_diff_eq!(layer2.d, 50., epsilon = 1e-6);
    // Check last layer depth
    let last_layer = ideal.samples.last().unwrap();
    assert_abs_diff_eq!(last_layer.depth, 4.5, epsilon = 1e-6);
}

#[test]
fn test_get_idealized_exp_avg_mode() {
    let data = create_test_data();

    let ideal = data.get_idealized_exp(SelectionMethod::Avg, "Ideal_Avg".into());

    // Sanity checks
    assert_eq!(ideal.borehole_id, "Ideal_Avg");

    // Should be based on union of depths: [1.5, 3.0, 4.5]
    assert_eq!(ideal.samples.len(), 3);

    // Check first layer values
    let layer1 = &ideal.samples[0];
    assert_abs_diff_eq!(layer1.depth, 1.5, epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.is50, 2.665, epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.d, 50., epsilon = 1e-6);

    // Check first layer values
    let layer2 = &ideal.samples[1];
    assert_abs_diff_eq!(layer2.depth, 3., epsilon = 1e-6);
    assert_abs_diff_eq!(layer2.is50, 2.623, epsilon = 1e-3);
    assert_abs_diff_eq!(layer2.d, 50., epsilon = 1e-6);
    // Check last layer depth
    let last_layer = ideal.samples.last().unwrap();
    assert_abs_diff_eq!(last_layer.depth, 4.5, epsilon = 1e-6);
}

#[test]
fn test_get_idealized_exp_max_mode() {
    let data = create_test_data();

    let ideal = data.get_idealized_exp(SelectionMethod::Max, "Ideal_Max".into());

    // Sanity checks
    assert_eq!(ideal.borehole_id, "Ideal_Max");

    // Should be based on union of depths: [1.5, 3.0, 4.5]
    assert_eq!(ideal.samples.len(), 3);

    // Check first layer values
    let layer1 = &ideal.samples[0];
    assert_abs_diff_eq!(layer1.depth, 1.5, epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.is50, 2.67, epsilon = 1e-6);
    assert_abs_diff_eq!(layer1.d, 50., epsilon = 1e-6);

    // Check first layer values
    let layer2 = &ideal.samples[1];
    assert_abs_diff_eq!(layer2.depth, 3., epsilon = 1e-6);
    assert_abs_diff_eq!(layer2.is50, 2.96, epsilon = 1e-3);
    assert_abs_diff_eq!(layer2.d, 50., epsilon = 1e-6);
    // Check last layer depth
    let last_layer = ideal.samples.last().unwrap();
    assert_abs_diff_eq!(last_layer.depth, 4.5, epsilon = 1e-6);
}
