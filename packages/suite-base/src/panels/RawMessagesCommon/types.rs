```rust
// Rust code for converting the given TypeScript/React code to Rust

use @mui/material as material;
use css;
use react::prelude::*;
use std::collections::{HashSet, HashMap};
use std::fmt;

// Define the raw message panel config struct
#[derive(Debug, Clone)]
pub struct RawMessagesPanelConfig {
    pub diff_enabled: bool,
    pub diff_method: String,
    pub diff_topic_path: String,
    pub expansion: Option<NodeExpansion>,
    pub show_full_message_for_diff: bool,
    pub topic_path: String,
    pub font_size: Option<f64>,
}

// Define the raw messages virtual panel config struct
#[derive(Debug, Clone)]
pub struct RawMessagesVirtualPanelConfig {
    pub diff_enabled: bool,
    pub diff_method: String,
    pub diff_topic_path: String,
    pub expansion: Option<NodeExpansion>,
    pub show_full_message_for_diff: bool,
    pub topic_path: String,
    pub font_size: Option<f64>,
}

// Define the node state enum
#[derive(Debug, Clone)]
pub enum NodeState {
    Collapsed,
    Expanded,
}

// Define the node expansion enum
pub enum NodeExpansion {
    All,
    None,
    HashMap<String, NodeState>,
}

// Define the diff object type
type DiffObject = HashMap<String, serde_json::Value>;

// Define the tree node struct
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub key: String;
    pub label: String;
    pub value: serde_json::Value;
    pub depth: usize;
    pub is_expandable: bool;
    pub key_path: Vec<&str>;
    pub parent_path: String,
}

// Define the value action type
#[derive(Debug, Clone)]
pub struct ValueAction {
    pub single_slice_path: String;
    pub multi_slice_path: String;
    pub primitive_type: String;
    pub filter_path: String;
}

// Define props for diff span
#[derive(Debug, Clone)]
pub struct PropsDiffSpan {
    children: Option<ReactNode>,
    style: Option<style::StyleProps>,
}

// Define props for diff stats
#[derive(Debug, Clone)]
pub struct PropsDiffStats {
    data: DiffObject,
    item_type: ReactNode,
}

// Define props for highlighted value
#[derive(Debug, Clone)]
pub struct PropsHighlightedValue {
    item_label: String,
}

// Define props for raw messages
#[derive(Debug, Clone)]
pub struct PropsRawMessages {
    config: Immutable<RawMessagesPanelConfig>,
    save_config: SaveConfig<RawMessagesPanelConfig>,
}

// Define props for raw messages virtual
#[derive(Debug, Clone)]
pub struct PropsRawMessagesVirtual {
    config: Immutable<RawMessagesVirtualPanelConfig>,
    save_config: SaveConfig<RawMessagesVirtualPanelConfig>,
}

// Define props maybe collapsed value
#[derive(Debug, Clone)]
pub struct PropsMaybeCollapsedValue {
    item_label: String,
}

// Define props metadata
#[derive(Debug, Clone)]
pub struct PropsMetadata {
    data: serde_json::Value,
    diff_data: serde_json::Value,
    diff: serde_json::Value,
    datatype: Option<&str>,
    message: MessageEvent,
    diff_message: Option<MessageEvent>,
}

// Define props toolbar
#[derive(Debug, Clone)]
pub struct PropsToolbar {
    can_expand_all: bool,
    diff_enabled: bool,
    diff_method: RawMessagesVirtualPanelConfig::diff_method,
    diff_topic_path: String,
    on_diff_topic_path_change: fn(new_topic_path: String),
    on_toggle_diff: fn(),
    on_toggle_expand_all: fn(),
    on_topic_path_change: fn(new_topic_path: String),
    save_config: SaveConfig<RawMessagesVirtualPanelConfig>,
    topic: Option<PlayerTopic>,
    topic_path: String,
}

// Define props value
#[derive(Debug, Clone)]
pub struct PropsValue {
    arr_label: String;
    basePath: String;
    item_label: String;
    item_value: serde_json::Value;
    value_action: Option<ValueAction>,
    on_topic_path_change: fn(new_topic_path: String),
    open_sibling_panel: OpenSiblingPanel,
}

// Define value action item
#[derive(Debug, Clone)]
pub struct ValueActionItem {
    key: String;
    tooltip: String,
    icon: ReactNode,
    onClick: Option<Fn()>,
    active_color: Option<String>,
    color: Option<String>,
}

// Define props virtualized tree
#[derive(Debug, Clone)]
pub struct PropsVirtualizedTree {
    data: serde_json::Value,
    expanded_nodes: HashSet<String>,
    on_toggle_expand: fn(key_path: Vec<&str>),
    font_size: Option<f64>,
    render_value: fn(node: TreeNode) -> ReactNode,
}

// Define props value labels
#[derive(Debug, Clone)]
pub struct ValueLabelsProps {
    constant_name: Option<String>,
    label: String,
    item_value: serde_json::Value,
    key_path: Vec<&str>,
}

// Define value labels
#[derive(Debug, Clone)]
pub struct ValueLabels {
    arr_label: String;
    item_label: String;
}

// Define shared config type
type SharedConfig = Immutable<RawMessagesPanelConfig>;

// Define shared config actions type
type SharedConfigActions<T extends SharedConfig> = fn(config: Partial<T>) -> ();

// Define use shared raw messages logic props type
type UseSharedRawMessagesLogicProps<T extends SharedConfig> = {
    config: T;
    save_config: SharedConfigActions<T>;
};

// Define use shared raw messages logic result type
type UseSharedRawMessagesLogicResult = {
    topic_ros_path: Option<MessagePath>,
    topic: Option<PlayerTopic>,
    root_structure_item: Option<MessagePathStructureItem>,
    base_item: Option<ReturnType<typeof use_message_data_item>[0]>,
    diff_item: Option<ReturnType<typeof use_message_data_item>[0]>;

    expansion: Option<NodeExpansion>;
    set_expansion: fn(expansion: NodeExpansion | (old: NodeExpansion | Option) => NodeExpansion),
    nodes: HashSet<String>;
    can_expand_all: bool;

    on_topic_path_change: fn(new_topic_path: String);
    on_diff_topic_path_change: fn(new_diff_topic_path: String);
    on_toggle_diff: fn();
    on_toggle_expand_all: fn();
    on_label_click: fn(keypath: Vec<&str>);
};

// Define use renderers props type
type UseValueRendererProps = {
    datatypes:
        | ReadonlyMap<String, {
              readonly name?: Option<String>;
              readonly definitions: readonly { readonly type: String; readonly name: String }[];
            }}
        | Map<String, {
              name?: Option<String>;
              definitions: readonly { readonly type: String; readonly name: String }[];
            }>;

    hover_observer_class_name: String;
    on_topic_path_change: fn(new_topic_path: String);
    open_sibling_panel: OpenSiblingPanel;
};

// Define value renderer function
type ValueRendererFunction = fn(
    structure_item: Option<MessagePathStructureItem>,
    data: Vec<serde_json::Value>,
    queried_data: MessagePathDataItem[],
    label: &str,
    item_value: serde_json::Value,
    ...key_path: Vec<&str>
) -> ReactNode;

// Define render diff label function
type RenderDiffLabelFunction = fn(label: &str, item_value: serde_json::Value) -> ReactNode;

// Define diff labels constant
const diff_labels: {
    ADDED: {
        labelText: String;
        color: String;
        backgroundColor: String;
        invertedBackgroundColor: String;
        indicator: String;
    };
    DELETED: {
        labelText: String;
        color: String;
        backgroundColor: String;
        invertedBackgroundColor: String;
        indicator: String;
    };
    CHANGED: {
        labelText: String;
    };
    ID: {
        labelText: String;
    };
} = {
    ADDED: {
        labelText: "STUDIO_DIFF___ADDED",
        color: "#404047",
        backgroundColor: "#daffe7",
        invertedBackgroundColor: "#182924",
        indicator: "+",
    },
    DELETED: {
        labelText: "STUDIO_DIFF___DELETED",
        color: "#404047",
        backgroundColor: "#ffdee3",
        invertedBackgroundColor: "#3d2327",
        indicator: "-",
    },
    CHANGED: {
        labelText: "STUDIO_DIFF___CHANGED",
    },
    ID: { labelText: "STUDIO_DIFF___ID" },
};
```