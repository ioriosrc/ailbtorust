```rust
use std::collections::HashMap;

// Define the filter structure
pub struct Filter {
    pub min_log_level: u32,
    pub search_terms: Vec<String>,
    pub name_filter: HashMap<String, Option<bool>>,
}

// Convert the TypeScript/React code to Rust functional code
fn filter_messages(
    events: &[LogMessageEvent],
    filter: &Filter,
) -> impl Iterator<Item = LogMessageEvent> {
    let has_active_filters = filter.min_log_level > 1
        || !filter.search_terms.is_empty()
        || filter.name_filter.values().any(|v| v.is_none() || v.unwrap());

    if !has_active_filters {
        return events.iter();
    }

    let search_terms_in_lowercase: Vec<String> = filter.search_terms.clone().into_iter().map(|term| term.to_lowercase()).collect();

    events.into_iter().filter(move |event| {
        let log_message = event.message;
        let effective_log_level = get_normalized_level(event.schema_name, log_message);
        if effective_log_level < filter.min_log_level {
            return false;
        }

        let maybe_name = might ActuallyBePartial(log_message).name();
        if maybe_name.is_none() || !filter.name_filter.get(maybe_name).unwrap_or(false) {
            return false;
        }

        if search_terms_in_lowercase.is_empty() {
            return true;
        }

        let lower_case_name = maybe_name.as_deref().map(|n| n.to_lowercase());
        let lower_case_msg = get_normalized_message(log_message).to_lowercase();
        search_terms_in_lowercase
            .any(|term| lower_case_name.map_or(false, |name| name.contains(term))
                || lower_case_msg.contains(term))
    })
}
```