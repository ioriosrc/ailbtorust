```rust
use three::{Color, ColorManagement};
use tinycolor2::Tinycolor;

pub const LIGHT_OUTLINE: Color = Color::new(0.0, 0.0, 0.0).convert_srgb_to_linear();
pub const DARK_OUTLINE: Color = Color::new(1.0, 1.0, 1.0).convert_srgb_to_linear();

// From https://github.com/mrdoob/three.js/blob/dev/src/math/ColorManagement.js
// which is not exported
fn srgb_to_linear(c: f64) -> f64 {
    if c < 0.04045 {
        c * 0.0773993808
    } else {
        (c * 0.9478672986 + 0.0521327014).powf(2.4)
    }
}

const LUT_SIZE: usize = 256;
const MAX_INDEX: usize = LUT_SIZE - 1;

// Convert SRGB to Linear RGB
fn srgb_to_linear_rgb(output: &mut [f64; LUT_SIZE], r: f64, g: f64, b: f64) {
    let ri = (r * MAX_INDEX as f64).round() as usize;
    let gi = (g * MAX_INDEX as f64).round() as usize;
    let bi = (b * MAX_INDEX as f64).round() as usize;

    output[ri] = srgb_to_linear(r);
    output[gi] = srgb_to_linear(g);
    output[bi] = srgb_to_linear(b);
}

pub fn string_to_rgba(output: &mut [f64; 4], color_str: &str) {
    let color = tinycolor(color_str);
    if !color.is_valid() {
        output.fill(1.0);
        return;
    }
    let rgb = color.to_rgb();
    output[0] = rgb.r / 255.;
    output[1] = rgb.g / 255.;
    output[2] = rgb.b / 255.;
    output[3] = rgb.a;
}

pub fn make_rgba() -> [f64; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

pub fn make_rgb() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

pub fn string_to_rgb<T: Color>(output: &mut [f64; 4], color_str: &str) -> &mut [f64; 4] {
    let color = tinycolor(color_str);
    if !color.is_valid() {
        output.fill(1.0);
        return output;
    }
    let rgb = color.to_rgb();
    output[0] = rgb.r / 255.;
    output[1] = rgb.g / 255.;
    output[2] = rgb.b / 255.;
    return output;
}

/// Converts a ColorRGB to THREE.Color and converts from sRGB to linear RGB.
pub fn rgb_to_three_color(output: &mut [f64; 4], rgb: [f64; 3]) -> [f64; 4] {
    let color = Color::new(rgb[0], rgb[1], rgb[2]);
    output.copy_from(&color.convert_srgb_to_linear());
    output
}

pub fn rgba_to_css_string(color: [f64; 4]) -> String {
    let r = (color[0] * 255.).round() as i32;
    let g = (color[1] * 255.).round() as i32;
    let b = (color[2] * 255.).round() as i32;
    format!("rgba({}, {}, {}, {})", r, g, b, color[3])
}

pub fn rgba_to_linear(output: &mut [f64; 4], color: [f64; 4]) -> &mut [f64; 4] {
    output[0] = srgb_to_linear(color[0]);
    output[1] = srgb_to_linear(color[1]);
    output[2] = srgb_to_linear(color[2]);
    output[3] = color[3];
    output
}

// https://stackoverflow.com/a/596243
pub fn get_luminance(r: f64, g: f64, b: f64) -> f64 {
    (0.5468 * r).hypot((0.7662 * g).hypot((0.3376 * b).hypot()))
}

/**
 * Computes a gradient step from colors `a` to `b` using pre-multiplied alpha to
 * match CSS linear gradients. The inputs are assumed to not have pre-multiplied
 * alpha, and the output will have pre-multiplied alpha.
 */
pub fn rgba_gradient(output: &mut [f64; 4], a: [f64; 3], b: [f64; 3], t: f64) -> &mut [f64; 4] {
    let a_r = a[0] * a[2];
    let a_g = a[1] * a[2];
    let a_b = a[2] * a[2];
    let b_r = b[0] * b[2];
    let b_g = b[1] * b[2];
    let b_b = b[2] * b[2];

    output[0] = lerp(a_r, b_r, t);
    output[1] = lerp(a_g, b_g, t);
    output[2] = lerp(a_b, b_b, t);
    output[3] = lerp(a[2], b[2], t);
    output
}
```