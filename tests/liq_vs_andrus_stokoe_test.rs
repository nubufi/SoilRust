use approx::assert_abs_diff_eq;
use soilrust::liquefaction::vs::andrus_stokoe::{calc_crr75, calc_settlement, calc_vs1c};

#[test]
fn test_calc_vs1c_low_fine_content() {
    // fc <= 5.0
    let fine_content = 3.0;
    let expected = 215.0;
    let result = calc_vs1c(fine_content);
    assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
}

#[test]
fn test_calc_vs1c_mid_fine_content() {
    // 5.0 < fc <= 35.0
    let fine_content = 20.0;
    let expected = 215.0 - 0.5 * (fine_content - 5.0); // = 207.5
    let result = calc_vs1c(fine_content);
    assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
}
#[test]
fn test_calc_vs1c_high_fine_content() {
    // fc > 35.0
    let fine_content = 40.0;
    let expected = 200.0;
    let result = calc_vs1c(fine_content);
    assert_abs_diff_eq!(result, expected, epsilon = 1e-6);
}

#[test]
fn test_calc_crr75_single_case() {
    let vs1 = 180.0; // m/s
    let vs1c = 200.0; // m/s
    let effective_stress = 7.0; // ton/m²
    let expected = 0.708;

    let result = calc_crr75(vs1, vs1c, effective_stress);
    assert_abs_diff_eq!(result, expected, epsilon = 1e-2);
}

#[test]
fn test_calc_settlement() {
    let fs = 1.;
    let layer_thickness = 1.0; // m
    let vs1 = 180.; // Corrected N60 value

    let expected = 1.03;
    let result = calc_settlement(fs, layer_thickness, vs1);

    assert_abs_diff_eq!(result, expected, epsilon = 1e-2);
}
