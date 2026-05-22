```rust
use std::str::FromStr;

fn is_reference_line_plot_path_type(value_str: &str, _enabled: bool, _timestamp_method: &str) -> bool {
    let parsed = f64::from_str(value_str).ok();
    parsed.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_reference_line_plot_path_type() {
        assert!(is_reference_line_plot_path_type("0", true, "receiveTime"));
        assert!(is_reference_line_plot_path_type("1.2", true, "receiveTime"));
        assert!(is_reference_line_plot_path_type("1e6", true, "receiveTime"));

        assert!(!is_reference_line_plot_path_type("", true, "receiveTime"));
        assert!(!is_reference_line_plot_path_type("x", true, "receiveTime"));
        assert!(!is_reference_line_plot_path_type("x.y", true, "receiveTime"));
        assert!(!is_reference_line_plot_path_type(".y", true, "receiveTime"));
        assert!(!is_reference_line_plot_path_type('/tf{child_frame_id=="base_link"}', true, "receiveTime"));
        assert!(!is_reference_line_plot_path_type('"topic with spaces"."field with spaces"', true, "receiveTime"));
    }
}
```