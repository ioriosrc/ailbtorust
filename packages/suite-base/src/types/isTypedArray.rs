```rust
fn is_typed_array<T>(value: T) -> bool {
    use std::any::TypeId;
    TypeId::of::<T>() == TypeId::of::<Int8Array>()
        || TypeId::of::<Uint8Array>()
        || TypeId::of::<Int16Array>()
        || TypeId::of::<Uint16Array>()
        || TypeId::of::<Int32Array>()
        || TypeId::of::<Uint32Array>()
        || TypeId::of::<Float32Array>()
        || TypeId::of::<Float64Array>()
        || TypeId::of::<BigInt64Array>()
        || TypeId::of::<BigUint64Array>()
        || TypeId::of::<Uint8ClampedArray>()
}
```