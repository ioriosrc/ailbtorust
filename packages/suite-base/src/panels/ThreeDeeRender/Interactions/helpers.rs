```rust
fn clamp_tooltip_axis(
  cursor: f64,
  size: f64,
  bounds_start: f64,
  bounds_end: f64,
  offset: f64,
) -> f64 {
  if (bounds_end - cursor).abs() >= size + offset {
    cursor + offset
  } else if (cursor - bounds_start).abs() >= size + offset {
    cursor - size - offset
  } else {
    std::cmp::max(bounds_start, bounds_end - size)
  }
}
```