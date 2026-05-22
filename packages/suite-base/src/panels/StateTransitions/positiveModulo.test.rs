```rust
fn positive_modulo(mut n: isize, modulus: usize) -> isize {
    while n < 0 {
        n += modulus;
    }
    return n % modulus;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_a_positive_value_between_0_inclusive_and_modulus_exclusive() {
        assert_eq!(positive_modulo(0, 10), 0);
        assert_eq!(positive_modulo(10, 10), 0);
        assert_eq!(positive_modulo(11, 10), 1);
        assert_eq!(positive_modulo(21, 10), 1);
        assert_eq!(positive_modulo(-1, 10), 9);
        assert_eq!(positive_modulo(-11, 10), 9);
    }
}
```