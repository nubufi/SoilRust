use crate::models::soil_profile::SoilProfile;

/// Calculates the center and thickness of a soil layer based on the ground water table (GWT) and the depth of the foundation (df).
///
/// # Arguments
/// * `soil_profile` - The soil profile containing the layers.
/// * `df` - The depth of the foundation.
/// * `layer_index` - The index of the layer.
///
/// # Returns
/// * A tuple containing the center and thickness of the layer.
pub fn get_center_and_thickness(
    soil_profile: &SoilProfile,
    df: f64,
    layer_index: usize,
) -> (f64, f64) {
    let gwt = soil_profile.ground_water_level.unwrap();
    let gwt_layer_index = soil_profile.get_layer_index(gwt);
    let df_layer_index = soil_profile.get_layer_index(df);
    let layer = &soil_profile.layers[layer_index];

    let (center, thickness) = if gwt_layer_index < layer_index {
        if layer_index == df_layer_index {
            let thickness = layer.thickness.unwrap() - df;
            let center = df + thickness / 2.0;
            (center, thickness)
        } else {
            let thickness = layer.thickness.unwrap();
            let center = layer.center.expect("Layer center must be Some");
            (center, thickness)
        }
    } else {
        let max_depth = df.max(gwt);
        let thickness = layer.thickness.unwrap() - max_depth;
        let center = max_depth + thickness / 2.0;
        (center, thickness)
    };

    (center, thickness)
}

/// Calculates the change in effective stress (delta_stress) based on the foundation pressure (q),
/// width, length, and center of the layer.
///
/// # Arguments
/// * `q` - Foundation pressure [t/m²].
/// * `width` - Width of the foundation [m].
/// * `length` - Length of the foundation [m].
/// * `center` - Center of the layer [m].
///
/// # Returns
/// * Change in effective stress [t/m²].
pub fn calc_delta_stress(q: f64, width: f64, length: f64, center: f64) -> f64 {
    q * width * length / (width + center) * (length + center)
}
