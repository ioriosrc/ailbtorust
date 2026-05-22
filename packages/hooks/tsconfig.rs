```json
{
  "name": "typescript-react-functional",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "build": "tsc"
  },
  "dependencies": {
    "@lichtblick/tsconfig/base": "^1.x",
    "react": "^18.x",
    "react-dom": "^18.x",
    "@types/react": "^18.x",
    "@types/node": "^20.3.0"
  }
}
```

```rust
// src/main.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() {
    console.log("Hello from Rust!");
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    console.log!("Starting the application...");
    Ok(())
}
```

```javascript
// package.json
{
  "name": "typescript-react-functional",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "build": "tsc",
    "start": "node dist/index.js"
  },
  "dependencies": {
    "@lichtblick/tsconfig/base": "^1.x",
    "react": "^18.x",
    "react-dom": "^18.x",
    "@types/react": "^18.x",
    "@types/node": "^20.3.0"
  }
}
```