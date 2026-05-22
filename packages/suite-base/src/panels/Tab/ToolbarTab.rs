```rust
use crate::components::{PanelToolbarConstants, TabActions};
use crate::styles::{makeStyles, Tss};
use stylist::*;

#[derive(Debug, PartialEq)]
pub enum ToolbarTabHighlight {
    Before,
    After,
}

#[derive(Debug, PartialEq)]
pub struct ToolbarTabProps {
    hidden: bool;
    highlight: Option<ToolbarTabHighlight>;
    inner_ref: Ref<Widget<PanelToolbarConstants>>;
    isActive: bool;
    is_dragging: bool;
    actions: TabActions;
    tab_count: usize;
    tab_index: usize;
    tab_title: String;
}

impl ToolbarTabProps {
    pub fn new(
        hidden: bool,
        highlight: Option<ToolbarTabHighlight>,
        inner_ref: Ref<Widget<PanelToolbarConstants>>,
        isActive: bool,
        is_dragging: bool,
        actions: TabActions,
        tab_count: usize,
        tab_index: usize,
        tab_title: String,
    ) -> Self {
        ToolbarTabProps {
            hidden,
            highlight,
            inner_ref,
            isActive,
            is_dragging,
            actions,
            tab_count,
            tab_index,
            tab_title,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ToolbarTabState {
    is_active: bool;
    is_dragging: bool;
    title: String;
    editing_title: bool;
}

impl ToolbarTabState {
    pub fn new(is_active: bool, is_dragging: bool, title: String, editing_title: bool) -> Self {
        ToolbarTabState {
            is_active,
            is_dragging,
            title,
            editing_title,
        }
    }

    pub fn select_tab(&self) {
        self.actions.select_tab(self.tab_index);
    }

    pub fn remove_tab(&self) {
        self.actions.remove_tab(self.tab_index);
    }

    pub fn set_tab_title(&mut self, title: String) {
        self.title = title;
    }
}

#[derive(Debug)]
pub struct ToolbarTabStyles {
    root: Style,
    active: Style,
    dragging: Style,
    hidden: Style,
    input: Style,
    drop_indicator: Style,
    icon_button: Style,
}

impl ToolbarTabStyles {
    pub fn new(tss: &Tss) -> Self {
        ToolbarTabStyles {
            root: tss.create_style("root", |c| {
                c.cursor = "pointer";
                c.color = "secondary";
                c.font_size = "body2.fontSize";
                c.font_weight = "body2.fontWeight";
                c.position = Position::Relative;
                c.display = Display::Flex;
                c.align_items = AlignItems::Center;
                c.width = Dimension::Percentage(100.0);
                c.height = Dimension::Pixels(PanelToolbarConstants::PANEL_TOOLBAR_MIN_HEIGHT as f64);
                c.padding = Dimension::Pixels(0.0, 1.0);
                c.user_select = UserSelect::None;
                c.background_color = Color::Transparent;
                c.max_width = Dimension::Percentage(MAX_TAB_WIDTH as f64);
                c.gap = Dimension::Pixels(0.5);
                c.top = Dimension::Pixels(1.0);

                c.on(":not(.active):hover", |c| {
                    c.color = "primary";
                });
            }),
            active: tss.create_style("active", |c| {
                c.color = "primary";
                c.font_weight = FontWeight::Subtitle2;
                c.background_color = Color::BackgroundPaper;
                c.user_select = UserSelect::All;
                c.z_index = 1;
                c.box_shadow = BoxShadow::new(
                    Shadow::Offset(-1.0, 1.0),
                    Shadow::Color(Color::ActionSelected),
                    Shadow::Blur(2.0),
                    Shadow::Spread(0.0),
                );
            }),
            dragging: tss.create_style("dragging", |c| {
                c.background_color = Color::BackgroundPaper;
                c.border_color = Color::PrimaryMain;
            }),
            hidden: tss.create_style("hidden", |c| {
                c.visibility = Visibility::Hidden;
            }),
            input: tss.create_style("input", |c| {
                c.font = Font::inherit();
                c.color = Color::Inherit();
            }),
            drop_indicator: tss.create_style("drop_indicator", |c| {
                c.position = Position::Absolute;
                c.top = Dimension::Pixels(0.0);
                c.bottom = Dimension::Pixels(1.0);
                c.width = Dimension::Pixels(2.0);
                c.height = Dimension::Percentage(100.0);
                c.background_color = Color::PrimaryMain;
                c.opacity = 0.8;
                c.border_radius = BorderRadius::new(
                    BorderRadius::Circle,
                    BorderRadius::Square,
                    BorderRadius::Half,
                );
                c.z_index = 1;
            }),
            icon_button: tss.create_style("icon_button", |c| {
                c.padding = Dimension::Pixels(0.125, 0.0);
                c.color = "secondary";

                c.on(":hover", |c| {
                    c.color = "primary";
                });
            }),
        }
    }
}

pub fn ToolbarTab(props: ToolbarTabProps) -> Widget<ToolbarTabConstants> {
    let props = props;
    let state = props.state.take();
    let styles = props.styles.take();

    let inner_ref = props.inner_ref.clone();
    let title = props.tab_title;

    let is_active = props.is_active;
    let is_dragging = props.is_dragging;

    let select_tab = move || {
        props.actions.select_tab(props.tab_index);
    };

    let remove_tab = move || {
        props.actions.remove_tab(props.tab_index);
    };

    let set_tab_title = move |title| {
        props.actions.set_tab_title(props.tab_index, title);
    };

    let on_change_title_input = move |ev: Event<KeyboardEvent>| {
        if let Some(value) = ev.target().downcast_ref::<Input>() {
            state.as_mut().unwrap().title = value.value();
        }
    };

    let end_title_editing = move || {
        props.actions.set_tab_title(props.tab_index, state.as_mut().unwrap().title.clone());
    };

    let confirm_new_title = move || {
        set_tab_title(state.as_mut().unwrap().title.clone());
        end_title_editing();
    };

    let reset_title = move || {
        state.as_mut().unwrap().title = title;
        end_title_editing();
    };

    let onKeyDown = move |event: Event<KeyboardEvent>| {
        if event.key() == "Escape" {
            reset_title();
        } else if event.key() == "Enter" {
            confirm_new_title();
        }
    };

    let mut is_dragging_ref = RefCell::new(false);
    let inner_ref = Rc::new(inner_ref);
    let inner_ref_clone = inner_ref.clone();

    let on_mouse_down = move |ev: Event<MouseEvent>| {
        if ev.button() == Button::Left && props.is_active {
            state.as_mut().unwrap().is_dragging = true;
            is_dragging_ref.replace(true);

            request_animation_frame(move || {
                if state.as_ref().unwrap().is_dragging {
                    let rect = inner_ref_clone.inner_viewport_rect();
                    let tab_width = rect.width() as f64;

                    let mut new_title = title.clone();
                    for i in 0..tab_count - props.tab_index - 1 {
                        new_title.push_str(&" ");
                    }

                    set_tab_title(new_title);
                }
            });
        }
    };

    let on_mouse_up = move |ev: Event<MouseEvent>| {
        if ev.button() == Button::Left && props.is_active {
            state.as_mut().unwrap().is_dragging = false;
            is_dragging_ref.replace(false);

            request_animation_frame(move || {
                if state.as_ref().unwrap().is_dragging {
                    let rect = inner_ref_clone.inner_viewport_rect();
                    let tab_width = rect.width() as f64;

                    let mut new_title = title.clone();
                    for i in 0..tab_count - props.tab_index - 1 {
                        new_title.push_str(&" ");
                    }

                    set_tab_title(new_title);
                }
            });
        }
    };

    let on_mouse_move = move |ev: Event<MouseEvent>| {
        if props.is_active && is_dragging_ref.take() {
            let rect = inner_ref_clone.inner_viewport_rect();
            let tab_width = rect.width() as f64;

            let mut new_title = title.clone();
            for i in 0..tab_count - props.tab_index - 1 {
                new_title.push_str(&" ");
            }

            set_tab_title(new_title);
        }
    };

    let styles: ToolbarTabStyles = if let Some(styles) = &props.styles {
        styles.clone()
    } else {
        ToolbarTabStyles::new(&default_tss());
    };

    let root_style = styles.root;
    let active_style = styles.active;
    let dragging_style = styles.dragging;
    let hidden_style = styles.hidden;
    let input_style = styles.input;
    let drop_indicator_style = styles.drop_indicator;
    let icon_button_style = styles.icon_button;

    Widget::div(root_style, move |_| {
        if props.highlight != None {
            Widget::div(drop_indicator_style, |_| {});
        }

        Widget::input(input_style, move |el| {
            el.value(props.title.clone());
            el.read_only(!props.editing_title);
            el.placeholder("Enter tab name");
            el.value(props.title.clone());
            el.on("change", on_change_title_input);
            el.on("blur", end_title_editing);
            el.on("keydown", onKeyDown);

            if props.editing_title {
                el.input_ref(|el| input_ref.replace(el));
            }
        });

        if props.is_active {
            Widget::icon_button(icon_button_style, |_| {});
        }
    });
}
```