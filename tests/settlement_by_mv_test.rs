use soilrust::consolidation_settlement::by_mv::calc_single_layer_settlement;

#[test]
fn test_settlement_by_mv() {
    let mv = 0.004;
    let thickness = 10.;
    let delta_stress = 10.;

    let expected_settlement = 40.0;

    let settlement = calc_single_layer_settlement(mv, thickness, delta_stress);

    assert_eq!(settlement, expected_settlement);
}
