```rust
use async_test::{AsyncTestRunner, TestResult, TestSetup};

mod utils;
use utils::create_event_dialog_test;

#[async_test]
async fn create_event_dialog_empty() {
  let mut runner = AsyncTestRunner::new();
  runner.add_scenario(create_event_dialog_test::empty);
}

#[async_test]
async fn create_event_dialog_normal() {
  let mut runner = AsyncTestRunner::new();
  runner.add_scenario(create_event_dialog_test::normal);
}

#[async_test]
async fn create_event_dialog_with_duplicates() {
  let mut runner = AsyncTestRunner::new();
  runner.add_scenario(create_event_dialog_test::with_duplicates);
}
```