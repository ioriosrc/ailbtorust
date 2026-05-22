```rust
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

type Immutable<T> = T;
type MessageEvent = Box<dyn std::any::Any>;
type BlockItem = { input_events: VecDeque<MessageEvent>; aliased: HashMap<String, MessageEvent>; };

/**
 * BlockTopicProcessor adds alias messages to blocks.
 *
 * It tries to keep referential stability for aliased messages by tracking the input messages on the
 * original topic and storing the aliased message arrays to return them if the input messages are
 * unchanged.
 */
pub struct BlockTopicProcessor {
    original_topic: String,
    aliases: Immutable<Vec<String>>,

    blocks: Vec<Option<BlockItem>>,
}

impl BlockTopicProcessor {
    pub fn new(original_topic: String, aliases: Immutable<Vec<String>>) -> Self {
        Self {
            original_topic,
            aliases,
            blocks: vec![None],
        }
    }

    /**
     * Alias the block and return the aliased messages by topic. The aliases and the input messages
     * are stored so if the block has already been aliased and is unchanged, then the existing aliased
     * messages by topic are returned.
     */
    pub fn alias_block(block: Immutable<MessageBlock>, index: usize) -> Immutable<HashMap<String, MessageEvent>> {
        let input_events = block.messages_by_topic.get(&self.original_topic).unwrap();
        if input_events.is_empty() {
            self.blocks[index] = None;
            return Immutable::default();
        }

        let existing = &mut self.blocks[index];
        if existing.map_or(false, |item| item.input_events == *input_events) {
            return Immutable::from(item.aliased.clone());
        }

        let aliased: HashMap<String, MessageEvent> = self.aliases
            .iter()
            .map(|alias| {
                let alias_message = input_events.iter().next().unwrap();
                (
                    alias,
                    Box::new(MessageEvent {
                        as_any: alias_message.as_any(),
                    }),
                )
            })
            .collect();

        // Save the input events and the aliased data
        existing.replace(BlockItem {
            input_events: input_events.to_vecDeque(),
            aliased,
        });

        Immutable::from(aliased)
    }
}
```