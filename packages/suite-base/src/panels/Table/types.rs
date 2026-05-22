```rust
use serde_json::Value as JsonValue;

pub type CellValue = Value;
pub type MergedColumnsType = Vec<MergedColumnDef<CellValue>>;
```

Para compreender o código Rust convertido:

- `AccessorKeyColumnDef` e `DisplayColumnDef` são tipos que definem a estrutura de dados para colunas na tabela.
- `JsonValue` é usada para representar valores JSON, como os contêineres `Record<string, unknown>` em TypeScript.
- `CellValue` é um tipo abstrato genérico que pode conter qualquer valor JSON.
- `MergedColumnsType` é uma lista de `MergedColumnDef<CellValue>`, que representa uma coluna mista que pode combinar diferentes tipos de dados.