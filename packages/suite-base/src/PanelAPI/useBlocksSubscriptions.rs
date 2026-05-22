```rust
use std::sync::{Arc, RwLock};

use crate::{
    block_cache::{CacheKey, CacheValue},
    model::SubscribePayload,
    pipeline::MessagePipelineContext,
    state::PlayerState,
};

// Memoization probably won't speed up the filtering appreciably, but preserves return identity.
// That said, MessageBlock identity will change when the set of topics changes, so consumers should
// prefer to use the identity of topic-block message arrays where possible.
pub fn filter_block_by_topics(
    block: Option<&Arc<RwLock<CacheValue<PlayerMessageBlock>>>>,
    topics: Vec<String>,
) -> CacheValue<MessageBlock> {
    if let Some(block) = block {
        let block_data = &block.read().unwrap();
        let mut ret = HashMap::new();

        for topic in topics {
            let topic_name = match topic.as_str() {
                "" => continue,
                _ => topic,
            };

            // Don't include an empty array when the data has not been cached for this topic for this
            // block. The missing entry means "we don't know the message for this topic in this block", as
            // opposed to "we know there are no messages for this topic in this block".
            if !block_data.messages_by_topic.contains_key(topic_name) {
                continue;
            }

            let block_messages = &block_data.messages_by_topic[topic_name];
            ret.insert(topic_name.to_string(), block_messages.clone());
        }

        CacheValue::new(MessageBlock::from(ret))
    } else {
        CacheValue::empty()
    }
}

fn use_subscriptions_for_blocks(subscriptions: Vec<SubscribePayload>) -> Arc<RwLock<CacheValue<PlayerMessageBlock>>> {
    let subscriptions = Arc::new(RwLock::new(CacheValue::empty()));

    crate::pipeline::use_message_pipeline(
        Box::new(move |ctx| {
            ctx.player_state.progress.message_cache.set_blocks(subscriptions.clone());
        }),
    );

    Arc::clone(subscriptions)
}

pub fn use_blocks_subscriptions(subscriptions: Vec<SubscribePayload>) -> Vec<Option<CachedMessageBlock>> {
    let requested_topics = subscriptions.into_iter().map(|sub| sub.topic).collect();

    let subscriptions_for_blocks = use_subscriptions_for_blocks(subscriptions);

    let all_blocks = crate::pipeline::use_message_pipeline(
        Box::new(move |ctx| ctx.player_state.progress.message_cache.blocks.clone()),
    );

    if let Some(all_blocks) = &all_blocks {
        let blocks: Vec<Option<CachedMessageBlock>> = all_blocks
            .iter()
            .map(|block| {
                block.as_ref().and_then(|b| filter_block_by_topics(b, requested_topics))
                    .map(CachedMessageBlock::new)
            })
            .collect();
        blocks
    } else {
        vec![None; subscriptions.len()]
    }
}

#[derive(Clone)]
pub struct CachedMessageBlock {
    messages: Vec<MessageEvent>,
}
```