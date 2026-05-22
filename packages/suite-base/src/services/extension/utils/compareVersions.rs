```rust
fn is_valid_part(x: &str) -> bool {
    x.chars().all(|c| c.is_digit(10))
}

pub fn compare_versions(v1: &str, v2: &str) -> i32 {
    let mut v1_parts: Vec<&str> = v1.split('.').collect();
    let mut v2_parts: Vec<&str> = v2.split('.').collect();

    if !v1_parts.iter().all(is_valid_part) || !v2_parts.iter().all(is_valid_part) {
        return std::i32::NAN;
    }

    while v1_parts.len() < v2_parts.len() {
        v1_parts.push("0");
    }
    while v2_parts.len() < v1_parts.len() {
        v2_parts.push("0");
    }

    v1_parts = v1_parts.iter().map(|&x| x.parse::<i32>().unwrap()).collect();
    v2_parts = v2_parts.iter().map(|&x| x.parse::<i32>().unwrap()).collect();

    for i in 0..v1_parts.len() {
        let part1 = v1_parts[i];
        let part2 = v2_parts[i];

        if part1 == part2 {
            continue;
        } else if part1 > part2 {
            return 1;
        } else {
            return -1;
        }
    }

    0
}
```