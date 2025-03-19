use soilrust::models::spt::*;

// -------------------------------------------------------------------------------------------
// Test NValue
#[test]
fn test_nvalue_from_i32() {
    assert_eq!(NValue::from_i32(10), NValue::Value(10));
}

#[test]
fn test_nvalue_to_i32() {
    assert_eq!(NValue::Value(10).to_i32(), 10);
    assert_eq!(NValue::Value(0).to_i32(), 0);
    assert_eq!(NValue::Refusal.to_i32(), 50);
}

#[test]
fn test_nvalue_mul() {
    assert_eq!(NValue::Value(10).mul(2.0), NValue::Value(20));
    assert_eq!(NValue::Value(5).mul(2.5), NValue::Value(12)); // 5 * 2.5 = 12.5 -> truncated to 12
    assert_eq!(NValue::Refusal.mul(3.0), NValue::Refusal);
}

#[test]
fn test_nvalue_add() {
    assert_eq!(NValue::Value(10).add(NValue::Value(5)), NValue::Value(15));
    assert_eq!(NValue::Value(0).add(NValue::Value(0)), NValue::Value(0));
    assert_eq!(NValue::Value(10).add(NValue::Refusal), NValue::Refusal);
    assert_eq!(NValue::Refusal.add(NValue::Value(5)), NValue::Refusal);
    assert_eq!(NValue::Refusal.add(NValue::Refusal), NValue::Refusal);
}

#[test]
fn test_nvalue_add_f64() {
    assert_eq!(NValue::Value(10).add_f64(5.5), NValue::Value(15)); // 10 + 5.5 -> truncated to 15
    assert_eq!(NValue::Value(3).add_f64(1.9), NValue::Value(4)); // 3 + 1.9 -> truncated to 4
    assert_eq!(NValue::Refusal.add_f64(5.0), NValue::Refusal);
}

#[test]
fn test_nvalue_default() {
    assert_eq!(NValue::default(), NValue::Value(0));
}

#[test]
fn test_nvalue_display() {
    assert_eq!(format!("{}", NValue::Value(42)), "42");
    assert_eq!(format!("{}", NValue::Refusal), "R");
}

#[test]
fn test_nvalue_ordering() {
    assert!(NValue::Refusal > NValue::Value(1000));
    assert!(NValue::Refusal > NValue::Value(0));
    assert!(NValue::Refusal > NValue::Value(-50));
    assert!(NValue::Value(10) > NValue::Value(5));
    assert!(NValue::Value(5) < NValue::Value(10));
    assert_eq!(NValue::Refusal, NValue::Refusal);
    assert_eq!(NValue::Value(10), NValue::Value(10));
}
// -------------------------------------------------------------------------------------------

// Test SPTBlow
#[test]
fn test_sptblow_new() {
    let spt = SPTBlow::new(
        10.0,
        NValue::from_i32(5),
        NValue::from_i32(10),
        NValue::from_i32(15),
    );

    assert_eq!(spt.depth, 10.0);
    assert_eq!(spt.n1, NValue::from_i32(5));
    assert_eq!(spt.n2, NValue::from_i32(10));
    assert_eq!(spt.n3, NValue::from_i32(15));
    assert_eq!(spt.n, NValue::from_i32(25)); // n2 + n3
    assert_eq!(spt.n60, None);
    assert_eq!(spt.n90, None);
    assert_eq!(spt.n1_60, None);
    assert_eq!(spt.n1_60f, None);
    assert_eq!(spt.cn, None);
    assert_eq!(spt.alpha, None);
    assert_eq!(spt.beta, None);
}

#[test]
fn test_apply_energy_correction() {
    let mut spt = SPTBlow::new(
        10.0,
        NValue::from_i32(5),
        NValue::from_i32(10),
        NValue::from_i32(15),
    );
    spt.apply_energy_correction(1.2);

    assert_eq!(spt.n60, Some(NValue::from_i32(30))); // 25 * 1.2 = 30
    assert_eq!(spt.n90, Some(NValue::from_i32(45))); // 30 * 1.5 = 45
}

#[test]
fn test_set_cn() {
    let mut spt = SPTBlow::new(
        10.0,
        NValue::from_i32(5),
        NValue::from_i32(10),
        NValue::from_i32(15),
    );
    spt.set_cn(0.5);

    assert_eq!(spt.cn, Some(f64::min(f64::sqrt(1. / 0.5) * 9.78, 1.7))); // sqrt(1/0.5) * 9.78, capped at 1.7
}

