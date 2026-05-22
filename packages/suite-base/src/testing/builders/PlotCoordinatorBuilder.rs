```rust
use std::collections::{HashMap, HashSet};

pub struct PlotCoordinatorBuilder {
    series_keys_by_topic: HashMap<String, HashSet<SeriesConfigKey>>,
}

impl PlotCoordinatorBuilder {
    pub fn new() -> Self {
        Self {
            series_keys_by_topic: HashMap::new(),
        }
    }

    pub fn series_keys_by_topic(
        entries: &[(String, Vec<&str>)],
        timestamp_method: TimestampMethod = TimestampMethod::ReceiveTime,
    ) -> Self {
        let mut builder = Self::new();
        let mut config_index = 0;
        for (topic, paths) in entries {
            let start_index = config_index;
            let keys = HashSet::from(paths.iter().map(|path| {
                format!(
                    "{}:{}:{}",
                    config_index + i,
                    timestamp_method.as_str(),
                    path
                )
            }));
            config_index += paths.len();
            builder.series_keys_by_topic.insert(topic.to_string(), keys);
        }
        builder
    }

    pub fn build(&self) -> &HashMap<String, HashSet<SeriesConfigKey>> {
        &self.series_keys_by_topic
    }
}
```

**Observações**:
1. A implementação do Rust usa `HashMap` para armazenar as configurações de séries por tópico.
2. O código é mais claro e fácil de entender que o TypeScript original, utilizando `Set` em vez de `Array` para manter os índices sequenciais.
3. Os métodos `new`, `series_keys_by_topic`, e `build` foram adicionados ao tipo `PlotCoordinatorBuilder`.
4. A função `format!` é usada para construir as chaves dos dicionários, garantindo a consistência das strings.