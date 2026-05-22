```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub fn contains_func_declaration(args: Vec<&dyn std::any::Any>) -> bool {
    for arg in args {
        if arg.is::<fn(&dyn std::any::Any, &GlobalVariables) -> Option<HashMap<String, serde_json::Value>>>() {
            return true;
        } else if let Some(map) = arg.downcast_ref::<HashMap<String, serde_json::Value>>() {
            for value in map.values() {
                if contains_func_declaration(vec![value]) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn stringify_funcs_in_object(arg: &dyn std::any::Any) -> Box<dyn std::any::Any> {
    if let Some(func) = arg.downcast_ref::<fn(&dyn std::any::Any, &GlobalVariables) -> Option<HashMap<String, serde_json::Value>>>() {
        return Box::new(func.to_string());
    } else if let Some(map) = arg.downcast_ref::<HashMap<String, serde_json::Value>>() {
        let mut new_map: HashMap<String, Box<dyn std::any::Any>> = HashMap::new();
        for (key, value) in map {
            new_map.insert(key.clone(), stringify_funcs_in_object(value));
        }
        return Box::new(new_map);
    }
    Box::new(arg)
}

pub fn get_args_to_print(args: Vec<&dyn std::any::Any>) -> Vec<String> {
    args.into_iter()
        .map(|arg| arg.to_string())
        .collect()
}

// Exported for tests.
#[cfg(test)]
mod test_contains_func_declaration {
    use super::*;

    #[test]
    fn contains_func_declaration_with_function() {
        assert!(contains_func_declaration(vec![Box::new(|| Some(hashmap!["key" => "value"]))]));
    }

    #[test]
    fn does_not_contain_func_declaration_with_object() {
        assert!(!contains_func_declaration(vec![Box::new(HashMap::from([
            ("key".to_string(), 42),
        ]))]);
    }
}

#[cfg(test)]
mod test_stringify_funcs_in_object {
    use super::*;

    #[test]
    fn stringify_func_to_string() {
        let result = stringify_funcs_in_object(Box::new(|| Some(hashmap!["key" => "value"]))).to_string();
        assert_eq!(result, "fn() -> Option<HashMap<String, serde_json::Value>>");
    }

    #[test]
    fn stringify_map_to_hashmap() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("key".to_string(), 42);
        let result = stringify_funcs_in_object(Box::new(map)).to_string();
        assert_eq!(result, "HashMap { key: 42 }");
    }
}

#[cfg(test)]
mod test_get_args_to_print {
    use super::*;

    #[test]
    fn get_args_to_print_with_function() {
        let result = get_args_to_print(vec![
            Box::new(|| Some(hashmap!["key" => "value"]))),
        ]);
        assert_eq!(result, vec!["fn() -> Option<HashMap<String, serde_json::Value>>"]);
    }

    #[test]
    fn get_args_to_print_with_object() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("key".to_string(), 42);
        let result = get_args_to_print(vec![
            Box::new(map),
        ]);
        assert_eq!(result, vec!["HashMap { key: 42 }"]);
    }
}

pub fn require_implementation(id: &str, project_code: HashMap<&str, String>) -> serde_json::Value {
    if let Some((file, source)) = project_code.get(&DEFAULT_STUDIO_SCRIPT_PREFIX.to_string() + id) {
        // Using new Function in order to execute user-input text in User Scripts as code
        let script_eval = format!(
            "({})({}({}))",
            file,
            DEFAULT_STUDIO_SCRIPT_PREFIX.to_string(),
            id
        );
        serde_json::from_str(&script_eval).unwrap()
    } else {
        panic!("User script required unknown module: '{}'!", id);
    }
}

pub fn register_script({
    script_code: String,
    project_code,
}: &RegisterScriptArgs) -> RegistrationOutput {
    let mut user_script_logs = vec![];
    let mut user_script_diagnostics = vec![];

    // Using new Function in order to execute user-input text in User Scripts as code
    let script_eval = format!(
        "({})({}({}))",
        script_code,
        DEFAULT_STUDIO_SCRIPT_PREFIX.to_string(),
        script_code
    );

    let eval_result: serde_json::Value = serde_json::from_str(&script_eval).unwrap();

    if let Some(node_callback) = eval_result.get("default") {
        node_callback = node_callback.as_ref().downcast::<Box<dyn std::any::Any>>()?;
    } else {
        panic!("Failed to find 'default' function in the script.");
    }

    Ok(RegistrationOutput {
        error: None,
        user_script_logs,
        user_script_diagnostics,
    })
}

pub fn process_message({
    message,
    global_variables,
}: &ProcessMessageArgs) -> ProcessMessageOutput {
    let mut user_script_logs = vec![];
    let mut user_script_diagnostics = vec![];

    // Using new Function in order to execute user-input text in User Scripts as code
    let script_eval = format!(
        "({})({}({}))",
        message,
        DEFAULT_STUDIO_SCRIPT_PREFIX.to_string(),
        script_code
    );

    let eval_result: serde_json::Value = serde_json::from_str(&script_eval).unwrap();

    if let Some(node_callback) = eval_result.get("default") {
        let new_message = node_callback.as_ref().downcast::<Box<dyn std::any::Any>>()?;
        Ok(ProcessMessageOutput {
            message: new_message.clone() as serde_json::Value,
            error: None,
            user_script_logs,
            user_script_diagnostics,
        })
    } else {
        let diagnostic = Diagnostic {
            source: DEFAULT_STUDIO_SCRIPT_PREFIX.to_string(),
            severity: DIAGNOSTIC_SEVERITY.Error,
            message: "Unknown error encountered running this node.",
            code: ERROR_CODES.RUNTIME,
        };
        Ok(ProcessMessageOutput {
            message: None as serde_json::Value,
            error: Some(diagnostic),
            user_script_logs,
            user_script_diagnostics,
        })
    }
}

struct RegisterScriptArgs {
    script_code: String;
    project_code: HashMap<&str, String>;
}

struct ProcessMessageArgs {
    message: String;
    global_variables: GlobalVariables;
}

struct RegistrationOutput {
    error: Option<String>;
    user_script_logs: Vec<UserScriptLog>;
    user_script_diagnostics: Vec<Diagnostic>;
}

struct UserScriptLog {
    source: &str;
    value: serde_json::Value;
}

enum Diagnostic {
    Source(&'static str),
    Severity(DiagnosticSeverity),
    Message(String),
    Code(u32),
}
```

Note that this Rust code assumes the existence of `GlobalVariables`, `DiagnosticSeverity`, and `ERROR_CODES` from a hypothetical module `@lichtblick/suite-base/hooks/useGlobalVariables`, `@lichtblick/suite-base/players/UserScriptPlayer/constants`, and `@lichtblick/suite-base/util/constants`. The `serde_json::Value` type is assumed to be imported from the `serde_json` crate. Additionally, the Rust code uses the `new_function!` macro for executing user-input text in User Scripts as code, which is not available in standard Rust due to security reasons.