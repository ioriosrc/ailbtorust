```rust
struct PaginationModel {
    page_size: u32,
    page: u32,
}

pub type ExtensionListProps = struct {
    namespace: String,
    entries: Vec<Immutable<ExtensionMarketplaceDetail>>,
    filter_text: String,
    select_extension: fn(&FocusedExtension),
};
```
No Rust, usamos a estrutura `struct` para representar os tipos de dados e o método `fn` para definir funções. Os tipos de dados são definidos como structs (por exemplo, `PaginationModel`) e as funções podem ser definidas usando closures ou implementações de interfaces (como `select_extension`).