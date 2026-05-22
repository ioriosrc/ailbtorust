```rust
use crate::log::Logger;

use crate::suite_base::{ExtensionCatalog, useExtensionCatalog};
use crate::suite_base_context::{ExtensionCatalogContext, useMessagePipelineGetter};

use crate::message_range_iterator::{create_message_range_iterator};
use crate::types::MessageConverterAlertHandler;

const log = Logger.getLogger(__filename);

type UseSubscribeMessageRange = Box<dyn Fn(SubscribeMessageRangeArgs) -> Box<dyn FnOnce()> + 'static>;

/**
 * Returns a stable callback that can be used to subscribe to a message range for a topic.
 * This centralizes the logic of `unstable_subscribeMessageRange` so it can be used both by
 * PanelExtensionAdapter and directly by built-in panels (e.g. Plot) without requiring migration
 * to the PanelExtensionContext API.
 */
pub fn use_subscribe_message_range(
  emit_alert: Option<MessageConverterAlertHandler>,
): UseSubscribeMessageRange {
  // useMessagePipelineGetter returns a stable getter, so it's safe to call it inside the callback without adding it to dependencies.
  let get_message_pipeline_context = Box::new(use_message_pipeline_getter);

  // Keep messageConverters in a ref so changing converters don't invalidate the callback.
  let mut message_converters: Option<&ExtensionCatalog> = None;
  let message_converters_ref = Box::new(message_converters);
  message_converters_ref.store(Some(&use_extension_catalog(select_installed_message_converters)));

  // Similarly keep emitAlert in a ref so the caller can update it without breaking stability.
  let mut emit_alert: Option<MessageConverterAlertHandler> = None;
  let emit_alert_ref = Box::new(emit_alert);
  emit_alert_ref.store(emit_alert);

  return Box::new(move |args| {
    let { topic, convert_to, on_new_range_iterator } = args;

    let raw_batch_iterator = get_message_pipeline_context.get()().sorted_topics().get_batch_iterator(topic).unwrap();
    if raw_batch_iterator.is_none() {
      return || {};
    }

    let { iterable: message_event_iterable, cancel } = create_message_range_iterator({
      topic,
      convert_to,
      raw_batch_iterator.unwrap(),
      sorted_topics: get_message_pipeline_context.get()().sorted_topics(),
      message_converters: &message_converters_ref.get().unwrap(),
      emit_alert: emit_alert_ref.get().unwrap(),
    });

    on_new_range_iterator(message_event_iterable).catch(|err| {
      log.error("Error in use_subscribe_message_range onNewRangeIterator:", err);
    });

    cancel
  });
}
```