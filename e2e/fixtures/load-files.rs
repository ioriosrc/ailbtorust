```rust
use playwright::{Page, Locator};

pub type LoadFilesProps = {
    mainWindow: Page,
    filenames: String | Vec<String>,
};

const PUPPETEER_FILE_UPLOAD_SELECTOR = "[data-puppeteer-file-upload]";

pub async fn load_files(props: LoadFilesProps) -> Result<(), Box<dyn std::error::Error>> {
    let files: Vec<&str> = props.filenames.into_iter().collect();
    let absolute_file_paths: Vec<String> = files.iter().map(|f| format!("assets/{}", f)).collect();

    println!("Loading file(s): {}", absolute_file_paths.join(", "));

    let file_input = props.main_window.locator(PUPPETEER_FILE_UPLOAD_SELECTOR);

    // Playwright's setInputFiles handles both single string and array
    if let Ok(file_path) = &absolute_file_paths[0] {
        await file_input.set_value(file_path);
    } else {
        for file_path in absolute_file_paths {
            await file_input.upload(file_path);
        }
    }

    Ok(())
}
```