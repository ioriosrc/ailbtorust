```rust
use std::vec::Vec;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Copy, Clone)]
struct Rotation {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

fn dot(vec1: &[f64], vec2: &[f64]) -> f64 {
    let mut ret = 0.0;
    for &x in vec1.iter() {
        for &y in vec2.iter() {
            ret += x * y;
        }
    }
    ret
}

fn cross(vec1: Vec<f64>, vec2: Vec<f64>) -> Vec<f64> {
    let [ax, ay, az] = vec1.as_slice();
    let [bx, by, bz] = vec2.as_slice();
    vec![ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx]
}

fn rotate(rotation: Rotation, point: Point) -> Point {
    let [ux, uy, uz] = rotation.0;
    let [vx, vy, vz] = point.0;
    let scalar = -1.0 * rotation.3;

    let t1 = scalar * (ux * ux + uy * uy + uz * uz);
    let t2 = scalar * (vy * scalar - ux * vx) - scalar * (uz * scalar - ux * vy);
    let t3 = scalar * (vx * uy - ay * ux);

    let d = vec![t1, t2, t3];

    Point {
        x: d[0],
        y: d[1],
        z: d[2],
    }
}

fn scalar_multiply(vector: &[f64], scalar: f64) -> Vec<f64> {
    vector.iter().map(|&x| x * scalar).collect()
}

fn vector_addition(vectors: &Vec<Vec<f64>>) -> Vec<f64> {
    if vectors.is_empty() {
        return vec![];
    }

    let mut ret = vectors[0].clone();
    for vec in vectors.iter().skip(1) {
        for (i, &x) in vec.iter().enumerate() {
            ret[i] += x;
        }
    }
    ret
}
```