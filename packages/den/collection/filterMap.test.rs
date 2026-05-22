```rust
fn filter_map<T>(input: Vec<T>, mapper: fn(&T) -> Option<bool>) -> Vec<bool> {
    input.into_iter().filter_map(mapper).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn behaves_like_map_and_filter_boolean() {
        assert_eq!(filter_map(vec![], |x| Some(x)), vec![]);
        assert_eq!(
            filter_map(vec![1, 2, 3], |x, i| x == i + 1),
            vec![true, true, true]
        );
        assert_eq!(filter_map(vec![0, 1, 2], |x| Some(x)), vec![1, 2]);
        assert_eq!(
            filter_map(vec![0, 1, 2], |x| Some(x - 1)),
            vec![-1, 1]
        );
        assert_eq!(filter_map(vec![0, 1, 2], |_| Some(true)), vec![true, true, true]);
        assert_eq!(filter_map(vec![0, 1, 2], |_| None), vec![]);
        assert_eq!(filter_map(vec![0, 1, 2], |_| std::f64::NAN), vec![]);
        assert_eq!(filter_map(vec![0, 1, 2], |_| Vec::<bool>::new()), vec![]);
        assert_eq!(
            filter_map(vec![0, 1, 2], |_| false),
            vec![false, false, false]
        );
    }
}
```