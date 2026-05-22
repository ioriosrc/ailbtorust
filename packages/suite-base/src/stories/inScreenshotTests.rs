```rust
use std::env;

fn main() -> bool {
    env::var("USER_AGENT").map_or(false, |user_agent| user_agent.contains("PuppeteerTestingChrome") || is_chromatic())
}
```

Note: Rust does not have a built-in `isChromatic` function. The `chromatic/is-chromatic` package is used in the TypeScript/React code to determine if we are running tests on Chromatic. However, this package is not available in Rust and will need to be installed separately. Additionally, Rust's environment variable handling differs from JavaScript, so the above Rust code will need to be adjusted accordingly.