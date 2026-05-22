```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[no_mangle]
  fn abort_signal_transfer_handler(
    aborted: bool,
    msg_port: *mut web_sys::MessagePort,
  );

  #[no_mangle]
  fn serialize_abort_signal(abort_signal: &AbortSignal) -> Vec<Vec<u8>>;
}

#[wasm_bindgen]
extern "C" {
  #[no_mangle]
  fn deserialize_abort_signal(data: &[Vec<u8>]) -> Vec<AbortSignal>;
}
```