```rust
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Immutable<T>(T);

impl From<T> for Immutable<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

pub type AnyArray<T = ()> = Vec<T>;

pub type Builtin = Primitive | Box<dyn std::error::Error + 'static> | Box<dyn std::fmt::Debug + 'static> | Box<dyn std::io::Read + 'static> | Box<dyn std::io::Write + 'static> | Box<dyn std::net::TcpStream + 'static> | Box<dyn std::net::UdpSocket + 'static>;

pub type IsTuple<T> = bool;

pub type IsUnknown<T> = bool;

fn is_any<T>(_: T) -> bool {
    false
}

#[derive(Debug)]
pub struct Map<K, V>(HashMap<K, V>);

impl From<HashMap<K, V>> for Map<K, V> {
    fn from(value: HashMap<K, V>) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Set<T>(HashSet<T>);

impl From<HashSet<T>> for Set<T> {
    fn from(value: HashSet<T>) -> Self {
        Self(value)
    }
}
```