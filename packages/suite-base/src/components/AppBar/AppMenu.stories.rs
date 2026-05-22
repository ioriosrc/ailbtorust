```rust
use mui::material::{PopoverPosition, PopoverReference};
use mui::testing_library::{user_event, within};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use crate::suite_base::context::PlayerSelectionContext;
use crate::suite_base::providers::CurrentLayoutProvider::MockCurrentLayoutProvider;
use crate::suite_base::providers::WorkspaceContextProvider;

use crate::components::AppBar::AppMenu;

type StoryArgs = {
  handleClose: fn();
  anchorEl?: HTMLElement;
  anchorReference?: PopoverReference;
  anchorPosition?: PopoverPosition;
  disablePortal?: bool;
  open: bool;
  testId?: string;
};

pub struct AppMenuStory;

impl mui::testing_library::Story for AppMenuStory {
    type Meta = StoryArgs;
    type Component = AppMenu;

    fn render(&self, args: Self::Meta) -> mui::testing_library::RenderResult<Self::Component> {
        let mock_current_layout_provider = MockCurrentLayoutProvider {};
        let workspace_context_provider = WorkspaceContextProvider {};
        let player_selection_context = PlayerSelectionContext { selection: vec![] };

        mui::testing_library::render(
            <MockCurrentLayoutProvider>
                <WorkspaceContextProvider>
                    <PlayerSelectionContext.Provider value={player_selection_context}>
                        <AppMenu {...args} />
                    </PlayerSelectionContext.Provider>
                </WorkspaceContextProvider>
            </MockCurrentLayoutProvider>,
        )
    }
}

impl mui::testing_library::Play for AppMenuStory {
    fn play(&self, canvas_element: &mui::testing_library::RenderResult<Self::Component>) {
        if let Some(canvas) = within(canvas_element.root) {
            user_event::hover(&canvas.find_with_text("App Menu File")?).perform();
        }
    }
}

// Connection
const player_selection: PlayerSelection = {
    select_source: |_| {},
    select_recent: |_| {},
    recent_sources: vec![
        // prettier-ignore
        { id: "1111", title: "NuScenes-v1.0-mini-scene-0655-reallllllllly-long-name-8829908290831091.bag", },
        { id: "2222", title: "http://localhost:11311", label: "ROS 1" },
        { id: "3333", title: "ws://localhost:9090/", label: "Rosbridge (ROS 1 & 2)" },
        { id: "4444", title: "ws://localhost:8765", label: "Foxglove WebSocket" },
        { id: "5555", title: "2369", label: "Velodyne Lidar" },
        { id: "6666", title: "THIS ITEM SHOULD BE HIDDEN IN STORYBOOKS", label: "!!!!!!!!!!!!" },
    ],
    available_sources: [],
};

type Story = mui::testing_library::Story<StoryArgs>;

impl mui::testing_library::Story for AppMenu {
    type Meta = StoryArgs;
    type Component = AppMenu;

    fn render(&self, args: Self::Meta) -> mui::testing_library::RenderResult<Self::Component> {
        AppMenuStory.render(self)
    }
}

// Dark
impl mui::testing_library::Play for AppMenu {
    fn play(&self, canvas_element: &mui::testing_library::RenderResult<Self::Component>) {
        if let Some(canvas) = within(canvas_element.root) {
            user_event::hover(&canvas.find_with_text("App Menu File")?).perform();
        }
    }
}

impl mui::testing_library::Play for AppMenu {
    fn play(&self, canvas_element: &mui::testing_library::RenderResult<Self::Component>) {
        if let Some(canvas) = within(canvas_element.root) {
            user_event::hover(&canvas.find_with_text("App Menu View")?).perform();
        }
    }
}

impl mui::testing_library::Play for AppMenu {
    fn play(&self, canvas_element: &mui::testing_library::RenderResult<Self::Component>) {
        if let Some(canvas) = within(canvas_element.root) {
            user_event::hover(&canvas.find_with_text("App Menu Help")?).perform();
        }
    }
}
```