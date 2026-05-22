```rust
use notistack::{enqueueSnackbar};
use path::PathBuf;
use wasm_bindgen::JsValue;

async fn open_files(sources: &[DataSourceFactory]) -> Result<(), JsValue> {
    let { select_source } = use_player_selection();
    let throwError_and_snackbar = |message| {
        enqueue Snackbar(message, { variant: "error" });
        Err(JsValue::from_str("Error").unwrap())
    };

    let all_extensions = sources
        .iter()
        .flat_map(|source| source.supported_file_types.as_ref().map(|types| types.iter()))
        .flatten()
        .collect::<Vec<&str>>();

    let normalize_extensions_array = normalize_extensions(all_extensions);

    let files_handle = show_open_file_picker({
        multiple: true,
        types: [
            {
                description: normalize_extensions_array.join(", "),
                accept: {
                    [FILE_ACCEPT_TYPE]: normalize_extensions_array.as_slice(),
                },
            },
        ],
    })?;

    if files_handle.len() == 0 {
        return Ok(());
    }

    let processed_files = files_handle
        .iter()
        .map(|handle| handle.file())
        .await
        .collect::<Result<Vec<JsValue>, JsValue>>()?;

    let unique_extensions = processed_files
        .iter()
        .map(|file| PathBuf::from(file.as_ref().as_js_string().unwrap()).extension())
        .collect::<HashSet<&str>>();

    if unique_extensions.len() > 1 {
        throwError_and_snackbar("Multiple file extensions detected. All files must have the same extension.");
    }

    let [extension] = unique_extensions;

    let matching_sources = sources
        .iter()
        .filter(|source| {
            source.supported_file_types.as_ref().map_or(false, |types| types.contains(extension))
                && source.type == "file"
        })
        .collect::<Vec<&DataSourceFactory>>();

    if matching_sources.len() == 0 {
        throwError_and_snackbar(format!("Cannot find a source to handle files with extension {}", extension));
    }

    if matching_sources.len() > 1 {
        throwError_and_snackbar(format!(
            "The file extension \"{}\" is not supported. Please select files with the following extensions: {}.",
            extension,
            all_extensions.join(", ")
        ));
    }

    /**
     * Should be removed when implement the rest of extensions.
     */
    if extension != ".mcap" && processed_files.len() > 1 {
        throwError_and_snackbar("The application only support multiple files for MCAP extension.");
    }
    select_source(matching_sources[0].id, {
        type: "file",
        handles: files_handle,
    });

    Ok(())
}
```