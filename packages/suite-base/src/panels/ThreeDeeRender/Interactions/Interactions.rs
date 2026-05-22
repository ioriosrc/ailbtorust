```rust
use fluent_ui::icons::{Cursor20Regular};
use mui::material::{Typography, ExpandingToolbar, ToolGroup, ToolGroupFixedSizePane};

use crate::types::{LayoutActions, InteractionData, Pose};
use crate::transforms::ObjectDetails;

const OBJECT_TAB_TYPE: &str = "Selected object";

type TabType = &'static str;

pub type SelectionObject<'a> = {
  object: {
    pose: Pose;
    interaction_data: Option<&'a InteractionData>;
  };
  instance_index: Option<usize>;
};

#[derive(Debug)]
pub struct Props<'a> {
  add_panel: LayoutActions<'a>,
  interactions_tab_type: TabType,
  on_show_topic_settings: fn(&str) -> (),
  selected_object: Option<SelectionObject<'a>>,
  set_interactions_tab_type: fn(TabType),
  timezone: Option<&'a str>,
}

fn InteractionsBaseComponent(props: Props<'_>) -> impl ReactElement {
  let selected_interaction_data = props.selected_object?.object.interaction_data.as_ref();
  let original_message = selected_interaction_data?.original_message;
  let instance_details = selected_interaction_data?.instance_details;

  <ExpandingToolbar
    tooltip="Inspect objects"
    icon={Cursor20Regular}
    selected_tab={props.interactions_tab_type}
    onSelect_tab={|new_selected_tab: TabType| props.set_interactions_tab_type(new_selected_tab)}
  >
    <ToolGroup name={OBJECT_TAB_TYPE}>
      <ToolGroupFixedSizePane>
        {original_message.is_some() && (
          <>
            {selected_interaction_data.topic.is_some() && (
              <TopicLink
                add_panel={props.add_panel}
                on_show_topic_settings={props.on_show_topic_settings}
                topic={selected_interaction_data.topic.unwrap()}
              />
            )}
            {instance_details.is_some() && (
              <ObjectDetails selected_object={instance_details} timezone={props.timezone} />
            )} else {
              <></>
            }}
            <ObjectDetails
              selected_object={original_message}
              interaction_data={selected_interaction_data}
              timezone={props.timezone}
            />
          </>
        )}
        {original_message.is_none() && (
          <Typography variant="body2" color="text.disabled" gutterBottom>
            Click an object in the 3D view to select it.
          </Typography>
        )}
      </ToolGroupFixedSizePane>
    </ToolGroup>
  </ExpandingToolbar>
}

// Wrap the Interactions so that we don't rerender every time any part of the PanelContext config changes, but just the
// one value that we care about.
pub fn Interactions(props: Props<'_>) -> impl ReactElement {
  InteractionsBaseComponent(props)
}
```