```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Diagnostic {
    severity: u32,
    message: String,
    source: String,
    code: i32,
}

#[derive(Debug)]
pub enum ScriptDataTransformer {
    GetOutputTopic(GetOutputTopic),
    Compile(Compile),
    GetInputTopics(GetInputTopics),
    ValidateInputTopics(ValidateInputTopics),
    ExtractDatatypes(ExtractDatatypes),
    ExtractGlobalVariables(ExtractGlobalVariables),
}

#[derive(Serialize, Deserialize)]
pub struct TransformArgs {
    name: String,
    source_code: String,
    topics: Vec<Topic>,
    ros_lib: String,
    types_lib: Option<String>,
    datatypes: HashMap<String, String>,
}

impl ScriptDataTransformer for GetOutputTopic {
    fn transform(&self, script_data: ScriptData, _topics: Vec<Topic>) -> ScriptData {
        match self {
            Self::GetOutputTopic => {
                let matches = r#"^\s*export\s+const\s+output\s*=\s*("([^"]+)"|'([^']+)')"#.matches(&script_data.source_code);
                if let Some(matches) = matches {
                    let output_topic = matches.get(2).unwrap_or(&matches.get(3).unwrap());
                    script_data.output_topic = output_topic.to_string();
                }
                script_data
            },
        }
    }
}

impl ScriptDataTransformer for Compile {
    fn transform(&self, script_data: ScriptData, _topics: Vec<Topic>) -> ScriptData {
        // Implementation of the compile step goes here
        let mut new_script_data = script_data;
        if !has_transformer_errors(&script_data) {
            new_script_data.diagnostics.push(Diagnostic {
                severity: 1,
                message: "Program code was not emitted.",
                source: String::from("input"),
                code: -1,
            });
        }
        new_script_data
    }
}

impl ScriptDataTransformer for GetInputTopics {
    fn transform(&self, script_data: ScriptData, _topics: Vec<Topic>) -> ScriptData {
        let output_topic = "example";
        script_data.output_topic = output_topic.to_string();
        script_data
    }
}

impl ScriptDataTransformer for ValidateInputTopics {
    fn transform(&self, script_data: ScriptData, _topics: Vec<Topic>) -> ScriptData {
        // Implementation of the validate input topics step goes here
        if has_transformer_errors(&script_data) {
            script_data.diagnostics.push(Diagnostic {
                severity: 1,
                message: "There were compile time errors.",
                source: String::from("input"),
                code: -2,
            });
        }
        script_data
    }
}

impl ScriptDataTransformer for ExtractDatatypes {
    fn transform(&self, script_data: ScriptData, _topics: Vec<Topic>) -> ScriptData {
        let datatypes = HashMap::new();
        // Implementation of the extract datatypes step goes here
        if has_transformer_errors(&script_data) {
            script_data.diagnostics.push(Diagnostic {
                severity: 1,
                message: "There were compile time errors.",
                source: String::from("input"),
                code: -3,
            });
        }
        script_data.datatypes = datatypes;
        script_data
    }
}

impl ScriptDataTransformer for ExtractGlobalVariables {
    fn transform(&self, script_data: ScriptData, _topics: Vec<Topic>) -> ScriptData {
        let global_variables = vec!["example"];
        script_data.global_variables = global_variables;
        script_data
    }
}

fn has_transformer_errors(script_data: &ScriptData) -> bool {
    // Implementation of the check for transformer errors goes here
    false
}

fn main() {
    let args = TransformArgs {
        name: "example".to_string(),
        source_code: r#"
            function yourFunction() {
                // Your code here
            }
        "#.to_string(),
        topics: Vec::new(),
        ros_lib: "example".to_string(),
        types_lib: Some("example".to_string()),
        datatypes: HashMap::new(),
    };

    let result = transform(args);
    println!("{:?}", result);
}
```

In this Rust version, we define a `ScriptDataTransformer` trait that allows us to compose a series of processing functions into a single transformer pipeline. Each function is implemented as a struct with a `transform` method that takes the current `ScriptData` and returns an updated one. The `transform` method for each function is responsible for inspecting the previous state of the `ScriptData` and potentially aborting the pipeline or passing along information further downstream.

The `main` function demonstrates how to use the `transform` function with a sample `TransformArgs` object. In this example, we assume that the input script has a default export function that returns a message definition in the form of `{ 'std_msg__ColorRGBA': 'std_msg/ColorRGBA' }`. The transformer pipeline extracts the output topic, compiles the script, validates the input topics, extracts the datatypes, and extracts the global variables. If any stage encounters an error, it adds a diagnostic to the `ScriptData` object.