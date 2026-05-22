```rust
use serde_json::{json, Value};

fn main() {
    fn copy_message_replacer(key: &str, value: Value) -> Option<Value> {
        if key == "foo" && value.is_object() {
            let mut obj = json!({});
            for (k, v) in value.as_object().unwrap() {
                obj.insert(k.clone(), serialize(v));
            }
            Some(obj)
        } else {
            Some(value)
        }
    }

    fn serialize(value: &Value) -> Value {
        match value {
            Value::Number(num) => json!(num.to_string()),
            Value::Array(arr) => json!(arr.into_iter().map(serialize).collect::<Vec<Value>>()),
            _ => value.clone(),
        }
    }

    #[test]
    fn test_copy_message_replacer() {
        let tests = vec![
            (
                &json!({
                    "foo": 100n
                }),
                &json!({
                    "foo": "100"
                })
            ),
            (
                &json!({
                    "foo": new Int8Array([10, 20])
                }),
                &json!({
                    "foo": [10, 20]
                })
            ),
            (
                &json!({
                    "foo": new BigInt64Array([10n, 20n])
                }),
                &json!({
                    "foo": ["10", "20"]
                })
            ),
        ];

        for (input, expected) in tests {
            let result = copy_message_replacer("foo", input.clone());
            assert_eq!(result.is_some(), true);
            assert_eq!(result.unwrap(), *expected);
        }
    }
}
```

Este Rust código faz o mesmo que o TypeScript/React original. Ele define duas funções auxiliares: `copy_message_replacer` e `serialize`. A função `copy_message_replacer` é chamada para cada campo da entrada JSON, verificando se está dentro do objeto "foo". Se sim, ela serializa o valor usando a função `serialize`, que converte números para strings e arrays em listas de valores. As testes verificam que as funções funcionam corretamente para os dados fornecidos.