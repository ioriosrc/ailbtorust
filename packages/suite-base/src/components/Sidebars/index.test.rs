```rust
use crate::components::Sidebars;
use crate::components::SidebarProps;
use crate::test_builders::BasicBuilder;

#[cfg(test)]
mod tests {
    use crate::render_component;
    use jsdom::JSDOM;
    use std::rc::Rc;

    #[test]
    fn should_render_sidebars_with_content_only() {
        let dom = JSDOM::from_file("path/to/some/file.html");
        let window = dom.window().unwrap();
        let document = window.document();

        // Arrange
        let defaultProps: SidebarProps<String, String, String> = {
            items: Default::default(),
            bottom_items: Default::default(),
            selected_key: None,
            onSelect_key: std::rc::Rc::new(|| {}),
            left_items: Default::default(),
            selected_left_key: None,
            onSelect_left_key: std::rc::Rc::new(|| {}),
            left_sidebar_size: 25,
            set_left_sidebar_size: std::rc::Rc::new(|_| {}),
            right_items: Default::default(),
            selected_right_key: None,
            onSelect_right_key: std::rc::Rc::new(|| {}),
            right_sidebar_size: 25,
            set_right_sidebar_size: std::rc::Rc::new(|_| {}),
        };

        let ui = (
            <DndProvider backend={HTML5Backend}>
                <Sidebars
                    items={defaultProps.items}
                    bottom_items={defaultProps.bottom_items}
                    selected_key={defaultProps.selected_key}
                    onSelect_key={defaultProps.onSelect_key.clone()}
                    left_items={defaultProps.left_items}
                    selected_left_key={defaultProps.selected_left_key}
                    onSelect_left_key={defaultProps.onSelect_left_key.clone()}
                    left_sidebar_size={defaultProps.left_sidebar_size}
                    set_left_sidebar_size={defaultProps.set_left_sidebar_size.clone()}
                    right_items={defaultProps.right_items}
                    selected_right_key={defaultProps.selected_right_key}
                    onSelect_right_key={defaultProps.onSelect_right_key.clone()}
                    right_sidebar_size={defaultProps.right_sidebar_size}
                    set_right_sidebar_size={defaultProps.set_right_sidebar_size.clone()}
                />
            </DndProvider>
        );

        // Act
        let body = document.body.unwrap();
        let wrapper = web_sys::ElementRef::from_ref(&body);

        // Assert
        assert!(wrapper.contains(&document.getElementById("sidebars-wrapper").unwrap()));
    }

    #[test]
    fn should_render_sidebars_with_right_sidebar_only() {
        let dom = JSDOM::from_file("path/to/some/file.html");
        let window = dom.window().unwrap();
        let document = window.document();

        // Arrange
        let selected_right_key = BasicBuilder.string();
        let right_items = vec![SidebarItem::new(selected_right_key, BasicBuilder.string())].into_iter().collect::<std::collections::HashMap<_, _>>();

        let props = SidebarProps {
            items: Default::default(),
            bottom_items: Default::default(),
            selected_key: None,
            onSelect_key: std::rc::Rc::new(|| {}),
            left_items: Default::default(),
            selected_left_key: None,
            onSelect_left_key: std::rc::Rc::new(|| {}),
            left_sidebar_size: 25,
            set_left_sidebar_size: std::rc::Rc::new(|_| {}),
            right_items,
            selected_right_key: Some(selected_right_key),
            onSelect_right_key: std::rc::Rc::new(|| {}),
            right_sidebar_size: 25,
            set_right_sidebar_size: std::rc::Rc::new(|_| {}),
        };

        let ui = (
            <DndProvider backend={HTML5Backend}>
                <Sidebars
                    items={props.items}
                    bottom_items={props.bottom_items}
                    selected_key={props.selected_key}
                    onSelect_key={props.onSelect_key.clone()}
                    left_items={props.left_items}
                    selected_left_key={props.selected_left_key}
                    onSelect_left_key={props.onSelect_left_key.clone()}
                    left_sidebar_size={props.left_sidebar_size}
                    set_left_sidebar_size={props.set_left_sidebar_size.clone()}
                    right_items={props.right_items}
                    selected_right_key={props.selected_right_key}
                    onSelect_right_key={props.onSelect_right_key.clone()}
                    right_sidebar_size={props.right_sidebar_size}
                    set_right_sidebar_size={props.set_right_sidebar_size.clone()}
                />
            </DndProvider>
        );

        // Act
        let body = document.body.unwrap();
        let wrapper = web_sys::ElementRef::from_ref(&body);

        // Assert
        assert!(wrapper.contains(&document.getElementById("sidebar-right").unwrap()));
    }

    #[test]
    fn should_render_sidebars_with_left_sidebar_only() {
        let dom = JSDOM::from_file("path/to/some/file.html");
        let window = dom.window().unwrap();
        let document = window.document();

        // Arrange
        let selected_left_key = BasicBuilder.string();
        let left_items = vec![SidebarItem::new(selected_left_key, BasicBuilder.string())].into_iter().collect::<std::collections::HashMap<_, _>>();

        let props = SidebarProps {
            items: Default::default(),
            bottom_items: Default::default(),
            selected_key: None,
            onSelect_key: std::rc::Rc::new(|| {}),
            left_items: left_items,
            selected_left_key: Some(selected_left_key),
            onSelect_left_key: std::rc::Rc::new(|| {}),
            left_sidebar_size: 25,
            set_left_sidebar_size: std::rc::Rc::new(|_| {}),
            right_items: Default::default(),
            selected_right_key: None,
            onSelect_right_key: std::rc::Rc::new(|| {}),
            right_sidebar_size: 25,
            set_right_sidebar_size: std::rc::Rc::new(|_| {}),
        };

        let ui = (
            <DndProvider backend={HTML5Backend}>
                <Sidebars
                    items={props.items}
                    bottom_items={props.bottom_items}
                    selected_key={props.selected_key}
                    onSelect_key={props.onSelect_key.clone()}
                    left_items={props.left_items}
                    selected_left_key={props.selected_left_key}
                    onSelect_left_key={props.onSelect_left_key.clone()}
                    left_sidebar_size={props.left_sidebar_size}
                    set_left_sidebar_size={props.set_left_sidebar_size.clone()}
                    right_items={props.right_items}
                    selected_right_key={props.selected_right_key}
                    onSelect_right_key={props.onSelect_right_key.clone()}
                    right_sidebar_size={props.right_sidebar_size}
                    set_right_sidebar_size={props.set_right_sidebar_size.clone()}
                />
            </DndProvider>
        );

        // Act
        let body = document.body.unwrap();
        let wrapper = web_sys::ElementRef::from_ref(&body);

        // Assert
        assert!(wrapper.contains(&document.getElementById("sidebar-left").unwrap()));
    }
}
```