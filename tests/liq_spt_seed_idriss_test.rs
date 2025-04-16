use approx::assert_abs_diff_eq;
use soilrust::liquefaction::spt::seed_idriss::{calc_crr75, calc_settlement};

#[test]
fn test_calc_crr75() {
    let n1_60_f = 15;
    let effective_stress = 8.0; // ton/m²

    let expected = 1.28;

    let result = calc_crr75(n1_60_f, effective_stress);
    assert_abs_diff_eq!(result, expected, epsilon = 1e-2);
}

#[test]
fn test_calc_settlement() {
    let fs = 1.;
    let layer_thickness = 1.0; // m
    let n60 = 11; // Corrected N60 value

    let expected = 1.7;
    let result = calc_settlement(fs, layer_thickness, n60);

    assert_abs_diff_eq!(result, expected, epsilon = 1e-1);
}
