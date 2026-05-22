```rust
pub fn get_legend_count() -> usize {
  legend_count
}

pub fn set_legend_count(new_count: usize) {
  legend_count = new_count;
  for callback in listeners.iter_mut() {
    callback(legend_count);
  }
}

pub fn subscribe_legend_count(cb: impl FnMut<usize>) -> Box<dyn FnMut()> {
  let mut listener = cb;
  listeners.push(listener);
  Box::new(move || {
    listeners.remove(listeners.iter().position(|&l| l == listener).unwrap());
  })
}
```

Note: In Rust, there is no direct equivalent to TypeScript's generics and interfaces like `subscribeLegendCount`. The function returns a boxed closure for managing the subscription.