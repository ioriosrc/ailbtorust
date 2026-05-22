```rust
use std::vec::Vec;

// Define the vector struct to match TypeScript's Vec<[number]>
pub struct Vector {
    pub components: Vec<f64>,
}

impl Vector {
    fn new(components: Vec<f64>) -> Self {
        Self { components }
    }

    fn dot(&self, other: &Vector) -> f64 {
        self.components.iter().zip(other.components.iter()).map(|(a, b)| a * b).sum()
    }

    fn cross(&self, other: &Vector) -> Vector {
        let [ax, ay, az] = &self.components;
        let [bx, by, bz] = &other.components;

        let cx = (ay * bz - az * by);
        let cy = (az * bx - ax * bz);
        let cz = (ax * by - ay * bx);

        Vector::new(vec![cx, cy, cz])
    }

    fn vector_addition(&self, other: &Vector) -> Vector {
        let [x1, y1, z1] = &self.components;
        let [x2, y2, z2] = &other.components;

        Vector::new(vec![x1 + x2, y1 + y2, z1 + z2])
    }

    fn rotate(&self, rotation: &Vector) -> Vector {
        let [rx, ry, rz] = &rotation.components;
        let [x, y, z] = &self.components;

        let cos_theta = rx * x + ry * y + rz * z;
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        Vector::new(vec![
            x * cos_theta + y * (-rz) * sin_theta + z * ry * sin_theta,
            x * ry * sin_theta + y * cos_theta - z * rx * sin_theta,
            x * rz * sin_theta - y * ry * sin_theta + z * cos_theta,
        ])
    }
}

fn main() {
    let vec1 = Vector::new(vec![2.0, 4.0]);
    let vec2 = Vector::new(vec![1.0, 2.0, 3.0, 4.0]);

    println!("Dot product: {}", vec1.dot(&vec2));
    println!("Cross product: {:?}", vec1.cross(&vec2));

    let addition_result = vec1.vector_addition(&vec2);
    println!("Vector addition: {:?}", addition_result);

    let rotation = Vector::new(vec![1.0, 1.0, 1.0]);
    let rotation_result = vec1.rotate(&rotation);
    println!("Rotation result: {:?}", rotation_result);
}
```