#[test]
fn test_set_alpha_beta() {
    let mut spt = SPTBlow::new(
        10.0,
        NValue::from_i32(5),
        NValue::from_i32(10),
        NValue::from_i32(15),
    );

    spt.set_alpha_beta(4.0);
    assert_eq!(spt.alpha, Some(0.0));
    assert_eq!(spt.beta, Some(1.0));

    spt.set_alpha_beta(10.0);
    assert!((spt.alpha.unwrap() - 0.869).abs() < 0.001);
    assert!((spt.beta.unwrap() - 1.021).abs() < 0.1);

    spt.set_alpha_beta(40.0);
    assert_eq!(spt.alpha, Some(5.0));
    assert_eq!(spt.beta, Some(1.2));
}

#[test]
fn test_apply_corrections() {
    let mut spt = SPTBlow::new(
        10.0,
        NValue::from_i32(5),
        NValue::from_i32(10),
        NValue::from_i32(15),
    );

    let sigma_effective = 0.5;
    let fine_content = 10.0;
    let cr = 1.1;
    let cs = 0.9;
    let cb = 1.05;
    let ce = 1.2;

    spt.apply_corrections(sigma_effective, fine_content, cr, cs, cb, ce);

    assert_eq!(spt.n60.unwrap().to_i32(), 30);
    assert_eq!(spt.n90.unwrap().to_i32(), 45);
    assert!((spt.cn.unwrap() - 1.7).abs() < 0.001);
    assert!((spt.alpha.unwrap() - 0.869).abs() < 0.001);
    assert!((spt.beta.unwrap() - 1.021).abs() < 0.1);
    assert_eq!(spt.n1_60.unwrap().to_i32(), 53);
    assert_eq!(spt.n1_60f.unwrap().to_i32(), 54);
}
// -------------------------------------------------------------------------------------------

// Test SPT
#[test]
fn test_get_idealized_exp() {
    let mut exp1 = SPTExp::new(vec![], "exp1".to_string());
    exp1.add_blow(1.5, NValue::Value(0), NValue::Value(0), NValue::Value(10));
    exp1.add_blow(2., NValue::Value(0), NValue::Value(0), NValue::Value(20));
    exp1.add_blow(3., NValue::Value(0), NValue::Value(0), NValue::Refusal);

    let mut exp2 = SPTExp::new(vec![], "exp2".to_string());
    exp2.add_blow(1.5, NValue::Value(0), NValue::Value(0), NValue::Value(15));
    exp2.add_blow(3., NValue::Value(0), NValue::Value(0), NValue::Value(14));

    let cr = 1.1;
    let cs = 0.9;
    let cb = 1.05;
    let ce = 1.2;
    let mut spt = SPT::new(ce, cb, cs, cr);
    spt.add_exp(exp1);
    spt.add_exp(exp2);

    let idealized_exp_min =
        spt.get_idealized_exp(IdealizedMode::Min, "idealized_exp_min".to_string());
    let idealized_exp_avg =
        spt.get_idealized_exp(IdealizedMode::Average, "idealized_exp_avg".to_string());
    let idealized_exp_max =
        spt.get_idealized_exp(IdealizedMode::Max, "idealized_exp_max".to_string());

    assert_eq!(idealized_exp_min.name, "idealized_exp_min");
    assert_eq!(idealized_exp_avg.name, "idealized_exp_avg");
    assert_eq!(idealized_exp_max.name, "idealized_exp_max");

    assert_eq!(idealized_exp_min.blows.len(), 3);
    assert_eq!(idealized_exp_avg.blows.len(), 3);
    assert_eq!(idealized_exp_max.blows.len(), 3);

    assert_eq!(idealized_exp_min.blows[0].depth, 1.5);
    assert_eq!(idealized_exp_min.blows[1].depth, 2.0);
    assert_eq!(idealized_exp_min.blows[2].depth, 3.0);

    assert_eq!(idealized_exp_avg.blows[0].depth, 1.5);
    assert_eq!(idealized_exp_avg.blows[1].depth, 2.0);
    assert_eq!(idealized_exp_avg.blows[2].depth, 3.0);

    assert_eq!(idealized_exp_max.blows[0].depth, 1.5);
    assert_eq!(idealized_exp_max.blows[1].depth, 2.0);
    assert_eq!(idealized_exp_max.blows[2].depth, 3.0);

    assert_eq!(idealized_exp_min.blows[0].n, NValue::Value(10));
    assert_eq!(idealized_exp_min.blows[1].n, NValue::Value(20));
    assert_eq!(idealized_exp_min.blows[2].n, NValue::Value(14));

    assert_eq!(idealized_exp_avg.blows[0].n, NValue::Value(13));
    assert_eq!(idealized_exp_avg.blows[1].n, NValue::Value(20));
    assert_eq!(idealized_exp_avg.blows[2].n, NValue::Value(32));

    assert_eq!(idealized_exp_max.blows[0].n, NValue::Value(15));
    assert_eq!(idealized_exp_max.blows[1].n, NValue::Value(20));
    assert_eq!(idealized_exp_max.blows[2].n, NValue::Refusal);
}
