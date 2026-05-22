```rust
use std::future::{Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

struct TimeoutError;

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Promise timed out")
    }
}

#[derive(Debug)]
struct PromiseTimeout<T> {
    timeout: std::time::Duration,
    inner: T,
}

impl<T> Future for PromiseTimeout<T>
where
    T: 'static + Send,
{
    type Output = Result<T, TimeoutError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &self.inner {
            Ok(inner) => Poll::Ready(Ok(*inner)),
            Err(err) => {
                if self.timeout <= std::time::Duration::zero() {
                    Poll::Pending
                } else {
                    let timeout = std::time::Instant::now() + self.timeout;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }
    }

    fn actix(&self, ctx: &mut Context<'_>) -> Option<Self::Output> {
        match &self.inner {
            Ok(inner) => Some(Ok(*inner)),
            Err(err) => {
                if self.timeout <= std::time::Duration::zero() {
                    None
                } else {
                    let timeout = std::time::Instant::now() + self.timeout;
                    ctx.waker().wake_by_ref();
                    None
                }
            }
        }
    }
}

fn promise_timeout<T>(contender: T, timeout: std::time::Duration) -> PromiseTimeout<T> {
    PromiseTimeout { timeout, inner: contender }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_error() {
        let contender = Promise::from(std::future::ready(42));
        assert_eq!(promise_timeout(contender, std::time::Duration::from_millis(10)), Ok(42));

        let contender = Promise::from(Panic::new("oops"));
        assert_eq!(
            promise_timeout(contender, std::time::Duration::from_millis(10)),
            Err(TimeoutError)
        );
    }
}
```