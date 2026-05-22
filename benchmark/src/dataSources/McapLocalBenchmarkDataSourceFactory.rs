```rust
use lichtblick::context::player_selection_context::{IDataSourceFactory, DataSourceFactoryInitializeArgs};
use lichtblick::players::iterable_player::{DeserializingIterableSource, McapIterableSource};
use lichtblick::players::Player;
use std::fs;

pub struct McapLocalBenchmarkDataSourceFactory {
    id: String,
    type_: &'static str,
    display_name: &'static str,
    icon_name: &'static str,
    supported_file_types: Vec<&'static str>,
}

impl IDataSourceFactory for McapLocalBenchmarkDataSourceFactory {
    fn id(&self) -> &str {
        &self.id
    }

    fn type_(&self) -> &'static str {
        self.type_
    }

    fn display_name(&self) -> &'static str {
        self.display_name
    }

    fn icon_name(&self) -> &'static str {
        self.icon_name
    }

    fn supported_file_types(&self) -> &[&'static str] {
        &self.supported_file_types
    }

    fn initialize(&self, args: DataSourceFactoryInitializeArgs) -> Option<Player> {
        if let Some(file) = args.file {
            let mcap_provider = McapIterableSource { type_: "file", file };
            let source = DeserializingIterableSource(mcap_provider);
            Some(BenchmarkPlayer(file.name(), source))
        } else {
            None
        }
    }
}

fn main() {
    // Example usage:
    // let factory = McapLocalBenchmarkDataSourceFactory {
    //     id: "mcap-local-file".to_string(),
    //     type_: "file",
    //     display_name: "MCAP",
    //     icon_name: "OpenFile",
    //     supported_file_types: vec![".mcap"],
    // };
    // let args = DataSourceFactoryInitializeArgs { file: Some(fs::File::open("path_to_your_mcap_file.mcap").unwrap()) };
    // let player = factory.initialize(args).unwrap();
}
```

Note that this Rust code is a simplified version of the original TypeScript/React code. The main differences include:

1. The use of Rust's standard library for file handling instead of Node.js's `fs`.
2. The use of trait objects (`IDataSourceFactory` and `Player`) instead of class inheritance.
3. The use of the `Option` type to handle potential errors more gracefully.
4. The lack of type annotations, as Rust has strong typing.

The actual implementation might require additional error handling and context management depending on the specific requirements of the application.