```rust
// Define the struct for the package metadata
struct PackageMetadata {
    name: String,
    version: String,
    license: String,
    private: bool,
    product_name: String,
    description: String,
    product_description: String,
    package_manager: String,
    repository: Repository,
    author: Author,
    homepage: String,
    engines: Engines,
    scripts: Scripts,
    workspaces: Workspaces,
    resolutions: Resolutions,
    dev_dependencies: DevDependencies,
}

// Define the struct for the repository metadata
struct Repository {
    type_: String,
    url: String,
}

// Define the struct for the author metadata
struct Author {
    name: String,
    email: String,
}

// Define the struct for the engines metadata
struct Engines {
    node: String,
}

// Define the struct for the scripts metadata
struct Scripts {
    // Define the script properties here
}

// Define the struct for the workspaces metadata
struct Workspaces {
    packages: Vec<String>,
}

// Define the struct for the resolutions metadata
struct Resolutions {
    typescript: String,
    dompurify: String,
    qs: String,
    tar: String,
    undici: String,
    flattened: String,
}

// Define the struct for the dev_dependencies metadata
struct DevDependencies {
    // Define the dependency properties here
}

fn main() {
    let package_metadata = PackageMetadata {
        name: "lichtblick".to_string(),
        version: "1.24.3".to_string(),
        license: "MPL-2.0".to_string(),
        private: true,
        product_name: "Lichtblick".to_string(),
        description: "Core components of Lichtblick".to_string(),
        product_description: "Lichtblick Suite".to_string(),
        package_manager: "yarn@3.6.3".to_string(),
        repository: Repository {
            type_: "git".to_string(),
            url: "https://github.com/lichtblick-suite/lichtblick.git".to_string(),
        },
        author: Author {
            name: "Lichtblick".to_string(),
            email: "lichtblick@bmwgroup.com".to_string(),
        },
        homepage: "https://github.com/lichtblick-suite".to_string(),
        engines: Engines {
            node: ">=20".to_string(),
        },
        scripts: Scripts {
            // Initialize the script properties here
        },
        workspaces: Workspaces {
            packages: vec![
                "desktop".to_string(),
                "packages/*",
                "packages/@types/*",
                "web",
                "benchmark",
            ],
        },
        resolutions: Resolutions {
            typescript: "20.10.0".to_string(),
            dompurify: "3.4.0",
            qs: "6.14.1",
            tar: "7.5.11",
            undici: "6.24.1",
            flattened: "3.4.2",
        },
        dev_dependencies: DevDependencies {
            // Initialize the dependency properties here
        },
    };

    println!("{:?}", package_metadata);
}
```