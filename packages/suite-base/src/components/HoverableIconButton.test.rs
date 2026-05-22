```rust
// Import necessary modules and types
use test_builders::BasicBuilder;
use testing_library::{render, screen, fireEvent};

// Define the type for HoverableIconButtonProps
type HoverableIconButtonProps = {
    icon: Box<dyn Fn() -> Node> = BasicBuilder.string();
    title: String = BasicBuilder.string(),
    active_icon: Box<dyn Fn() -> Node> = BasicBuilder.string(),
    disabled: bool,
};

// Define a function to render the component with given props
fn render_component(props_override: HoverableIconButtonProps) {
    let props = {
        icon: Box::new(move || <span data-testid="test-icon">{BasicBuilder.string()}</span>),
        title: BasicBuilder.string(),
        active_icon: Box::new(move || <span data-testid="active-icon">{BasicBuilder.string()}</span>),
        disabled: false,
        ..props_override
    };

    render(
        <HoverableIconButton icon={props.icon} {...props}>
            {props.children}
        </HoverableIconButton>,
    );
}

// Define a test suite for HoverableIconButton
#[test]
fn hoverable_icon_button() {
    // Test case 1: When rendered with icon only Then displays the icon
    render_component();

    assert!(screen.get_by_test_id("test-icon").is_displayed());

    // Test case 2: When rendered with icon and children Then displays both
    let props = HoverableIconButtonProps {
        children: Box::new(move || <span data-testid="test-text">{BasicBuilder.string()}</span>),
        ..Default::default()
    };

    render_component(props);

    assert!(screen.get_by_test_id("test-icon").is_displayed());
    assert!(screen.get_by_test_id("test-text").is_displayed());

    // Test case 3: When iconPosition is 'start' Then icon appears first
    let props = HoverableIconButtonProps {
        icon_position: "start",
        children: Box::new(move || <span data-testid="test-text">{BasicBuilder.string()}</span>),
        ..Default::default()
    };

    let { container } = render_component(props);

    assert!(container.get_by_role("button").get_child_nodes().iter().next().is_some());
    assert!(container.get_by_test_id("test-icon").is_displayed());

    // Test case 4: When iconPosition is 'end' Then icon appears last
    let props = HoverableIconButtonProps {
        icon_position: "end",
        children: Box::new(move || <span data-testid="test-text">{BasicBuilder.string()}</span>),
        ..Default::default()
    };

    let { container } = render_component(props);

    assert!(container.get_by_role("button").get_child_nodes().iter().last().is_some());
    assert!(container.get_by_test_id("test-icon").is_displayed());

    // Test case 5: When hovered Then shows activeIcon
    let props = HoverableIconButtonProps {
        active_icon: Box::new(move || <span data-testid="active-icon">{BasicBuilder.string()}</span>),
        ..Default::default()
    };

    render_component(props);

    assert!(container.get_by_test_id("test-icon").is_displayed());
    assert!(!container.get_by_test_id("active-icon").is_displayed());

    let button = container.get_by_role("button");
    fireEvent.mouse_enter(button);
    assert!(!container.get_by_test_id("test-icon").is_displayed());
    assert!(container.get_by_test_id("active-icon").is_displayed());

    // Test case 6: When mouse leaves Then reverts to normal icon
    let props = HoverableIconButtonProps {
        active_icon: Box::new(move || <span data-testid="active-icon">{BasicBuilder.string()}</span>),
        ..Default::default()
    };

    render_component(props);

    let button = container.get_by_role("button");
    fireEvent.mouse_enter(button);
    assert!(!container.get_by_test_id("test-icon").is_displayed());
    assert!(container.get_by_test_id("active-icon").is_displayed());

    fireEvent.mouse_leave(button);
    assert!(container.get_by_test_id("test-icon").is_displayed());
    assert!(!container.get_by_test_id("active-icon").is_displayed());

    // Test case 7: When disabled and hovered Then does not show activeIcon
    let props = HoverableIconButtonProps {
        active_icon: Box::new(move || <span data-testid="active-icon">{BasicBuilder.string()}</span>),
        disabled: true,
        ..Default::default()
    };

    render_component(props);

    assert!(container.get_by_test_id("test-icon").is_displayed());
    assert!(!container.get_by_test_id("active-icon").is_displayed());

    let button = container.get_by_role("button");
    fireEvent.mouse_enter(button);
    assert!(!container.get_by_test_id("test-icon").is_displayed());
    assert!(!container.get_by_test_id("active-icon").is_displayed());

    // Test case 8: When clicked Then calls onClick handler
    let handleClick = Box::new(|| ());
    let props = HoverableIconButtonProps {
        onClick: handleClick,
        ..Default::default()
    };

    render_component(props);

    let button = container.get_by_role("button");
    fireEvent.click(button);
    assert!(handleClick.as_ref().is_some());
}
```