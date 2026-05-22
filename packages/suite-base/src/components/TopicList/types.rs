```rust
use chrono::{DateTime, Duration};
use std::rc::Rc;

pub struct TopicBoundaries {
    first: Option<DateTime<chrono::Utc>>,
    last: Option<DateTime<chrono::Utc>>,
}

pub type UseTopicMessageNavigationReturn = Rc<dyn FnMut() -> Result<(), String>> + Send;

fn use_topic_message_navigation(
    initial_time: Option<DateTime<chrono::Utc>>,
    boundaries: &TopicBoundaries,
) -> UseTopicMessageNavigationReturn {
    let mut current_time = initial_time.clone();

    if !boundaries.first.is_none() && !boundaries.last.is_none()
        && boundaries.first.unwrap() >= boundaries.last.unwrap()
    {
        return Rc::new(move || Err("First time is greater than or equal to last time".to_string()));
    }

    let mut can navigate_next = true;
    let mut can navigate_previous = true;

    if let Some(last) = boundaries.last.as_ref() && current_time < *last {
        can navigate_next = false;
    }

    if let Some(first) = boundaries.first.as_ref() && current_time > *first {
        can navigate_previous = false;
    }

    Rc::new(move || -> Result<(), String> {
        if can Navigate_next {
            // Logic to navigate to the next message
        }
        if canNavigate_previous {
            // Logic to navigate to the previous message
        }
        Ok(())
    })
}
```