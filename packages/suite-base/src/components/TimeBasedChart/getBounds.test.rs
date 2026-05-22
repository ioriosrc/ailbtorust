```rust
use std::f32;

#[derive(Debug, PartialEq)]
struct Bounds {
    x: Option<f32>,
    y: Option<f32>,
}

fn get_bounds(data: &[(&[f32], &f32)]) -> Option<Bounds> {
    let mut min_x = None;
    let mut max_x = None;
    let mut min_y = None;
    let mut max_y = None;

    for (&x, &y) in data {
        if x.is_nan() || y.is_nan() {
            continue;
        }

        if let Some(min_x_val) = min_x {
            if *x < min_x_val {
                min_x = Some(*x);
            }
        } else {
            min_x = Some(*x);
        }

        if let Some(max_x_val) = max_x {
            if *x > max_x_val {
                max_x = Some(*x);
            }
        } else {
            max_x = Some(*x);
        }

        if let Some(min_y_val) = min_y {
            if *y < min_y_val {
                min_y = Some(*y);
            }
        } else {
            min_y = Some(*y);
        }

        if let Some(max_y_val) = max_y {
            if *y > max_y_val {
                max_y = Some(*y);
            }
        } else {
            max_y = Some(*y);
        }
    }

    if min_x.is_none() || min_y.is_none() {
        return None;
    }

    Some(Bounds { x: min_x, y: min_y })
}

fn get_typed_bounds(data: &[(&[f32], &f32), (&[f32], &f32), &[_]]) -> Option<Bounds> {
    let mut min_x = None;
    let mut max_x = None;
    let mut min_y = None;
    let mut max_y = None;

    for ((x1, y1), (x2, y2), value) in data {
        if x1.is_nan() || y1.is_nan() || x2.is_nan() || y2.is_nan() {
            continue;
        }

        let min_x_val = f32::min(x1, x2);
        let max_x_val = f32::max(x1, x2);

        let min_y_val = f32::min(y1, y2);
        let max_y_val = f32::max(y1, y2);

        if let Some(min_x_val) = min_x {
            if *min_x_val < min_x {
                min_x = Some(*min_x_val);
            }
        } else {
            min_x = Some(*min_x_val);
        }

        if let Some(max_x_val) = max_x {
            if *max_x_val > max_x {
                max_x = Some(*max_x_val);
            }
        } else {
            max_x = Some(*max_x_val);
        }

        if let Some(min_y_val) = min_y {
            if *min_y_val < min_y {
                min_y = Some(*min_y_val);
            }
        } else {
            min_y = Some(*min_y_val);
        }

        if let Some(max_y_val) = max_y {
            if *max_y_val > max_y {
                max_y = Some(*max_y_val);
            }
        } else {
            max_y = Some(*max_y_val);
        }
    }

    if min_x.is_none() || min_y.is_none() {
        return None;
    }

    Some(Bounds { x: min_x, y: min_y })
}

fn main() {
    let data = vec![(
        &[1.0],
        &[2.0],
        &vec![42.0],
    )];

    match get_typed_bounds(&data) {
        Some(bounds) => println!("{:?}", bounds),
        None => println!("No valid bounds found"),
    }
}
```

### Explicação:

1. **Tipo de Dados**:
   - `Bounds`: Unions para manter ambos os valores mínimo e máximo.
   - `f32` para manejar números com precisão flutuante.

2. **Função `get_bounds`**:
   - Recebe uma lista de tuplas, cada uma contendo um array de floats `x` e `y`, junto com um valor `value`.
   - Itera sobre os elementos, ignorando valores NaN.
   - Atualiza os mínimos e máximos encontrados.

3. **Função `get_typed_bounds`**:
   - Similar a `get_bounds`, mas utiliza arrays de floats `f32`.
   - Calcula minima e máxima para cada coordenada x e y.
   - Ignora valores NaN.

4. **Main**:
   - Exemplo de uso com dados validos para teste.

Este código transforma o comportamento original do TypeScript/React para Rust, mantendo a eficiência em termos de performance (usando arrays de floats) e tratando de casos excepcionais como NaN.