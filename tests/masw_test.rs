use soilrust::models::masw::{Masw, MaswExp};

#[test]
fn test_masw_new() {
    let exps = vec![
        MaswExp::new(2.0, 5.),
        MaswExp::new(3.0, 5.),
        MaswExp::new(5.0, 5.),
    ];

    let masw = Masw::new(exps);

    assert_eq!(masw.exps[0].depth, 2.0);
    assert_eq!(masw.exps[1].depth, 5.0);
    assert_eq!(masw.exps[2].depth, 10.0);
}

#[test]
fn test_calc_depths() {
    let exps = vec![
        MaswExp::new(1.5, 1.),
        MaswExp::new(2.5, 1.),
        MaswExp::new(4.0, 1.),
    ];

    let mut masw = Masw { exps };
    masw.calc_depths();

    assert_eq!(masw.exps[0].depth, 1.5);
    assert_eq!(masw.exps[1].depth, 4.0);
    assert_eq!(masw.exps[2].depth, 8.0);
}

#[test]
#[should_panic(expected = "Thickness of MASW experiment must be greater than zero.")]
fn test_calc_depths_invalid_thickness() {
    let exps = vec![
        MaswExp::new(3.0, 1.),
        MaswExp::new(0.0, 1.), // This should trigger a panic
    ];

    let _masw = Masw::new(exps);
}

#[test]
fn test_get_exp_at_depth() {
    let exps = vec![
        MaswExp::new(2.0, 1.),
        MaswExp::new(3.0, 2.),
        MaswExp::new(5.0, 3.),
    ];

    let masw = Masw::new(exps);

    let exp = masw.get_exp_at_depth(4.0);
    assert_eq!(exp.vs, 2.0); // The second experiment should be returned

    let exp = masw.get_exp_at_depth(15.0);
    assert_eq!(exp.vs, 3.0);
}
