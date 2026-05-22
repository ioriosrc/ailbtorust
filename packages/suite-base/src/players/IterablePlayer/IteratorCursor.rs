```rust
use std::future::{Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct IteratorCursor<MessageType = ()> {
    iter: Pin<Box<dyn Iterator<Item = Result<IteratorResult<MessageType>, Error>> + Send>>,
    abort: Option<AbortSignal>,
    last_result: Option<Result<IteratorResult<MessageType>, Error>>,
}

impl<T> IteratorCursor<T> {
    pub fn new(iter: impl Iterator<Item = Result<IteratorResult<T>, Error>> + Send) -> Self {
        IteratorCursor {
            iter: Pin::new(Box::from(iter)),
            abort: None,
            last_result: None,
        }
    }

    pub async fn next(&mut self) -> Option<Result<IteratorResult<T>, Error>> {
        if let Some(abort) = &self.abort {
            if abort.is_aborted() {
                return None;
            }
        }

        match Pin::poll_unchecked_mut(&mut self.iter).await? {
            Ok(result) => Some(result),
            Err(e) => None,
        }
    }

    pub async fn next_batch(&mut self, duration_ms: u32) -> Option<Vec<IteratorResult<T>>> {
        let first_result = self.next().await?;

        if matches!(first_result.type_, "alert") {
            return vec![first_result];
        }

        let results = Vec::new();
        let cutoff_time = add_time(first_result.stamp, duration_ms * 1000);

        for result in Pin::iter_unchecked_mut(&mut self.iter).take_while(|result| !matches!(result?, "alert")) {
            if let Ok(value) = result.await? {
                results.push(value);
            }

            if value.type_ == "stamp" && compare(value.stamp, cutoff_time) > 0 {
                break;
            }
        }

        Some(results)
    }

    pub async fn read_until(&mut self, end: Time) -> Option<Vec<IteratorResult<T>>> {
        let is_aborted = match &self.abort {
            Some(abort) => abort.is_aborted(),
            None => false,
        };

        if is_aborted {
            return None;
        }

        let mut results = Vec::new();

        if let Some(last_result) = self.last_result.take() {
            if matches!(last_result.type_, "stamp") && compare(last_result.stamp, end) >= 0 {
                return results;
            }
        }

        if let Some(last_result) = last_result.take() {
            results.push(last_result);
        }

        for result in Pin::iter_unchecked_mut(&mut self.iter).take_while(|result| !matches!(result?, "alert")) {
            if let Ok(value) = result.await? {
                results.push(value);
            }

            if value.type_ == "stamp" && compare(value.stamp, end) >= 0 {
                self.last_result = Some(result?.clone());
                break;
            }
        }

        Some(results)
    }

    pub async fn end(&mut self) -> Result<(), Error> {
        self.iter.as_mut().return()?.await
    }
}

pub enum IteratorResult<T> {
    Alert,
    Stamp(Time),
    MessageEvent(MessageEvent),
}

#[derive(Clone)]
pub struct Time {
    // Define the fields of a Time structure here
}

pub struct MessageEvent {
    receive_time: Time,
    // Define the fields of a MessageEvent structure here
}
```