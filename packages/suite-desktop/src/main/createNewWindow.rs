```rust
use std::env;
use log::{debug, error};
use crate::studio_window::StudioWindow;
use crate::file_utils::{is_file_to_open, get_files_to_open};
use crate::inject_files_to_open;

fn create_new_window(argv: Vec<String>) {
    let deep_links = argv[1..].iter().filter(|arg| arg.starts_with("lichtblick://")).cloned().collect::<Vec<_>>();
    let files: Vec<String> = argv[1..].iter()
        .filter(|arg| !arg.starts_with("--")) // Filter out flags
        .filter(|arg| !arg.starts_with("lichtblick://")) // Filter out deep links
        .filter(|arg| is_file_to_open(arg))
        .cloned()
        .collect::<Vec<_>>();

    debug!("Files extracted from second instance: {:?}", files);

    let mut new_window = StudioWindow::new(&deep_links);
    let files_to_open = get_files_to_open(&argv);

    // Wait for the window to be ready before injecting files
    new_window.get_browser_window().web_contents.once("did-finish-load", move || {
        if !files_to_open.is_empty() {
            debug!("Injecting files into new window: {:?}", files_to_open);
            inject_files_to_open(&new_window.get_browser_window().web_contents.debugger(), &files_to_open)
                .map_err(|err| error!("Failed to inject files: {}", err))
                .expect("Injected files successfully");
        }
    });

    new_window.load();
}
```

Note: This is a simplified version of the original TypeScript/React code and assumes that `StudioWindow`, `fileUtils`, `getFilesToOpen`, and `injectFilesToOpen` are defined elsewhere in your Rust project.