```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReportTestResult {
    title: String,
    status: &'static str,
    duration: f64,
    retries: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    status: &'static str,
    duration: f64,
    retry: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    results: Vec<Result>,
}

#[derive(Serialize, Deserialize)]
pub struct Spec {
    title: String,
    tests: Vec<Test>,
}

#[derive(Serialize, Deserialize)]
pub struct TestSuite {
    title: String,
    file: String,
    specs: Vec<Spec>,
}

#[derive(Serialize, Deserialize)]
pub struct PlaywrightJSONReport {
    suites: Vec<TestSuite>,
}
```