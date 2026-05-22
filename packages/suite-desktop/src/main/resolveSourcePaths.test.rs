```rust
use std::collections::HashSet;
use std::fs::{self, DirEntry, File};
use std::io::{ErrorKind, Read};

fn generic_string() -> String {
    random_string::generate({
        length: 6,
        charset: "alphanumeric",
    })
}

#[derive(Debug)]
struct FileStructure {
    name: String,
    content: String,
}

fn build_file(file: Option<&FileStructure>) -> FileStructure {
    let file = file.unwrap_or(&FileStructure {
        name: generic_string(),
        content: generic_string(),
    });
    FileStructure {
        name: format!("{}.{}", file.name, file.extension.unwrap_or("mcap")),
        content: file.content.clone(),
    }
}

fn build_path() -> String {
    generic_string()
}

fn setup(fs_config_override: Option<&HashMap<String, FileStructure>>) -> (String, HashMap<String, FileStructure>) {
    let mut path = build_path();
    let file1 = build_file(Some(&FileStructure {
        extension: Some("txt".to_string()),
    }));
    let file2 = build_file(Some(&FileStructure {
        extension: Some("txt".to_string()),
    }));
    let file3 = build_file(Some(&FileStructure {
        extension: Some("mcap".to_string()),
    }));
    let files = if let Some(fs_config) = fs_config_override {
        fs_config
    } else {
        HashMap::from([
            (path.clone(), file1.clone()),
            (path.clone(), file2.clone()),
            (path.clone(), file3.clone()),
        ])
    };
    (fs_config, path)
}

fn get_files_from_directory(directory: &str) -> Vec<String> {
    fs::read_dir(directory)
        .expect("Failed to read directory")
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let metadata = entry.metadata().expect("Failed to read metadata");
            if metadata.is_file() && entry.path().extension().and_then(|ext| ext.to_str()).map_or(false, |ext| ext.ends_with(".mcap")) {
                Some(entry.path().file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect()
}

fn is_path_to_directory(paths: &[String]) -> bool {
    paths.len() == 1 && fs::metadata(&paths[0]).is_ok() && fs::read_dir(&paths[0])
        .expect("Failed to read directory")
        .all(|entry| entry.ok().map_or(false, |entry| entry.path().is_dir()))
}

fn resolve_source_paths(source_parameter: Option<&str>) -> Vec<String> {
    let homedir = dirs::home_dir().unwrap();
    let mut paths = source_parameter.map_or(vec![], |p| p.split(',').collect());
    for path in &mut paths {
        let mut parts = path.split('/');
        let base_path = if parts.len() > 2 { &parts[0] } else { homedir.to_str().unwrap() };
        parts.remove(0);
        let dir_path = fs::canonicalize(format!("{}/{}", base_path, parts.join("/"))).expect("Failed to resolve path");
        paths.push(dir_path.to_string_lossy().into_owned());
    }
    paths
}
```