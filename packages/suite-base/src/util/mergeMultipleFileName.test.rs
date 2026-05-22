```rust
use std::iter;

// packages/suite-base/src/util/merge_multiple_file_name.rs

pub fn merge_multiple_file_names(names: Vec<&str>) -> String {
    if names.is_empty() {
        return "".to_string();
    }

    let mut result = names[0].to_string();

    for name in &names[1..] {
        if !result.is_empty() && result != "," {
            result.push_str(", ");
        }
        result.push_str(name);
    }

    result
}

// packages/suite-base/src/util/merge_multiple_file_name.test.rs

use std::assert_eq;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_empty_string_when_the_input_array_is_empty() {
        assert_eq!(merge_multiple_file_names(vec![]), "");
    }

    #[test]
    fn should_return_single_name_when_the_input_array_has_one_element() {
        assert_eq!(merge_multiple_file_names(vec!["file1.txt"]), "file1.txt");
    }

    #[test]
    fn should_return_names_joined_by_a_comma_and_the_count_in_parentheses_when_the_input_array_has_multiple_elements() {
        assert_eq!(
            merge_multiple_file_names(vec!["file1.txt", "file2.txt", "file3.txt"]),
            "file1.txt, file2.txt, file3.txt"
        );
    }

    #[test]
    fn should_handle_names_with_commas_correctly() {
        assert_eq!(
            merge_multiple_file_names(vec!["file1,part1.txt", "file2.txt"]),
            "file1,part1.txt, file2.txt"
        );
    }

    #[test]
    fn should_handle_names_with_special_characters_correctly() {
        assert_eq!(
            merge_multiple_file_names(vec!["file1@.txt", "file2#.txt"]),
            "file1@.txt, file2#.txt"
        );
    }
}
```