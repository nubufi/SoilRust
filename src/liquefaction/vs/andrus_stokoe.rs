use crate::{
    helper::interp1d,
    liquefaction::{
        helper_functions::{calc_csr, calc_msf, calc_rd},
        models::{CommonLiquefactionLayerResult, VSLiquefactionLayerResult, VSLiquefactionResult},
    },
    models::{masw::Masw, soil_profile::SoilProfile},
    validation::ValidationError,
};

/// Validates the input data for liquefaction calculations.
///
/// # Arguments
/// * `masw` - The MASW data.
/// * `soil_profile` - The soil profile data.
///
/// # Returns
/// * `Result<(), ValidationError>`: Ok if valid, Err if invalid.
pub fn validate_input(masw: &Masw, soil_profile: &SoilProfile) -> Result<(), ValidationError> {
    masw.validate(&["thickness", "vs"])?;
    soil_profile.validate(&[
        "thickness",
        "dry_unit_weight",
        "saturated_unit_weight",
        "plasticity_index",
        "fine_content",
    ])?;

    Ok(())
}

/// Calculates Vs1c based on fine content
///
/// # Arguments
/// * `fine_content` - Fine content in percentage
///
/// # Returns
/// * `vs1c` - Vs1c value
pub fn calc_vs1c(fine_content: f64) -> f64 {
    match fine_content {
        fc if fc <= 5.0 => 215.0,
        fc if fc > 5.0 && fc <= 35.0 => 215. - 0.5 * (fc - 5.0),
        _ => 200.,
    }
}

/// Calculates cyclic resistance ratio (CRR) based on N1_60 and effective stress
///
/// # Arguments
/// * `vs1` - Vs1 value
/// * `vs1c` - Vs1c value
/// * `effective_stress` - Effective stress in ton/m²
///
/// # Returns
/// * `crr` - Cyclic resistance ratio
pub fn calc_crr75(vs1: f64, vs1c: f64, effective_stress: f64) -> f64 {
    ((0.03 * (vs1 / 100.).powf(2.)) + 0.09 / (vs1c - vs1) - 0.09 / vs1c) * effective_stress
}

/// Calculates Cn correction factor based on effective stress
///
/// # Arguments
/// * `effective_stress` - Effective stress in ton/m²
///
/// # Returns
/// * `cn` - Cn correction factor
pub fn calc_cn(effective_stress: f64) -> f64 {
    let cn = 1.16 * (1. / effective_stress).powf(0.5);
    cn.min(1.7)
}

/// Calculates settlement due to liquefaction for a single layer
///
/// # Arguments
/// * `fs` - Factor of Safety
/// * `layer_thickness` - Thickness of the layer (m)
/// * `vs1` - Vs1c value
///
/// # Returns
/// * Settlement in cm
pub fn calc_settlement(fs: f64, layer_thickness: f64, vs1: f64) -> f64 {
    let dr = 17.974 * (vs1 / 100.).powf(1.976);

    let a0 = 0.3773;
    let a1 = -0.0337;
    let a2 = 1.5672;
    let a3 = -0.1833;
    let b0 = 28.45;
    let b1 = -9.3372;
    let b2 = 0.7975;

    let dr_list = [30.0, 40., 50.0, 60.0, 70.0, 80.0, 90.];
    let q_list = [33.0, 45.0, 60.0, 80.0, 110., 147.0, 200.0];

    let q = interp1d(&dr_list, &q_list, dr);

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
    soil_profile: &mut SoilProfile,
    masw: &mut Masw,
    pga: f64,
    mw: f64,
) -> Result<VSLiquefactionResult, ValidationError> {
    validate_input(masw, soil_profile)?;
    soil_profile.calc_layer_depths();

    let mut masw_exp = masw.get_idealized_exp("idealized".to_string());
    masw_exp.calc_depths();

    let msf = calc_msf(mw);
    let mut layer_results = Vec::new();
    let mut vs_layers = Vec::new();

    for layer in soil_profile.layers.iter() {
        let thickness = layer.thickness.unwrap();
        let depth = layer.depth.unwrap();
        let rd = calc_rd(depth);
        let effective_stress = soil_profile.calc_effective_stress(depth);
        let normal_stress = soil_profile.calc_normal_stress(depth);
        let soil_layer = soil_profile.get_layer_at_depth(depth);
        let plasticity_index = soil_layer.plasticity_index.unwrap();
        let masw_layer = masw_exp.get_layer_at_depth(depth);
        let vs = masw_layer.vs.unwrap();
        let cn = calc_cn(effective_stress);
        let vs1 = vs * cn;
        let vs1c = calc_vs1c(soil_layer.fine_content.unwrap());

        let conditions = [
            soil_profile.ground_water_level.unwrap() >= depth,
            plasticity_index >= 12.,
            vs1 >= vs1c,
        ];
        if conditions.iter().any(|&x| x) {
            let layer_result = CommonLiquefactionLayerResult {
                depth,
                normal_stress,
                effective_stress,
                rd,
                ..Default::default()
            };
            layer_results.push(layer_result);
            continue;
        }
        let csr = calc_csr(pga, normal_stress, rd);
        let crr75 = calc_crr75(vs1, vs1c, effective_stress);
        let crr = msf * crr75;
        let safety_factor = crr / csr;

        let settlement = calc_settlement(safety_factor, thickness, vs1);
        let vs_layer_result = VSLiquefactionLayerResult {
            vs1: Some(vs1),
            vs1c: Some(vs1c),
            cn: Some(cn),
        };
        vs_layers.push(vs_layer_result);

        let layer_result = CommonLiquefactionLayerResult {
            soil_layer: soil_layer.clone(),
            depth,
            normal_stress,
            effective_stress,
            crr: Some(crr),
            crr75: Some(crr75),
            csr: Some(csr),
            safety_factor: Some(safety_factor),
            is_safe: safety_factor > 1.1,
            settlement,
            rd,
        };
        layer_results.push(layer_result);

        // Add the layer result to the liquefaction result
    }
    let total_settlement = layer_results.iter().map(|x| x.settlement).sum();
    Ok(VSLiquefactionResult {
        layers: layer_results,
        vs_layers,
        total_settlement,
        msf,
    })
}
