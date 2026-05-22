```rust
use std::future::Future;
use futures::stream::{AsyncStream, AsyncTryStreamExt};
use futures::task::{Context, Poll};

use crate::{
    message_range_iterator::create_message_range_iterator,
    use_extension_catalog::use_extension_catalog,
    use_message_pipeline_getter::use_message_pipeline_getter,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn does_not_call_on_new_range_iterator_when_batch_iterator_is_unavailable() {
        // Given
        let mut mock_use_extension_catalog = MockExtensionCatalog::new();
        mock_use_extension_catalog.expect_with(|| {
            Ok(vec![])
        });
        let on_new_range_iterator = Arc::new(MockFutureResponse {});
        let (cancel, _) = Arc::try_unwrap(on_new_range_iterator).unwrap();

        let use_message_pipeline_getter = MockMessagePipelineGetter::new();
        let mut result = use_subscribe_message_range(use_message_pipeline_getter);

        // When
        let _ = result.await;

        // Then
        assert_eq!(cancel.is_some(), false);
    }

    #[tokio::test]
    async fn returns_a_callable_cancel_function_when_batch_iterator_is_unavailable() {
        // Given
        let mut mock_use_extension_catalog = MockExtensionCatalog::new();
        mock_use_extension_catalog.expect_with(|| {
            Ok(vec![])
        });
        let on_new_range_iterator = Arc::new(MockFutureResponse {});
        let (cancel, _) = Arc::try_unwrap(on_new_range_iterator).unwrap();

        let use_message_pipeline_getter = MockMessagePipelineGetter::new();
        let mut result = use_subscribe_message_range(use_message_pipeline_getter);

        // When
        let cancel = result.await;

        // Then
        assert!(cancel.is_some());
    }

    #[tokio::test]
    async fn calls_on_new_range_iterator_with_the_iterable_when_batch_iterator_is_available() {
        // Given
        let topic = BasicBuilder::string();
        let mock_iterable: AsyncStream<MessageEvent> = futures::stream::iter(vec![]);
        let mock_cancel = Arc::new(MockFutureResponse {});
        let (cancel, _) = Arc::try_unwrap(mock_cancel).unwrap();

        let mut mock_batch_iterator = MockBatchIterator::new();
        let mock_get_batch_iterator = Arc::new(MockFutureResponse { value: Some(mock_batch_iterator) });
        let mut use_message_pipeline_getter = MockMessagePipelineGetter::new();
        use_message_pipeline_getter.expect_with(|| {
            Ok(vec![])
        });

        let result = use_subscribe_message_range(use_message_pipeline_getter);

        // When
        let cancel = result.await;

        // Then
        assert!(cancel.is_some());
        assert_eq!(
            mock_get_batch_iterator.value(),
            Some(&mock_batch_iterator),
            "batch iterator should be the same object"
        );
        assert_eq!(
            *mock_batch_iterator,
            MockBatchIterator::new(vec![MessageEvent {
                topic: topic.to_string(),
                data: vec![],
            }]),
            "batch iterator should have the correct data"
        );
    }
}
```