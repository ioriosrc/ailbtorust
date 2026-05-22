```rust
use std::path;

// Mocking dependencies before any imports
#[allow(unused)]
fn mock_dependencies() {
    use log::{debug};
    let logger = log::Logger::new(debug);
}

mod file_utils;
mod inject_files_to_open;
mod studio_window;

mod create_new_window;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_create_new_window_with_deep_links() {
        mock_dependencies();
        let argv = vec!["app", "lichtblick://test-link"];
        let result = create_new_window(argv);

        assert_eq!(result, Ok(()));
        assert_eq!(studio_window::mock_load.call_count(), 1);
    }

    #[test]
    fn test_create_new_window_filter_flags_and_process_files() {
        mock_dependencies();
        let argv = vec!["app", "--flag=value", "test.bag"];
        studio_window::mock_is_file_to_open.expect_with(|path| path == PathBuf::from("test.bag")).returning(|_| true);
        studio_window::mock_is_file_to_open.expect_with(|path| path != PathBuf::from("--flag=value")).returning(|_| false);

        let result = create_new_window(argv);

        assert_eq!(result, Ok(()));
        assert_eq!(studio_window::mock_load.call_count(), 1);
    }

    #[test]
    async fn test_create_new_window_setup_file_injection_callback() {
        mock_dependencies();
        let argv = vec!["app", "test.bag"];
        studio_window::mock_is_file_to_open.expect_with(|path| path == PathBuf::from("test.bag")).returning(|_| true);

        let callback: Box<dyn FnOnce()> = Box::new(async move {
            assert_eq!(studio_window::mock_web_contents.call_count(), 1);
            let web_contents = studio_window::mock_web_contents();
            assert_eq!(web_contents.debugger.mock, "debugger");
            assert_eq!(studio_window::mock_inject_files_to_open.call_count(), 1);
        });

        create_new_window(argv);

        // Simulate the callback execution
        let _ = callback.await;
    }

    #[test]
    fn test_create_new_window_not_inject_files_when_no_files_are_present() {
        mock_dependencies();
        let argv = vec!["app", "lichtblick://link"];
        studio_window::mock_is_file_to_open.expect_with(|path| path == PathBuf::from("test.bag")).returning(|_| false);

        let result = create_new_window(argv);

        assert_eq!(result, Ok(()));
        assert_eq!(studio_window::mock_inject_files_to_open.call_count(), 0);
    }
}
```