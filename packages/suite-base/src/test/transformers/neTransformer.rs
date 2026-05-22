```rust
use nearley::{Parser, Error};

/// Processes a Nearley grammar source to generate JavaScript code.
///
/// # Arguments
/// * `source_text` - The source text of the Nearley grammar.
///
/// # Returns
/// A map containing the generated JavaScript code under the key "code".
pub fn process(source_text: &str) -> HashMap<String, String> {
    let mut results = HashMap::new();
    // Parse the grammar source into an AST
    let parser = Parser::new(&nearley Grammar);
    if let Err(e) = parser.feed(source_text) {
        eprintln!("Error parsing Nearley grammar: {}", e);
        return results;
    }

    // Compile the AST into a set of rules
    let grammar_info_object = compile(parser.results[0], ());
    // Generate JavaScript code from the rules
    if let Err(e) = generate(grammar_info_object, "grammar") {
        eprintln!("Error generating JavaScript code: {}", e);
        return results;
    }

    results.insert("code".to_string(), grammar_info_object.generate().unwrap());
    results
}
```

Note:
- This Rust function uses the `nearley` crate to parse and compile Nearley grammars.
- The returned dictionary contains a single key-value pair, where the key is "code" and the value is the generated JavaScript code.
- Error handling is included to print any parsing or compilation errors.