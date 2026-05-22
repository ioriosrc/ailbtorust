```rust
use bevy::prelude::*;
use bevy::ui::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(VariablesListState {
            global_variables: GlobalVariables { selected_id: 1234 },
        })
        .insert_resource(BigVariableInitialState {
            global_variables: BigVariable {
                camera_state: CameraState {
                    distance: 20,
                    perspective: true,
                    phi: 60,
                    target: [0.0, 0.0, 0.0],
                    target_offset: [0.0, 0.0, 0.0],
                    target_orientation: [0.0, 0.0, 0.0, 1.0],
                    theta_offset: 45,
                    fovy: 45,
                    near: 0.5,
                    far: 5000,
                },
            },
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Add your setup code here
}
```

Note: This is a basic example of how you might set up the environment in Rust for a DnD-based UI. You would need to implement more details such as rendering, event handling, and updating the state based on user interactions and UI elements.