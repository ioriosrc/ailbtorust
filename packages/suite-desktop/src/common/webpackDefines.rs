```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProductInfo {
    product_name: String,
    version: String,
    homepage: String,
}

fn main() {
    let product_info = ProductInfo {
        product_name: LICHTBLICK_PRODUCT_NAME.to_string(),
        version: LICHTBLICK_PRODUCT_VERSION.to_string(),
        homepage: LICHTBLICK_PRODUCT_HOMEPAGE.to_string(),
    };

    println!("Product Name: {}", product_info.product_name);
    println!("Version: {}", product_info.version);
    println!("Homepage: {}", product_info.homepage);
}
```