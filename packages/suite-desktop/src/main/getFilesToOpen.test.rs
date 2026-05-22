```rust
use std::path::{Path, PathBuf};

fn main() {
    // Mock all dependencies (you can replace these with actual implementations if needed)
    mock_file_utils();
    mock_parse_cliflags();
    mock_resolve_source_paths();

    describe("get_files_to_open", move || {
        before_each(|| {
            // Default mock implementations
            mock_is_file_to_open(|_| true);
        });

        describe("files from args only", move || {
            it("should return files passed as arguments", || {
                let argv = vec!["app", "file1.mcap", "file2.bag"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, vec![PathBuf::from("file1.mcap"), PathBuf::from("file2.bag")]);
            });

            it("should filter out invalid files from arguments", || {
                let argv = vec!["app", "file1.mcap", "invalid.xyz", "file2.bag"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, vec![PathBuf::from("file1.mcap"), PathBuf::from("file2.bag")]);
            });
        });

        describe("files from --source only", move || {
            it("should return files passed via --source parameter", || {
                let argv = vec!["app", "--source=source1.mcap,source2.bag"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, vec![PathBuf::from("source1.mcap"), PathBuf::from("source2.bag")]);
            });

            it("should handle empty --source parameter", || {
                let argv = vec!["app", "--source="];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, Vec::new());
            });

            it("should handle missing --source parameter", || {
                let argv = vec!["app"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, Vec::new());
            });
        });

        describe("files from both args and --source", move || {
            it("should combine files from both arguments and --source parameter", || {
                let argv = vec!["app", "arg1.mcap", "arg2.bag", "--source=source1.mcap,source2.mcap"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, vec![PathBuf::from("arg1.mcap"), PathBuf::from("arg2.bag"), PathBuf::from("source1.mcap"), PathBuf::from("source2.mcap")]);
            });

            it("should remove duplicate files when same file is provided in both args and --source", || {
                let argv = vec!["app", "unique1.mcap", "duplicated.bag", "--source=duplicated.bag,unique2.mcap"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, vec![PathBuf::from("unique1.mcap"), PathBuf::from("duplicated.bag"), PathBuf::from("unique2.mcap")]);
            });

            it("should filter out invalid files from both sources and keep valid ones", || {
                let argv = vec!["app", "valid1.mcap", "invalid1.xyz", "--source=valid2.mcap,invalid2.abc"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, vec![PathBuf::from("valid1.mcap"), PathBuf::from("valid2.mcap")]);
            });

            it("should handle complex scenario with flags, duplicates, and invalid files", || {
                let argv = vec!["app", "file1.mcap", "--debug", "duplicate.bag", "invalid.xyz", "--source=duplicate.bag,file2.mcap,another-invalid.abc", "--verbose=true"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, vec![PathBuf::from("file1.mcap"), PathBuf::from("duplicate.bag"), PathBuf::from("file2.mcap")]);
            });
        });

        describe("edge cases", move || {
            it("should handle empty argv", || {
                let argv: Vec<String> = Vec::new();
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, Vec::<PathBuf>::new());
            });

            it("should handle argv with only app name", || {
                let argv = vec!["app"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, Vec::<PathBuf>::new());
            });

            it("should handle argv with only flags", || {
                let argv = vec!["app", "--debug", "--verbose=true"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, Vec::<PathBuf>::new());
            });

            it("should handle all files being invalid", || {
                let argv = vec!["app", "invalid1.xyz", "invalid2.abc", "--source=invalid3.def"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, Vec::<PathBuf>::new());
            });

            it("should handle relative paths by converting them to absolute", || {
                let argv = vec!["app", "./relative1.mcap", "../relative2.bag"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, vec![PathBuf::from("./relative1.mcap"), PathBuf::from("../relative2.bag")]);
            });

            it("should maintain order of files (args first, then source files)", || {
                let argv = vec!["app", "arg1.mcap", "arg2.bag", "--source=source1.mcap,source2.mcap"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, vec![PathBuf::from("arg1.mcap"), PathBuf::from("arg2.bag"), PathBuf::from("source1.mcap"), PathBuf::from("source2.mcap")]);
            });
        });

        describe("integration with dependencies", move || {
            it("should call parse_cliflags with correct argv", || {
                let argv = vec!["app", "file.mcap", "--debug", "--source=test.bag"];
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, vec![PathBuf::from("file.mcap"), PathBuf::from("test.bag")]);
            });

            it("should call resolve_source_paths with source parameter from parsed flags", || {
                let argv = vec!["app", "--source=test1.mcap,test2.bag"];
                let source_param = "test1.mcap,test2.bag";
                mock_parse_cliflags(&argv).unwrap();
                mock_resolve_source_paths().unwrap();

                let result = get_files_to_open(&argv);

                assert_eq!(result, vec![PathBuf::from("test1.mcap"), PathBuf::from("test2.bag")]);
            });
        });
    });
}

fn mock_file_utils() {
    // Replace this with actual implementation
}

fn mock_parse_cliflags(argv: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    // Replace this with actual implementation
    Ok(())
}

fn mock_resolve_source_paths() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    // Replace this with actual implementation
    Ok(vec![])
}

// Define your own get_files_to_open function here
fn get_files_to_open(argv: &[String]) -> Vec<PathBuf> {
    // Implement your logic here
    vec![PathBuf::from("example.txt")]
}
```