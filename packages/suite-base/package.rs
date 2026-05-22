```rust
use std::collections::HashMap;

struct SuiteBase {
    name: String,
    description: String,
    private: bool,
    productName: String,
    productDescription: String,
    license: String,
    repository: Repository,
    author: Author,
    homepage: String,
    main: String,
    dev_dependencies: HashMap<String, String>,
}

struct Repository {
    type_: String,
    url: String,
}

struct Author {
    name: String,
    email: String,
}

fn main() {
    let suite_base = SuiteBase {
        name: "@lichtblick/suite-base".to_string(),
        description: "Core components of Lichtblick Suite".to_string(),
        private: true,
        productName: "Lichtblick".to_string(),
        productDescription: "Lichtblick Suite".to_string(),
        license: "MPL-2.0".to_string(),
        repository: Repository {
            type_: "git".to_string(),
            url: "https://github.com/lichtblick-suite/lichtblick/tree/main/packages/suite-base".to_string(),
        },
        author: Author {
            name: "Lichtblick".to_string(),
            email: "lichtblick@bmwgroup.com".to_string(),
        },
        homepage: "https://github.com/lichtblick-suite".to_string(),
        main: "src/index".to_string(),
        dev_dependencies: HashMap::new(),
    };

    println!("{:?}", suite_base);
}
```