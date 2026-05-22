```rust
use bevy::prelude::*;

#[derive(Component)]
struct PanelOverlay {
    variant: String,
}

pub struct PanelOverlayPlugin;

impl Plugin for PanelOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(StartupSystem, setup);
    }

    fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let delete_icon = asset_server.load("icons/delete_20_regular.svg");
        let tab_desktop_icon = asset_server.load("icons/tab_desktop_20_regular.svg");
        let tab_desktop_multiple_icon = asset_server.load("icons/tab_desktop_multiple_20_regular.svg");
        let table_simple_icon = asset_server.load("icons/table_simple_20_regular.svg");

        commands.spawn((
            PanelOverlay {
                variant: "validDropTarget".to_string(),
            },
            BackgroundContent,
            Icon {
                texture: delete_icon.clone(),
            },
        ));

        commands.spawn((PanelOverlay {
            variant: "invalidDropTarget".to_string(),
        }, BackgroundContent, Icon {
            texture: delete_icon.clone(),
        }));

        commands.spawn((
            PanelOverlay {
                variant: "selected".to_string(),
            },
            BackgroundContent,
            Icon {
                texture: tab_desktop_icon.clone(),
            },
            Icon {
                texture: tab_desktop_multiple_icon.clone(),
            },
        ));

        commands.spawn((
            PanelOverlay {
                variant: "selected".to_string(),
            },
            BackgroundContent,
            Icon {
                texture: table_simple_icon.clone(),
            },
            Button {
                text: Text::new("Split panel"),
                icon: Icon {
                    texture: delete_icon.clone(),
                },
            },
            Button {
                text: Text::new("Remove panel"),
                icon: Icon {
                    texture: delete_icon.clone(),
                },
                style: Style {
                    color: Color::RED,
                },
            },
        ));
    }
}

struct BackgroundContent;

impl Component for BackgroundContent {}

struct Icon {
    texture: Handle<Image>,
}

fn setup(mut commands: Commands) {
    let delete_icon = commands.add_image("icons/delete_20_regular.svg").id();
    let tab_desktop_icon = commands.add_image("icons/tab_desktop_20_regular.svg").id();
    let tab_desktop_multiple_icon = commands.add_image("icons/tab_desktop_multiple_20_regular.svg").id();
    let table_simple_icon = commands.add_image("icons/table_simple_20_regular.svg").id();

    commands.spawn((
        Icon {
            texture: delete_icon,
        },
    ));

    commands.spawn((
        Icon {
            texture: tab_desktop_icon,
        },
    ));

    commands.spawn((
        Icon {
            texture: tab_desktop_multiple_icon,
        },
    ));

    commands.spawn((
        Icon {
            texture: table_simple_icon,
        },
    ));
}
```