```rust
type ConfigById = {
    Plot!a: {
        title: String,
        paths: Vec<Path>,
        showXAxisLabels: bool,
        showYAxisLabels: bool,
        showLegend: bool,
        legendDisplay: String,
        showPlotValuesInLegend: bool,
        isSynced: bool,
        xAxisVal: String,
        sidebarDimension: usize,
        followingViewWidth: usize
    }
};

struct Path {
    value: String,
    enabled: bool,
    timestampMethod: String
}

fn main() {
    let config_data = r#"
{
  "configById": {
    "Plot!a": {
      "title": "10 second window",
      "paths": [
        {
          "value": "sinewave_0.value",
          "enabled": true,
          "timestampMethod": "receiveTime"
        },
        {
          "value": "sinewave_1.value",
          "enabled": true,
          "timestampMethod": "receiveTime"
        },
        {
          "value": "sinewave_2.value",
          "enabled": true,
          "timestampMethod": "receiveTime"
        },
        {
          "value": "sinewave_3.value",
          "enabled": true,
          "timestampMethod": "receiveTime"
        },
        {
          "value": "sinewave_4.value",
          "enabled": true,
          "timestampMethod": "receiveTime"
        },
        {
          "value": "sinewave_5.value",
          "enabled": true,
          "timestampMethod": "receiveTime"
        }
      ],
      "showXAxisLabels": true,
      "showYAxisLabels": true,
      "showLegend": true,
      "legendDisplay": "floating",
      "showPlotValuesInLegend": false,
      "isSynced": true,
      "xAxisVal": "timestamp",
      "sidebarDimension": 240,
      "followingViewWidth": 10
    }
  },
  "globalVariables": {},
  "userNodes": {},
  "playbackConfig": {
    "speed": 1
  },
  "layout": "Plot!a"
}
"#;

    // Parse the JSON string to a Rust struct
    let config: ConfigById = serde_json::from_str(config_data).unwrap();

    println!("Parsed configuration:");
    println!("{:?}", config);
}
```

Este código não inclui todas as partes do JavaScript original, pois Rust não tem um sistema de variáveis globais ou objetos nativos como o JavaScript. Além disso, Rust não tem uma estrutura similar para `userNodes` e `playbackConfig`. Ela é mais adequada para aplicações que requerem uma abordagem funcional sem uso de classes.