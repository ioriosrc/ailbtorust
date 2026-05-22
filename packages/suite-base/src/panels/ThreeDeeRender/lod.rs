```rust
use webgl::{WebGLCapabilities, WebGLRenderingContext};

pub enum DetailLevel {
    Low,
    Medium,
    High,
}

/// Returns the number of samples used for Multi-Sample Anti-Aliasing (MSAA)
pub fn msaa_samples(capabilities: &WebGLCapabilities) -> u32 {
    capabilities.max_samples.unwrap_or(0) as u32
}

/// Returns the number of subdivisions for arrow shaft
pub fn arrow_shaft_subdivisions(lod: DetailLevel) -> u32 {
    match lod {
        DetailLevel::Low => 12,
        DetailLevel::Medium => 20,
        DetailLevel::High => 32,
    }
}

/// Returns the number of subdivisions for arrow head
pub fn arrow_head_subdivisions(lod: DetailLevel) -> u32 {
    match lod {
        DetailLevel::Low => 12,
        DetailLevel::Medium => 20,
        DetailLevel::High => 32,
    }
}

/// Returns the number of subdivisions for cylinder
pub fn cylinder_subdivisions(lod: DetailLevel) -> u32 {
    match lod {
        DetailLevel::Low => 12,
        DetailLevel::Medium => 20,
        DetailLevel::High => 32,
    }
}

/// Returns the number of subdivisions for sphere
pub fn sphere_subdivisions(lod: DetailLevel) -> u32 {
    match lod {
        DetailLevel::Low => 10,
        DetailLevel::Medium => 24,
        DetailLevel::High => 32,
    }
}
```