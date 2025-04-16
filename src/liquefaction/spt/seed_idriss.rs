use crate::{
    helper::interp1d,
    liquefaction::{
        helper_functions::{calc_csr, calc_msf, calc_rd},
        models::{LiquefactionLayerResult, LiquefactionResult},
    },
    models::{soil_profile::SoilProfile, spt::SPTExp},
};

/// Validates the soil profile and SPT data
fn validate(soil_profile: &SoilProfile, spt: &SPTExp) -> Result<(), String> {
    if spt.blows.is_empty() {
        return Err("No SPT data provided.".to_string());
    }
    for blow in &spt.blows {
        if blow.n1_60.is_none() || blow.n1_60f.is_none() {
            return Err("N1_60 and N1_60f values are required.".to_string());
        }
        if blow.thickness.is_none() {
            return Err("Layer thickness is required.".to_string());
        }
    }
    if soil_profile.layers.is_empty() {
        return Err("No soil profile data provided.".to_string());
    }
    for layer in &soil_profile.layers {
        if layer.plasticity_index.is_none() {
            return Err("Plasticity index is required.".to_string());
        }
        if layer.dry_unit_weight.is_none() {
            return Err("Drainage condition is required.".to_string());
        }
        if layer.saturated_unit_weight.is_none() {
            return Err("Saturated unit weight is required.".to_string());
        }
    }

    Ok(())
}

/// Calculates cyclic resistance ratio (CRR) based on N1_60 and effective stress
///
/// # Arguments
/// * `n1_60` - N1_60 value
/// * `effective_stress` - Effective stress in ton/m²
///
/// # Returns
/// * `crr` - Cyclic resistance ratio
pub fn calc_crr75(n1_60_f: i32, effective_stress: f64) -> f64 {
    let n1_60_f = n1_60_f as f64;
    ((1.0 / (34.0 - n1_60_f)) + (n1_60_f / 135.0) + (50.0 / ((10.0 * n1_60_f + 45.0).powi(2)))
        - 1.0 / 200.0)
        * effective_stress
}

/// Calculates settlement due to liquefaction for a single layer
///
/// # Arguments
/// * `fs` - Factor of Safety
/// * `layer_thickness` - Thickness of the layer (m)
/// * `n60` - Corrected N60 value
///
/// # Returns
/// * Settlement in cm
pub fn calc_settlement(fs: f64, layer_thickness: f64, n60: i32) -> f64 {
    let mut n90 = (n60 as f64) * 6.0 / 9.0;
    n90 = n90.clamp(3.0, 30.0);

    let a0 = 0.3773;
    let a1 = -0.0337;
    let a2 = 1.5672;
    let a3 = -0.1833;
    let b0 = 28.45;
    let b1 = -9.3372;
    let b2 = 0.7975;

    let n90_list = [3.0, 6.0, 10.0, 14.0, 25.0, 30.0];
    let q_list = [33.0, 45.0, 60.0, 80.0, 147.0, 200.0];

    let q = interp1d(&n90_list, &q_list, n90);

    let settlement = match fs {
        f if f > 2.0 => 0.0,
        f if f < 2.0 && f > (2.0 - 1.0 / (a2 + a3 * q.ln())) => {
            let s1 = (a0 + a1 * q.ln()) / ((1.0 / (2.0 - f)) - (a2 + a3 * q.ln()));
            let s2 = b0 + b1 * q.ln() + b2 * q.ln().powi(2);
            s1.min(s2)
        }
        _ => b0 + b1 * q.ln() + b2 * q.ln().powi(2),
    };

    settlement * layer_thickness
}

/// Calculates liquefaction potential for a soil profile using SPT data
///
/// # Arguments
/// * `soil_profile` - Soil profile data
/// * `spt` - SPT data
/// * `pga` - Peak Ground Acceleration
/// * `mw` - Moment magnitude
///
/// # Returns
/// * `LiquefactionResult` - Result of liquefaction analysis
pub fn calc_liquefacion(
    soil_profile: &SoilProfile,
    spt: &SPTExp,
    pga: f64,
    mw: f64,
) -> LiquefactionResult {
    validate(soil_profile, spt).unwrap();
    let msf = calc_msf(mw);
    let mut layer_results = Vec::new();

    for blow in spt.blows.iter() {
        let thickness = blow.thickness.unwrap();
        let depth = blow.depth;
        let rd = calc_rd(depth);
        let n60 = blow.n60.unwrap().to_i32();
        let n1_60 = blow.n1_60.unwrap().to_i32();
        let n1_60_f = blow.n1_60f.unwrap().to_i32();
        let effective_stress = soil_profile.calc_effective_stress(depth);
        let normal_stress = soil_profile.calc_normal_stress(depth);
        let soil_layer = soil_profile.get_layer_at_depth(depth);
        let plasticity_index = soil_layer.plasticity_index.unwrap();

        let conditions = [
            soil_profile.ground_water_level >= depth,
            plasticity_index >= 12.,
            n1_60 >= 30,
            n1_60_f >= 34,
        ];
        if conditions.iter().all(|&x| x) {
            let layer_result = LiquefactionLayerResult {
                normal_stress,
                effective_stress,
                crr: None,
                crr75: None,
                csr: None,
                safety_factor: None,
                is_safe: true,
                settlement: 0.0,
                rd,
                ..Default::default()
            };
            layer_results.push(layer_result);
            continue;
        }
        let csr = calc_csr(pga, normal_stress, rd);
        let crr75 = calc_crr75(n1_60_f, effective_stress);
        let crr = msf * crr75;
        let safety_factor = crr / csr;

        let settlement = calc_settlement(safety_factor, thickness, n60);

        let layer_result = LiquefactionLayerResult {
            normal_stress,
            effective_stress,
            crr: Some(crr),
            crr75: Some(crr75),
            csr: Some(csr),
            safety_factor: Some(safety_factor),
            is_safe: safety_factor > 1.1,
            settlement,
            rd,
            ..Default::default()
        };
        layer_results.push(layer_result);

        // Add the layer result to the liquefaction result
    }
    let total_settlement = layer_results.iter().map(|x| x.settlement).sum();
    LiquefactionResult {
        layers: layer_results,
        total_settlement,
        msf,
    }
}
