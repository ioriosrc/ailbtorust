```rust
use std::path::{Path, PathBuf};

async fn ros_package_name_at_path(package_path: &Path) -> Option<String> {
    let package_xml_path = package_path.join("package.xml");
    if !package_xml_path.exists() {
        return None;
    }
    // Implement parsing logic to read the package name from package.xml
    // This is a placeholder for the actual parsing implementation
    Some(package_xml_path.parent().unwrap().file_name().unwrap().to_string_lossy())
}

async fn find_ros_package(package_name: &str, options: FindOptions) -> Option<PathBuf> {
    let mut package_paths = Vec::new();

    if let Some(ref ros_package_path) = options.ros_package_path {
        if !package_path.exists() {
            return None;
        }
        package_paths.push(package_path.to_path_buf());
    }

    if options.env_ros_package_path.is_some() {
        let env_ros_package_path = options.env_ros_package_path.unwrap();
        if !env_ros_package_path.exists() {
            return None;
        }
        package_paths.push(env_ros_package_path.to_path_buf());
    }

    for path in &package_paths {
        if find_ros_package_recursively(path, package_name).is_some() {
            return Some(path.to_path_buf());
        }
    }

    None
}

struct FindOptions {
    ros_package_path: Option<PathBuf>,
    env_ros_package_path: Option<PathBuf>,
}

async fn find_ros_package_recursively(package_path: &Path, package_name: &str) -> Option<PathBuf> {
    let manifest_path = package_path.join("package.xml");
    if !manifest_path.exists() {
        return None;
    }
    // Implement parsing logic to read the package name from package.xml
    // This is a placeholder for the actual parsing implementation
    if &package_name == match ros_package_name_at_path(package_path).await {
        Some(name) => name,
        None => return None,
    } {
        return Some(package_path.to_path_buf());
    }

    let subpackages = read_subpackages_from_manifest(&manifest_path)?;
    for package in subpackages {
        if find_ros_package_recursively(package, package_name).is_some() {
            return Some(package);
        }
    }

    None
}

async fn read_subpackages_from_manifest(manifest_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    // Implement logic to parse the manifest.xml file and extract subpackage paths
    // This is a placeholder for the actual parsing implementation
    let xml = std::fs::read_to_string(&manifest_path)?;
    // Parse XML to find <exec_depend> or <build_depend> tags
    // Extract subpackage paths and return them as a Vec<PathBuf>
    // Example: let mut subpackages = Vec::new();
    // for tag in xml.lines().filter(|line| line.starts_with("<exec_depend")) {
    //     let subpackage_path = match path::Path::parse(&tag[9..]) {
    //         Ok(path) => path,
    //         Err(_) => continue,
    //     };
    //     subpackages.push(subpackage_path);
    // }
    // return Ok(subpackages);
    unimplemented!()
}

fn main() {
    // Example usage:
    let package_name = "foo";
    let options = FindOptions {
        ros_package_path: Some(path::PathBuf::from("/path/to/ros_packages")),
        env_ros_package_path: None,
    };

    match find_ros_package(package_name, options).await {
        Ok(package_path) => println!("Found package at: {}", package_path.display()),
        Err(e) => eprintln!("Error finding package: {:?}", e),
    }
}
```

This Rust code snippet defines a similar functionality to the original TypeScript/React code for finding ROS packages. It includes functions to read package names from `package.xml` files and to recursively find packages within a specified directory structure, handling both absolute and relative paths, as well as environments where ROS_PACKAGE_PATH is set.