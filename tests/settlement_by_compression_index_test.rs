use approx::assert_abs_diff_eq;
use soilrust::consolidation_settlement::by_compression_index::calc_single_layer_settlement;

#[test]
fn test_calc_single_layer_settlement() {
    // Test case 1: Normal case
    let h = 10.0; // Thickness of the layer [m]
    let cc = 0.2; // Compression Index (Cc)
    let cr = 0.2; // Recompression Index (Cr)
    let e0 = 0.3; // Initial Void Ratio (e₀)
    let gp = 10.0; // Preconsolidation Pressure [t]
    let g0 = 20.0; // Initial Effective Stress [t]
    let delta_stress = 10.0; // Stress increase due to foundation [t]

    let expected = 27.091; // Expected settlement [cm]
    let settlement = calc_single_layer_settlement(h, cc, cr, e0, gp, g0, delta_stress);
    assert_abs_diff_eq!(settlement, expected, epsilon = 0.001);
}
