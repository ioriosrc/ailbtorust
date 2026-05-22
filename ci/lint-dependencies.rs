```rust
use depcheck::{Detector, Dependency};
use glob::glob;
use std::{
    fs::{self},
    io::{Read, Write},
};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

#[derive(Debug)]
struct CommentDetector;
impl Detector for CommentDetector {
    fn detect(&self, node: &Dependency) -> Vec<String> {
        let value = match node.value.as_ref() {
            "CommentBlock" | "CommentLine" => &node.value,
            _ => return vec![],
        };
        value
            .split('\n')
            .filter(|line| line.starts_with("// foxglove-depcheck-used: "))
            .map(|line| {
                let parts = line.split(':').collect::<Vec<&str>>();
                if parts.len() == 2 {
                    parts[1].trim().to_string()
                } else {
                    "".to_string()
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct TripleSlashDetector;
impl Detector for TripleSlashDetector {
    fn detect(&self, node: &Dependency) -> Vec<String> {
        let value = match node.value.as_ref() {
            "CommentLine" => &node.value,
            _ => return vec![],
        };
        value.split('/').filter(|line| line.starts_with("// <reference types=")).collect::<Vec<&str>>()
    }
}

async fn run(root_path: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Linting dependencies in {}", root_path);
    let options = depcheck::Options {
        detectors: vec![
            DependencyDetector,
            CommentDetector {},
            TripleSlashDetector {},
        ],
        ignore_matches: ["~".to_string()],
    };
    depcheck::check(root_path, options)?;
    Ok(())
}

fn print_and_analyze_results(unused: &depcheck::Results, package_name: &str) -> bool {
    let had_error = false;

    if !unused.dev_dependencies.is_empty() {
        had_error = true;

        error!("Unused devDependencies in {}: {:?}", package_name, unused.dev_dependencies);
        for dep in unused.dev_dependencies {
            info!("- {}", dep);
        }
        info("");
    }

    if !unused.dependencies.is_empty() {
        had_error = true;
        error!("Unused dependencies in {}: {:?}", package_name, unused.dependencies);
        for dep in unused.dependencies {
            info!("- {}", dep);
        }
        info("");
    }

    // Don't consider the package itself to be a missing dep
    // https://github.com/depcheck/depcheck/issues/564
    if !unused.missing.is_empty() {
        had_error = true;
        error!("Missing dependencies in {}: {:?}", package_name, unused.missing);
        for (dep, locations) in unused.missing.iter().filter(|(_, locations)| !locations.is_empty()) {
            info!("- {} (used in {:?})", dep, locations[0]);
        }
        info("");
    }

    if !unused.invalid_files.is_empty() {
        had_error = true;
        error!("Invalid files in {}: {:?}", package_name, unused.invalid_files);
        for (file_path, err) in unused.invalid_files.iter().filter(|(_, err)| !err.is_empty()) {
            info!("- {} ({})", file_path, err);
        }
        info("");
    }

    if !unused.invalid_dirs.is_empty() {
        had_error = true;
        error!("Invalid directories in {}: {:?}", package_name, unused.invalid_dirs);
        for (dir_path, err) in unused.invalid_dirs.iter().filter(|(_, err)| !err.is_empty()) {
            info!("- {} ({})", dir_path, err);
        }
        info("");
    }

    if !had_error {
        info!("No missing or unused dependencies in {}", package_name);
        info("");
    }

    !had_error
}

async fn get_all_workspace_packages(roots: Vec<&str>) -> Result<Vec<depcheck::Result>, Box<dyn std::error::Error>> {
    let results = vec![];
    for root in roots {
        let workspace_info = fs::read_to_string(path.join(root, "package.json"))?;
        let workspace_json: serde_json::Value = serde_json::from_str(&workspace_info)?;
        let patterns: Vec<&str> = match workspace_json.get("workspaces") {
            Some(json) => json.as_array().unwrap_or_default(),
            None => match workspace_json.get("workspaces").and_then(|json| json.get("packages")) {
                Some(json) => json.as_array().unwrap_or_default(),
                None => Vec::new(),
            },
        };

        let cwd = path.join(root);
        for pattern in patterns {
            let matches = glob(pattern)?;
            for package_path in matches {
                if !fs::metadata(&package_path)?.is_dir() {
                    continue;
                }
                let abs_path = package_path.to_string_lossy().into_owned();
                let package_json_path = path.join(abs_path, "package.json");
                if !fs::try_exists(package_json_path).unwrap_or_default() {
                    info!("Skipping {} (no package.json)", &abs_path);
                    continue;
                }
                results.push(run(abs_path));
            }
        }
    }

    Promise.all(results).await?;
    Ok(results)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let roots: Vec<String> = std::env::args().skip(1).collect();
    if roots.is_empty() {
        error!("Usage: lint-dependencies [workspace1] [workspace2] ...");
        process::exit(1);
    }
    info!("Linting dependencies in workspaces: {:?}", roots);

    let packages = get_all_workspace_packages(roots).await?;
    let had_error = packages.into_iter().any(|result| result.is_err());

    if had_error {
        info(
            "NOTE: Dependencies can be marked explicitly used with a comment, e.g.:\n  // foxglove-depcheck-used: foo-package",
        );
        process::exit(1);
    } else {
        info!("No errors!");
    }

    Ok(())
}
```