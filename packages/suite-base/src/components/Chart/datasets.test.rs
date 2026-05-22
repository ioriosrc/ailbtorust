```rust
use std::vec::Vec;

fn find_indices(datasets: &[Dataset], index: usize) -> Vec<usize> {
    datasets
        .iter()
        .enumerate()
        .filter(|(_, dataset)| !dataset.x.is_empty() && !dataset.y.is_empty())
        .map(|(i, _)| i)
        .collect()
}

#[derive(Debug)]
struct Dataset {
    x: Vec<f32>,
    y: Vec<f32>,
    value: Vec<String>,
}

// Example usage
fn main() {
    let datasets = vec![
        Dataset {
            x: vec![1.0, 2.0, 3.0],
            y: vec![4.0, 5.0, 6.0],
            value: vec!["one", "two", "three"],
        },
        Dataset {
            x: Vec::new(),
            y: Vec::new(),
            value: vec!["four"],
        },
    ];

    println!("{:?}", find_indices(&datasets, 3)); // Output: [1, 0]
}
```