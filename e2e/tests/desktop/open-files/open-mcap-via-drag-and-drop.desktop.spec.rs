```rust
use electron::{BrowserWindow, Event};
use std::path::PathBuf;

async fn load_files(main_window: &BrowserWindow, filenames: Vec<PathBuf>) {
    // Given
    for filename in filenames {
        let file_path = filename;
        mainWindow.emit("file-changed", file_path);
    }
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use electron::{app, BrowserWindow};
    use std::env;

    app.set_auto_restart(true);
    app.set_name(env!("CARGO_NAME"));
    let mainWindow = BrowserWindow::new().expect("failed to create browser window");
    let filenames: Vec<PathBuf> = env::args()
        .skip(1)
        .map(|arg| PathBuf::from(arg))
        .collect();

    load_files(&main_window, filenames).await?;

    mainWindow.on("file-changed", move |event: Event<BrowserWindow>> {
        let file_path = event.payload().unwrap();
        println!("File changed: {:?}", file_path);
    });

    main_window.load_url("https://example.com").expect("failed to load url");

    loop {
        app.run();
    }
}
```

No Rust, o código foi reescrito para usar a biblioteca `electron` e `std::path` para trabalhar com arquivos. O arquivo `.mcap` é carregado por meio da função `load_files`, que emite um evento chamado "file-changed" com o caminho do arquivo. Este evento é capturado pelo evento `"file-changed"` no objeto `main_window`.