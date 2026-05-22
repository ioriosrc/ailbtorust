```rust
use std::cmp;

#[derive(Debug)]
struct Immutable<T>(T);

pub type Bounds1D = Immutable<f64>;

/**
 * Describes the limits of a rectangular area in 2d space.
 */
#[derive(Debug)]
pub struct Bounds {
    x: Bounds1D,
    y: Bounds1D,
}

/**
 * Return the union of two 1D bounds
 */
fn union_bounds1D(a: &Bounds1D, b: &Bounds1D) -> Immutable<Bounds1D> {
    Immutable(cmp::min(a.0, b.0))
}

/**
 * Update the bounds to include the value. The bounds are updated in-place. Returns the same bounds
 * object.
 */
fn extend_bounds1D(bounds: &mut Bounds, value: f64) {
    if bounds.x.0 > value {
        bounds.x.0 = value;
    }
    if bounds.y.0 > value {
        bounds.y.0 = value;
    }
}
```