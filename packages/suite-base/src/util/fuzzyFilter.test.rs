```rust
use std::cmp::Ordering;

struct FuzzyFilterOptions {
    options: Vec<String>,
    filter: String,
    get_text: fn(&str) -> String,
    sort: bool,
}

fn fuzzy_filter(options: FuzzyFilterOptions) -> Vec<&str> {
    let mut filtered_options = options.options
        .iter()
        .filter(|&option| {
            let option_text = options.get_text(option);
            let filter_text = options.filter;
            let normalized_option = option_text.to_lowercase().replace(|c| !c.is_alphanumeric(), "");
            let normalized_filter = filter_text.to_lowercase().replace(|c| !c.is_alphanumeric(), "");

            if normalized_option.contains(&normalized_filter) {
                match options.sort {
                    true => normalized_option.cmp(&normalized_filter),
                    false => Ordering::Equal,
                }
            } else {
                ordering::Less
            }
        })
        .collect();

    if options.sort {
        filtered_options.sort_unstable();
    }

    filtered_options.into_iter().map(|option| &options.options[filtered_options.iter().position(|&o| o == option).unwrap()]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_correctly() {
        let options = FuzzyFilterOptions {
            options: vec!["abc", "def"],
            filter: "a",
            get_text: |x| x,
            sort: true,
        };
        assert_eq!(fuzzy_filter(options), vec!["abc"]);

        let options = FuzzyFilterOptions {
            options: vec!["abc", "def"],
            filter: "e",
            get_text: |x| x,
            sort: true,
        };
        assert_eq!(fuzzy_filter(options), vec!["def"]);

        let options = FuzzyFilterOptions {
            options: vec!["abc", "def"],
            filter: "aa",
            get_text: |x| x,
            sort: true,
        };
        assert_eq!(fuzzy_filter(options), Vec::<&str>::new());

        let options = FuzzyFilterOptions {
            options: vec!["abc", "def"],
            filter: "z",
            get_text: |x| x,
            sort: true,
        };
        assert_eq!(fuzzy_filter(options), Vec::<&str>::new());
    }

    #[test]
    fn sorts_better_matches_first() {
        let options = FuzzyFilterOptions {
            options: vec!["abbc", "abc"],
            filter: "abc",
            get_text: |x| x,
            sort: true,
        };
        assert_eq!(fuzzy_filter(options), vec!["abc", "abbc"]);

        let options = FuzzyFilterOptions {
            options: vec!["abb", "ab"],
            filter: "ab",
            get_text: |x| x,
            sort: true,
        };
        assert_eq!(fuzzy_filter(options), vec!["ab", "abb"]);
    }

    #[test]
    fn allows_disabling_sorting() {
        let options = FuzzyFilterOptions {
            options: vec!["abbc", "abc"],
            filter: "abc",
            get_text: |x| x,
            sort: false,
        };
        assert_eq!(fuzzy_filter(options), vec!["abbc", "abc"]);

        let options = FuzzyFilterOptions {
            options: vec!["abb", "ab"],
            filter: "ab",
            get_text: |x| x,
            sort: false,
        };
        assert_eq!(fuzzy_filter(options), vec!["abb", "ab"]);
    }

    #[test]
    fn ignores_punctuation_and_capitalization() {
        let options = FuzzyFilterOptions {
            options: vec!["ab/cDE"],
            filter: "a-b_Cde",
            get_text: |x| x,
            sort: true,
        };
        assert_eq!(fuzzy_filter(options), vec!["ab/cDE"]);
    }

    #[test]
    fn supports_custom_objects() {
        let options = FuzzyFilterOptions {
            options: vec![{"x" => "abc"}, {"x" => "def"}],
            filter: "a",
            get_text: |x| x.get("x"),
            sort: true,
        };
        assert_eq!(fuzzy_filter(options), vec![{"x" => "abc"}]);
    }
}
```