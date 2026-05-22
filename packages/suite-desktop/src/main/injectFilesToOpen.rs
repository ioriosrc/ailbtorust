```rust
use electron::{Debugger, dialog};
use std::fs;

async fn inject_files_to_open(debug: Debugger, files_to_open: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if files_to_open.is_empty() {
        log::debug!("inject_files_to_open: no files - skipping");
        return Ok(());
    }

    try {
        if !debug.is_attached() {
            debug.attach("1.1")?;
        }

        let document_res = debug.send_command("DOM.getDocument")?;
        let query_res = debug.send_command("DOM.querySelector", &[
            "document",
            input_element_id,
            &["nodeId"],
        ])?;
        debug.send_command("DOM.setFileInputFiles", &[
            "input",
            query_res[0],
            &files_to_open,
        ]);

        log::debug(&format!("Set input files #{}: \n{}", input_element_id, files_to_open.join("\n")));

        // clear the files once we've opened them
        let _ = fs::remove_file(input_element_id); // Assuming a unique identifier is used for this approach

    } catch (err: Box<dyn std::error::Error>) {
        log::error(&err);
        dialog::show_error_box(
            "Internal error",
            &format!("The app encountered an internal error trying to open: {}", files_to_open.join(",")),
        );
    } finally {
        debug.detach()?;
    }

    Ok(())
}
```