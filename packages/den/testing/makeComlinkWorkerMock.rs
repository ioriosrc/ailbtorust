```rust
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
  let event_loop = Arc::new(Mutex::new(None));
  let worker = make_comlink_worker(event_loop.clone());

  // Simulate sending a message to the Worker
  let msg = "Hello from Rust";
  worker.post_message(msg);

  Ok(())
}

fn make_comlink_worker(event_loop: Arc<Mutex<Option<JsValue>>>) -> JsValue {
  let worker = wasm_bindgen::unstable::JsUnwrap::upcast(
    JsValue::from_serde(&WorkerEndpoint { event_loop })
  );

  Comlink::expose(worker.clone(), &worker);

  worker
}

struct WorkerEndpoint {
  event_loop: Arc<Mutex<Option<JsValue>>>,
}

impl EventTarget for WorkerEndpoint {
  fn add_event_listener<S, F>(&self, type_: S, f: F)
  where
    S: AsRef<str>,
    F: FnMut(&Self, &Event),
  {
    self.event_loop.lock().unwrap().as_mut().map(|event_loop| {
      event_loop.add_event_listener(type_, move |ev| {
        let js_value = ev.clone();
        f(self, &JsValue::from(js_value));
      })
    });
  }

  fn remove_event_listener<S, F>(&self, type_: S, f: F)
  where
    S: AsRef<str>,
    F: FnMut(&Self, &Event),
  {
    self.event_loop.lock().unwrap().as_mut().map(|event_loop| {
      event_loop.remove_event_listener(type_, move |ev| {
        let js_value = ev.clone();
        f(self, &JsValue::from(js_value));
      })
    });
  }
}

trait Event {
  fn type_(&self) -> String;
}

impl Event for JsValue {
  fn type_(&self) -> String {
    self.as_string().unwrap()
  }
}
```