```rust
fn toggle<T>(array: &mut Vec<T>, item: T, predicate: impl Fn(&T) -> bool) {
    array.retain(|&x| !predicate(&x));
    if predicate(&item) {
        array.push(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_shallow_equality_by_default() {
        let mut items = vec! [{ foo: "bar" }, { foo: "baz" }];
        let arr = toggle(&mut items, &items[0], |_| true);
        assert_eq!(arr, vec![{ foo: "baz" }]);
        assert_eq!(items, vec! [{ foo: "bar" }, { foo: "baz" }]);
    }

    #[test]
    fn removes_item_if_predicate_returns_true_for_it() {
        let mut items = vec! [{ foo: "bar" }, { foo: "baz" }];
        let arr = toggle(&mut items, &{ foo: "bar" }, |item| item.foo == "bar");
        assert_eq!(arr, vec![{ foo: "baz" }]);
        assert_eq!(items, vec! [{ foo: "bar" }, { foo: "baz" }]);
    }

    #[test]
    fn adds_item_if_predicate_returns_false_for_everything() {
        let mut items = vec! [{ foo: "bar" }, { foo: "baz" }];
        let arr = toggle(&mut items, &{ foo: "bar" }, |_| false);
        assert_eq!(arr, vec![&{ foo: "bar" }, &{ foo: "baz" }, &{ foo: "bar" }]);
        assert_eq!(items, vec! [{ foo: "bar" }, { foo: "baz" }, &{ foo: "bar" }]);
    }
}
```