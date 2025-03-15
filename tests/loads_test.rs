use soilrust::models::loads::{LoadCase, LoadSeverity, Loads, Stress};

#[test]
fn test_calc_eccentricity() {
    let loading = Loads {
        moment_x: Some(20.0),
        moment_y: Some(15.0),
        ..Default::default()
    };

    let (ex, ey) = loading.calc_eccentricity(10.);

    assert!((ex - 2.).abs() < 1e-6);
    assert!((ey - 1.5).abs() < 1e-6);
}

#[test]
fn test_calc_eccentricity_zero_load() {
    let loading = Loads {
        moment_x: Some(20.0),
        moment_y: Some(15.0),
        ..Default::default()
    };

    let (ex, ey) = loading.calc_eccentricity(0.);

    assert_eq!(ex, 0.0);
    assert_eq!(ey, 0.0);
}

#[test]
fn test_get_vertical_stress() {
    // Create a struct with known values
    let stress_data = Loads {
        service_load: Stress {
            min: Some(10.0),
            avg: Some(15.0),
            max: Some(20.0),
        },
        ultimate_load: Stress {
            min: Some(25.0),
            avg: Some(30.0),
            max: Some(35.0),
        },
        seismic_load: Stress {
            min: Some(40.0),
            avg: Some(45.0),
            max: None,
        },
        ..Default::default()
    };

    // Test Service Load
    assert_eq!(
        stress_data.get_vertical_stress(LoadCase::ServiceLoad, LoadSeverity::Min),
        10.0
    );
    assert_eq!(
        stress_data.get_vertical_stress(LoadCase::ServiceLoad, LoadSeverity::Avg),
        15.0
    );
    assert_eq!(
        stress_data.get_vertical_stress(LoadCase::ServiceLoad, LoadSeverity::Max),
        20.0
    );

    // Test Ultimate Load
    assert_eq!(
        stress_data.get_vertical_stress(LoadCase::UltimateLoad, LoadSeverity::Min),
        25.0
    );
    assert_eq!(
        stress_data.get_vertical_stress(LoadCase::UltimateLoad, LoadSeverity::Avg),
        30.0
    );
    assert_eq!(
        stress_data.get_vertical_stress(LoadCase::UltimateLoad, LoadSeverity::Max),
        35.0
    );

    // Test Seismic Load
    assert_eq!(
        stress_data.get_vertical_stress(LoadCase::SeismicLoad, LoadSeverity::Min),
        40.0
    );
    assert_eq!(
        stress_data.get_vertical_stress(LoadCase::SeismicLoad, LoadSeverity::Avg),
        45.0
    );
    assert_eq!(
        stress_data.get_vertical_stress(LoadCase::SeismicLoad, LoadSeverity::Max),
        0.0
    );
}
