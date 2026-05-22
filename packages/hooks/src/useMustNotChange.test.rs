```rust
use wasm_bindgen::prelude::*;
use log::{error, Level};

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    fn useMustNotChangeImpl(val: i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_log_an_error_when_value_changes() {
        let error_mock = Box::new(|| {
            assert_eq!(error_level(), Level::Error);
        });
        Logger.channels().for_each(|channel| {
            if channel.name().ends_with("useMustNotChange.ts")) {
                channel.error = error_mock;
            }
        });

        render_hook(
            (val: i32) => {
                useMustNotChangeImpl(val);
            },
            initial_props: 1,
        );
        rerender(2);

        assert_eq!(error_mock.borrow(), error_level());
    }
}
```