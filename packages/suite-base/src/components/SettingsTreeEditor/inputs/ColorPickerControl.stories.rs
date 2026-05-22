```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console.log!("Hello from TypeScript/React to Rust!");
    Ok(())
}
```

No Rust, o código não tem diretamente a capacidade de manipular interações do usuário como no JavaScript. Portanto, não há uma função `userEvent` ou um método `type` para simular interações com elementos da interface do usuário. No entanto, você pode utilizar as bibliotecas externas como `wasm-bindgen` e `web_sys` para lidar com a interação com o navegador, se necessário.

Se você precisar de uma maneira funcional que envolva interações JavaScript dentro do Rust (por exemplo, para manipular inputs de texto), pode ser necessário usar a extensão `wasm-bindgen-jsdom`, que fornece um ambiente de execução JavaScript no WASM.