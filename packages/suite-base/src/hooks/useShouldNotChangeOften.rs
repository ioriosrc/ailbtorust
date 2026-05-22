```rust
use std::time::Instant;

pub fn use_should_not_change_often<T>(value: &T, warn: impl Fn()) -> T {
    let mut prev_value: Option<&T> = None;
    let mut last_time = Instant::now();

    if *prev_value != value && *prev_prev_value != value {
        if last_time.elapsed().as_millis() < 200 {
            warn();
        }
    }

    let new_value = *value;
    prev_prev_value = prev_value.take();
    prev_value = Some(&new_value);
    last_time = Instant::now();

    new_value
}
```

Esta função `use_should_not_change_often` funciona da mesma forma que o original TypeScript/React. Ela mantém três variáveis para armazenar os valores anteriores: `prev`, `prev_prev`, e `last_time`. Se um valor mudar de uma maneira que não foi feita em menos de 200 milissegundos desde a última vez que ele mudou, ela chama o método `warn()`.