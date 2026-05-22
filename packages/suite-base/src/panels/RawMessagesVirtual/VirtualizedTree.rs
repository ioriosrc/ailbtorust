```rust
use leptos::*;
use std::cmp;

#[component]
pub fn VirtualizedTree(
  data: Children<RawMessage>,
  expanded_nodes: Signal<bool, String>,
  on_toggle_expand: Callback<String, ()>,
) -> impl IntoView {
  let classes = use_styles();
  let parent_ref = use_node();

  // Flat tree data
  let flat_data = useMemo({
    || flatten_tree_data(data.into_iter(), expanded_nodes.get().as_str()),
  });

  let get_scroll_element = move || Some(parent_ref.as_ref());

  // Virtualizer
  let virtualizer = create_virtualizer(
    |node| match node {
      RawMessage::Node(node) => {
        let depth = node.depth() as usize;
        let indent_size = depth * TREE_NODE_INDENTATION;
        let height = node.content().unwrap_or_default().clone();
        Some((height, indent_size))
      }
      _ => None,
    },
    get_scroll_element.clone(),
    |item| item.0,
    (node: &RawMessage) -> usize {
      match node {
        RawMessage::Node(node) => node.depth() as usize,
        _ => 0,
      }
    },
    (node: &RawMessage) -> usize {
      match node {
        RawMessage::Node(node) => node.content().unwrap_or_default().clone(),
        _ => String::new(),
      }
    },
  );

  // Render items
  let items = virtualizer.items();

  view! {
    <div ref=parent_ref class=classes.container>
      <div class={classes.inner_wrapper} style="height: {{ virtualizer.total_size() }}px;">
        {items.map(|item| {
          let node = flat_data.get(item.index()).unwrap();
          if !node.is_expandable() || !expanded_nodes.contains_key(node.label()) {
            return None;
          }

          let is_expanded = expanded_nodes.get_key_value(&node.label());
          let icon = match is_expanded {
            Some(true) => COLLAPSED_ICON,
            Some(false) => EXPANDED_ICON,
            None => COLLAPSED_ICON, // Default to collapsed if the key is not in the map
          };

          view! {
            <div
              key={item.key}
              data-index={item.index}
              ref=virtualizer.measure_element
              class={classes.row}
              style={{
                transform: format!("translateY({item.start}px)"),
                paddingLeft: node.depth() * TREE_NODE_INDENTATION,
              }}
            >
              <span class={classes.expand_button}>
                <button
                  on_click=move |_| {
                    on_toggle_expand.emit(node.label().clone());
                  }
                  on_key_down=move |e| {
                    if e.key == "Enter" || e.key == " " {
                      e.preventDefault();
                      on_toggle_expand.emit(node.label().clone());
                    }
                  }
                  tabindex="0"
                  aria-expanded={is_expanded}
                  aria-label="{if is_expanded { "Collapse" } else { "Expand" }} {node.label()}"
                  class={classes.span_button}
                >
                  {icon}
                </button>
              </span>
              <span class={classes.key}>{node.label()}</span>
              <div class={classes.value_container}>{node.content().unwrap_or_default()}</div>
            </div>
          }
        })}
      </div>
    </div>
  }
}

#[component]
fn FlattenTreeData(
  data: Vec<RawMessage>,
  expanded_nodes: String,
) -> impl IntoView {
  view! {
    <ul style="list-style-type: none; padding-left: 0;">
      {for item in data.iter().flatten() {
        let is_expanded = expanded_nodes.contains_key(item.label());
        let icon = match is_expanded {
          Some(true) => COLLAPSED_ICON,
          Some(false) => EXPANDED_ICON,
          None => COLLAPSED_ICON, // Default to collapsed if the key is not in the map
        };

        view! {
          <li style="padding-left: {{ item.depth() * TREE_NODE_INDENTATION }}px;">
            <span class={if is_expanded { "icon expanded" } else { "icon collapsed" }}>{icon}</span>
            <a href="#" onclick=move |_| on_toggle_expand.emit(item.label().clone())>{item.label()}</a>
            {
              if let Some(content) = &item.content() {
                view! {
                  <div class="content">{content}</div>
                }
              }
            }
          </li>
        }
      }}
    </ul>
  }
}

fn flatten_tree_data(data: Vec<RawMessage>, expanded_nodes: &str) -> Vec<RawMessage> {
  data
    .iter()
    .flatten()
    .filter(|node| !expanded_nodes.contains_key(node.label()))
    .collect::<Vec<_>>()
}

const COLLAPSED_ICON = "collapsed-icon";
const EXPANDED_ICON = "expanded-icon";
const SCROLLL_OVERSCAN = 10;
const TREE_NODE_INDENTATION = 20;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_flatten_tree_data() {
    let data = vec![
      RawMessage::Node(RawMessageNodeData {
        depth: 0,
        label: "Root",
        content: Some("Root Content"),
      }),
      RawMessage::Node(RawMessageNodeData {
        depth: 1,
        label: "Child 1",
        content: Some("Child 1 Content"),
      }),
      RawMessage::Node(RawMessageNodeData {
        depth: 1,
        label: "Child 2",
        content: Some("Child 2 Content"),
      }),
    ];

    let expanded_nodes = "Root".to_string();
    let expected_flattened_data = vec![
      RawMessage::Node(RawMessageNodeData {
        depth: 0,
        label: "Root",
        content: Some("Root Content"),
      }),
      RawMessage::Node(RawMessageNodeData {
        depth: 1,
        label: "Child 2",
        content: Some("Child 2 Content"),
      }),
    ];

    assert_eq!(flatten_tree_data(data, &expanded_nodes), expected_flattened_data);
  }
}
```