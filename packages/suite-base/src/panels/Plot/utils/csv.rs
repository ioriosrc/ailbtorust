```rust
use std::fs::{self};
use std::io::{BufWriter, Write};

fn get_csv_row(label: &str, data: &CsvDataset) -> Vec<&str> {
    let x = data.x();
    let receive_time_float = format!("{}{}", data.receive_time(), data.receive_time().format("%s"));
    let stamp_time = if data.header_stamp() != 0.0 {
        format!("{}{}", data.header_stamp(), data.header_stamp().format("%s"))
    } else {
        ""
    };
    vec![x, &receive_time_float, &stamp_time, label, &data.value()]
}

fn get_csv_col_name(x_axis_val: PlotXAxisVal) -> &'static str {
    match xAxis_val {
        PlotXAxisVal::Custom | PlotXAxisVal::CurrentCustom => "x value",
        PlotXAxisVal::Index => "index",
        PlotXAxisVal::Timestamp => "elapsed time",
    }
}

fn generate_csv(datasets: Vec<CsvDataset>, xAxis_val: PlotXAxisVal) -> String {
    let head_line = vec![
        get_csv_col_name(x_axis_val),
        "receive time",
        "header.stamp",
        "topic",
        "value",
    ];
    let combined_lines: Vec<&str> = vec![head_line];

    for dataset in datasets {
        for datum in &dataset.data {
            combined_lines.push(get_csv_row(dataset.label(), datum));
        }
    }

    combined_lines.join("\n")
}

fn download_csv(
    filename: &str,
    datasets: Vec<CsvDataset>,
    xAxis_val: PlotXAxisVal,
) {
    let csv_data = generate_csv(datasets, xAxis_val);
    let mut file = BufWriter::new(fs::File::create(filename).unwrap());

    writeln!(file, "{}", csv_data).expect("Failed to write CSV data");
}
```