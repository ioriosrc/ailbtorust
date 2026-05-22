```rust
use std::convert::From;

pub fn yuv_to_rgba8(
    y1: f32,
    u: f32,
    v: f32,
    output: &mut [f32; 4],
) {
    // rgba
    output[0] = y1 + (1403.0 * v) / 1000.0;
    output[1] = y1 - ((344.0 * u) + (714.0 * v)) / 1000.0;
    output[2] = y1;
    output[3] = 255.0;
}

pub fn decode_mono8(mono8: &[u8], width: usize, height: usize, output: &mut [f32; 4]) {
    if width == 0 || height == 0 {
        return;
    }

    for i in 0..height {
        for j in 0..width {
            let val = mono8[i * width + j] as f32 / 255.0;
            output[(i * width + j) * 4..(i * width + j + 1) * 4].fill(val);
        }
    }
}

pub fn decode_mono16(mono16: &[u8], width: usize, height: usize, output: &mut [f32; 4]) {
    if width == 0 || height == 0 {
        return;
    }

    for i in 0..height {
        for j in 0..width {
            let val = (mono16[i * width + j] as f32 - 32768.0) / 32767.0; // Normalize to [0, 1]
            output[(i * width + j) * 4..(i * width + j + 1) * 4].fill(val);
        }
    }
}

pub fn make_specialized_decode_bayer(tl: char, tr: char, bl: char, br: char) -> impl Fn(&[u8], usize, usize, &mut [f32; 4]) {
    let data = |data: &[u8], width: usize, height: usize, output: &mut [f32; 4]| {
        if width < height * 2 / 2 || height < width * 2 / 2 {
            panic!("Invalid Bayer image dimensions");
        }

        for i in 0..height / 2 {
            let tl = data[i as usize * (width * 2) + tl as usize];
            let tr = data[i as usize * (width * 2) + tr as usize];
            let bl = data[(i as usize + 1) * (width * 2) - 2];
            let br = data[(i as usize + 1) * (width * 2) - 1];

            for j in 0..width / 2 {
                output[i as usize * width * 4 + j as usize] = tl;
                output[i as usize * width * 4 + j as usize + 1] = tr;
                output[(i as usize + 1) * width * 4 + j as usize] = bl;
                output[(i as usize + 1) * width * 4 + j as usize + 1] = br;
            }
        }
    };

    return data;
}

// Specialize the Bayer decode function to a certain encoding.
pub const decode_bayer_rggb8: fn(&[u8], usize, usize, &mut [f32; 4]) = make_specialized_decode_bayer('r', 'g0', 'g1', 'b');
pub const decode_bayer_bggr8: fn(&[u8], usize, usize, &mut [f32; 4]) = make_specialized_decode_bayer('b', 'g0', 'g1', 'r');
pub const decode_bayer_gbrg8: fn(&[u8], usize, usize, &mut [f32; 4]) = make_specialized_decode_bayer('g0', 'b', 'r', 'g1');
pub const decode_bayer_grbg8: fn(&[u8], usize, usize, &mut [f32; 4]) = make_specialized_decode_bayer('g0', 'r', 'b', 'g1');
```