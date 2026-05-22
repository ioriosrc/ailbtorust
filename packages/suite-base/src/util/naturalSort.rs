```rust
use std::cmp::Ordering;

fn natural_sort<T>(key: Option<&str>) -> impl Fn(&T, &T) -> Ordering {
    let sort_key = key.map(|k| k.to_lowercase());
    move |a, b| {
        if sort_key.is_none() {
            a.partial_cmp(b).unwrap()
        } else {
            a.get(key.unwrap()).cmp(&b.get(key.unwrap())).unwrap()
        }
    }
}
```