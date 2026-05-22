```rust
use crate::{ColorMapConfig, ColorModeConfig, GaugeConfig, BuildConicGradientProps};

fn setup(
    config_override: Option<ConicGradientConfig>,
    props_override: Option<BuildConicGradientProps>,
) -> (
    BuildConicGradientProps,
    String, // Gradient result
) {
    let config = ConicGradientConfig {
        color_map: ColorMapConfig::RAINBOW,
        color_mode: ColorModeConfig::COLORMAP,
        gradient: vec!["#000000".to_string(), "#FFFFFF".to_string()],
        reverse: false,
        ..config_override.unwrap_or_default()
    };

    let props = BuildConicGradientProps {
        height: 100,
        width: 200,
        gauge_angle: std::f64::consts::PI / 4.0,
        config: config as GaugeConfig,
        ..props_override.unwrap_or_default()
    };

    let gradient_result = build_conic_gradient(props);

    (props, gradient_result)
}

#[test]
fn test_build_conic_gradient_red_yellow_green() {
    let (props, _) = setup(Some(ColorMapConfig::RED_YELLOW_GREEN));
    // Compare the result with the expected gradient
}

#[test]
fn test_build_conic_gradient_rainbow() {
    let (props, _) = setup(Some(ColorMapConfig::RAINBOW));
    // Compare the result with the expected gradient
}

#[test]
fn test_build_conic_gradient_turbo() {
    let (props, _) = setup(Some(ColorMapConfig::TURBO));
    // Compare the result with the expected gradient
}

#[test]
fn test_build_conic_gradient_gradient_mode() {
    let (props, _) = setup(None, Some(BuildConicGradientProps {
        color_mode: ColorModeConfig::GRADIENT,
        gradient: vec!["#FF0000".to_string(), "#00FF00".to_string()],
        ..Default::default()
    }));
    // Compare the result with the expected gradient
}

#[test]
fn test_build_conic_gradient_reverse() {
    let (props, _) = setup(None, Some(BuildConicGradientProps {
        color_mode: ColorModeConfig::GRADIENT,
        reverse: true,
        ..Default::default()
    }));
    // Compare the result with the expected gradient
}

#[test]
fn test_build_conic_gradient_invalid_color_map() {
    let (props, _) = setup(None, Some(BuildConicGradientProps {
        color_mode: "INVALID_MODE".to_string(),
        ..Default::default()
    }));
    // Compare the result with a default gradient that includes "conic-gradient"
}

#[test]
fn test_build_conic_gradient_angle() {
    let (props, _) = setup(None, Some(BuildConicGradientProps {
        gauge_angle: std::f64::consts::PI / 2.0,
        ..Default::default()
    }));
    // Compare the result with the expected gradient start angle
}
```