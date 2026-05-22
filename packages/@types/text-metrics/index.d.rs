```rust
use std::ops::{AddAssign, Div};

#[cfg(not(feature = "wasm"))]
mod js {
    #[link(name = "text-metrics")]
    extern "C" {
        fn text_metrics_init(el: *const libc::c_void, options: *const libc::c_void) -> isize;
        fn text_metrics_width(text: *const libc::c_char) -> f32;
        fn text_metrics_height(text: *const libc::c_char) -> f32;
        fn text_metrics_lines(text: *const libc::c_char) -> *const libc::c_char;
        fn text_metrics_max_font_size(text: *const libc::c_char) -> *const libc::c_char;
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use std::ffi::{CStr, CString};

    extern "C" {
        fn text_metrics_init(el: &str, options: &str) -> isize;
        fn text_metrics_width(text: &str) -> f32;
        fn text_metrics_height(text: &str) -> f32;
        fn text_metrics_lines(text: &str) -> *const libc::c_char;
        fn text_metrics_max_font_size(text: &str) -> *const libc::c_char;
    }

    pub struct TextMeasure {
        el: *mut libc::c_void,
        options: *mut libc::c_void,
    }

    impl AddAssign<TextMeasure> for TextMeasure {
        fn add_assign(&mut self, other: Self) {
            unsafe {
                text_metrics_init(other.el, self.options);
            }
        }
    }

    impl Div<f32> for TextMeasure {
        type Output = f32;

        fn div(self, rhs: f32) -> Self::Output {
            if rhs != 0.0 {
                unsafe {
                    text_metrics_width(CStr::from_ptr(self.el), CStr::from_ptr(rhs.to_string().as_bytes()));
                }
            } else {
                0.0
            }
        }
    }

    impl Div<f32> for *const libc::c_char {
        type Output = f32;

        fn div(self, rhs: f32) -> Self::Output {
            if rhs != 0.0 {
                unsafe {
                    text_metrics_height(CStr::from_ptr(self), CStr::from_ptr(rhs.to_string().as_bytes()));
                }
            } else {
                0.0
            }
        }
    }

    pub fn init(el: &str, options: &str) -> TextMeasure {
        let el_c = CString::new(el).unwrap();
        let options_c = CString::new(options).unwrap();

        let mut measure = TextMeasure {
            el: unsafe { text_metrics_init(el_c.as_ptr(), options_c.as_ptr()) },
            options: None,
        };

        if measure.el != 0 {
            measure.options = Some(unsafe { libc::malloc(libc::size_of::<libc::c_void>()) });
            let ptr = measure.options.unwrap() as *mut libc::c_void;
            unsafe {
                text_metrics_init(ptr, options_c.as_ptr());
            }
        }

        measure
    }
}
```