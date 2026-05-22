```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (handle_speed_change, update_player_speed))
        .run();
}

#[derive(Component)]
struct SpeedControl;

#[system]
async fn handle_speed_change(
    mut commands: Commands,
    mut messages: EventReader<SetSpeed>,
    mut players: ResMut<Players>,
) {
    for speed_event in messages.iter() {
        if let Ok(player_id) = speed_event.player_id {
            players.get_mut(player_id).unwrap().speed = speed_event.speed;
        }
    }

    commands.spawn(SpeedControl);
}

#[system]
async fn update_player_speed(
    mut players: ResMut<Players>,
    mut controls: Query<&SpeedControl, With<PlayerControl>>,
) {
    for player in players.iter() {
        if let Ok(player_control) = controls.get_one(player.id).await {
            player.control.set_speed(player_control.speed);
        }
    }
}

struct Players(Vec<Entity>);

#[system]
async fn add_player(mut commands: Commands) {
    for _ in 0..10 {
        commands.spawn(PlayerControl {});
    }
}

#[derive(Component)]
struct PlayerControl;

fn setup_speed_controls(mut commands: Commands, mut layout: ResMut<CurrentLayout>) {
    commands
        .spawn(SpeedControl)
        .insert(layout.center());
}
```