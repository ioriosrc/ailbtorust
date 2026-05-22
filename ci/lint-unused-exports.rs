```rust
use std::path::{Path, PathBuf};
use regex::Regex;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

async fn main() -> std::io::Result<()> {
    let repo_root = PathBuf::from("..");
    let tsconfig_path = PathBuf::from("../packages/suite-base/tsconfig.json");

    let ignore_paths_regex = Regex::new(
        [
            String::from(".stories.ts?$"),
            String::from("packages/suite-base/src/index.ts"),
            String::from("packages/suite-base/src/panels/ThreeDeeRender/transforms/index.ts"), // `export *` is not correctly analyzed <https://github.com/pzavolinsky/ts-unused-exports/issues/286>
            String::from("packages/suite-base/src/test/"),
        ]
        .join("|")
    )?;

    let results = analyze_ts_config(&tsconfig_path, &["--findCompletelyUnusedFiles", "--ignoreLocallyUsed"]);
    if results.unused_exports.is_empty() && results.unused_files.is_empty() {
        return Ok(());
    }

    for (path, items) in results.unused_exports {
        let path_from_repo_root = repo_root.join(&path);
        if ignore_paths_regex.is_match(&path_from_repo_root.to_string_lossy()) {
            continue;
        }
        for item in items {
            info!(
                "::error file={}::{},line={},col={}:Unused export {} in {}",
                &path_from_repo_root.to_string_lossy(),
                item.location.line,
                item.location.character,
                item.export_name,
                path_from_repo_root.to_string_lossy()
            );
        }
    }

    for path in results.unused_files {
        let path_from_repo_root = repo_root.join(&path);
        if ignore_paths_regex.is_match(&path_from_repo_root.to_string_lossy()) {
            continue;
        }
        info!("::error file={}::{},line={},col={}:Unused file {}",
               &path_from_repo_root.to_string_lossy(),
               0,
               0,
               "",
               path_from_repo_root.to_string_lossy()
           );
    }

    Ok(())
}

fn analyze_ts_config(config_path: &Path, args: &[&str]) -> Result<unused_exports::Results, Box<dyn std::error::Error>> {
    unused_exports::analyze(config_path, args)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    main().await
}
```