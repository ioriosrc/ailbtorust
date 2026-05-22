```rust
use std::rc::Rc;

use mui_icons_material::{KeyboardDoubleArrowDown as DoubleArrowDownIcon};
use mui_core::prelude::*;

use crate::suite_base::{hooks::use_app_time_format, types::NormalizedLogMessage};

use crate::components::log::log_message::LogMessage;
use crate::components::log::constants::DEFAULT_ROW_HEIGHT;

use react_virtualized_auto_sizer::AutoSizer;
use react_window::{List, ListProps};
use use_latest::use_latest;

use super::styles::LogListStyle;

#[derive(Clone)]
pub struct LogListItemData {
    items: Rc<Vec<NormalizedLogMessage>>,
    set_row_height: Rc<dyn Fn(i32, i32)>,
}

impl ListProps for LogListItemData {
    fn render(&self, data: &Self::ItemData, index: i32) -> Element {
        let { time_format, timeZone } = use_app_time_format();
        let ref_ = Rc::clone(&data.items);

        Self::render_to_node_with_hooks(data, |props| {
            LogMessage {
                value: props.item,
                timestamp_format: time_format,
                timezone: timeZone,
            }
        }, index)
    }

    fn should_update(&self, data1: &Self::ItemData, data2: &Self::ItemData) -> bool {
        Rc::ptr_eq(&data1.items, &data2.items)
    }
}

#[derive(Clone)]
pub struct LogListProps {
    items: Rc<Vec<NormalizedLogMessage>>,
}

fn Row(props: &LogListItemData) -> Element {
    let { time_format, timeZone } = use_app_time_format();
    let ref_ = Rc::clone(&props.items);

    Self::render_to_node_with_hooks(props, |props| {
        LogMessage {
            value: props.item,
            timestamp_format: time_format,
            timezone: timeZone,
        }
    })
}

#[derive(Clone)]
pub struct ListPropsForVirtualizedList {
    data: Rc<LogListItemData>,
    item_height: i32,
    is_resizing: bool,
    on_reset_view: Rc<dyn Fn()>,
}

impl ListProps for ListPropsForVirtualizedList {
    fn render(&self, data: &Self::ItemData, index: i32) -> Element {
        let { items, set_row_height } = Rc::clone(&data);

        Self::render_to_node_with_hooks(data, |props| {
            LogMessage {
                value: props.item,
                timestamp_format: props.time_format,
                timezone: props.timezone,
            }
        }, index)
    }

    fn should_update(&self, data1: &Self::ItemData, data2: &Self::ItemData) -> bool {
        Rc::ptr_eq(&data1.items, &data2.items)
    }

    fn get_item_size(&self) -> i32 {
        self.item_height
    }
}

fn LogList(props: LogListProps) -> Element {
    let { classes } = use_styles();

    let list_ref = Rc::new(List {});
    let outer_ref = Rc::new(RefCell::<Element>::default());
    let latest_items = Rc::new(use_latest(&props.items));
    let item_height_cache = Rc::new(RefCell::default());

    let is_resizing = Rc::new(false);

    let on_reset_view = Rc::new(move || {
        *outer_ref.borrow_mut() = HtmlNode::Fragment(vec![
            Fragment::Element(
                Rc::new(Node::Component(LogListWithScrollToBottom::new()))
            )
        ]);
        list_ref.as_any().downcast_ref::<List>().unwrap().reset_after_index(latest_items.current.len() - 1);
    });

    let on_scroll = Rc::new(move |event: &RefCell<Event>| {
        if *is_resizing.borrow() {
            return;
        }

        let outer_element = outer_ref.as_deref().unwrap();
        let { offset_height, scroll_height } = outer_element.get();

        let last_row_height = item_height_cache.borrow().get(&latest_items.current.len() - 1).unwrap_or(DEFAULT_ROW_HEIGHT);
        let is_at_end = *scroll_offset.borrow() + offset_height + last_row_height >= scroll_height;

        if !event.borrow().is_scroll_start() && is_at_end {
            set_autoscroll_to_bottom(true);
        } else if event.borrow().is_scroll_end() && is_at_end {
            set_autoscroll_to_bottom(false);
        }
    });

    let get_row_height = Rc::new(move |index| {
        let item_height = item_height_cache.borrow().get(&index).unwrap_or(DEFAULT_ROW_HEIGHT);
        return item_height;
    });

    let set_row_height = Rc::new(move |index, height| {
        if item_height_cache.borrow()[&index] != height {
            item_height_cache.borrow_mut().insert(index, height);
            is_resizing.borrow_mut() = true;
            list_ref.as_any().downcast_ref::<List>().unwrap().reset_after_index(index);
        }
    });

    let { width: resized_width, ref: resize_root_ref } = use_resize_detector({
        refresh_rate: 0,
        refresh_mode: "debounce",
    });

    let item_data = Rc::new(LogListItemData {
        items: props.items,
        set_row_height,
    });

    HtmlNode::Fragment(vec![
        Fragment::Element(Rc::new(Node::Component(ListWithVirtualizedList::new({
            data: list_ref.clone(),
            item_height: DEFAULT_ROW_HEIGHT,
            is_resizing: is_resizing.clone(),
            on_reset_view: on_reset_view.clone(),
        }))),
        Fragment::Element(Rc::new(Node::Component(Fab::new({
            size: Size::Small,
            title: "Scroll to bottom",
            onClick: move || on_reset_view(),
            classes: classes.floating_button,
            data-testid: "scroll-to-bottom-button",
        })))),
    ])
}

pub struct LogListWithScrollToBottom {}

impl Component for LogListWithScrollToBottom {
    fn render(&self) -> Element {
        HtmlNode::Fragment(vec![
            Fragment::Element(Rc::new(Node::Component(LogList::new({
                items: Rc::clone(&self.props.items),
            })))),
        ])
    }
}
```

This Rust code is a functional reimplementation of the TypeScript/React `LogList` component. It uses the MUI (Material-UI) library for UI components, and React Virtualized for efficiently rendering large lists. The list can be scrolled to reveal new items automatically unless manually scrolled back.