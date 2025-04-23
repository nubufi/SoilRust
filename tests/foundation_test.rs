use soilrust::models::foundation::Foundation;

#[test]
fn test_calc_effective_lengths() {
    let mut foundation = Foundation {
        foundation_length: Some(10.0),
        foundation_width: Some(5.0),
        ..Default::default()
    };

    let ex = 1.0; // Eccentricity in x-direction (m)
    let ey = 1.5; // Eccentricity in y-direction (m)

    foundation.calc_effective_lengths(ex, ey);

    // Expected values:
    // b' = 5 - 2 * 1.0 = 3.0
    // l' = 10 - 2 * 1.5 = 7.0
    // effective_width = min(3.0, 7.0) = 3.0
    // effective_length = max(3.0, 7.0) = 7.0
    assert_eq!(foundation.effective_width, Some(3.0));
    assert_eq!(foundation.effective_length, Some(7.0));
}

#[test]
fn test_calc_effective_lengths_zero_eccentricity() {
    let mut foundation = Foundation {
        foundation_length: Some(8.0),
        foundation_width: Some(4.0),
        ..Default::default()
    };

    foundation.calc_effective_lengths(0.0, 0.0);

    // No eccentricity, so effective dimensions should remain the same
    assert_eq!(foundation.effective_width, Some(4.0));
    assert_eq!(foundation.effective_length, Some(8.0));
}

#[test]
fn test_calc_effective_lengths_negative_effective_size() {
    let mut foundation = Foundation {
        foundation_length: Some(6.0),
        foundation_width: Some(3.0),
        ..Default::default()
    };

    let ex = 2.0; // Large eccentricity causing negative width
    let ey = 2.0; // Large eccentricity causing negative length

    foundation.calc_effective_lengths(ex, ey);

    // Negative values should be prevented (width or length cannot be negative)
    assert_eq!(foundation.effective_width, Some(0.0));
    assert_eq!(foundation.effective_length, Some(2.0)); // The remaining length
}
