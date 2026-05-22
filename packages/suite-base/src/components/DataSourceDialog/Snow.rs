```rust
use bevy::prelude::*;
use rand::{thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Points {})
        .insert_resource(Configuration { effect: "snow", yspeed: 0.1 })
        .run();
}

#[derive(Resource)]
struct Points {
    positions: Vec3<f32>,
    colors: Vec3<f32>,
    sizes: Vec<f32>,
    phases: Vec<f32>,
}

#[derive(Component)]
struct Point {
    position: Vec3<f32>,
    color: Vec3<f32>,
    size: f32,
    phase: f32,
}

fn snow(mut points: ResMut<Points>, config: Res<Configuration>) -> Result<(), anyhow::Error> {
    let mut rng = thread_rng();

    let POINT_COUNT = if config.effect == "snow" { 75 } else { 100 };

    for i in 0..POINT_COUNT {
        let position = Vec3::new(rng.next_f32(), rng.next_f32(), -1.0);
        let color = if config.effect == "snow" {
            Vec3::splat(1.0)
        } else {
            Color::hsl(rng.gen_range(0.0, 1.0), 1.0, 0.5).as_rgb_vec()
        };
        let size = rng.next_f32();
        let phase = rng.gen_range(0.0, f32::PI);

        points.positions.push(position);
        points.colors.push(color);
        points.sizes.push(size);
        points.phases.push(phase);
    }

    Ok(())
}

fn update_points(
    mut points: ResMut<Points>,
    config: Res<Configuration>,
    time: Res<Time>,
) -> Result<(), anyhow::Error> {
    let mut rng = thread_rng();

    for i in 0..points.positions.len() {
        let point = &mut points.positions[i];
        let color = &mut points.colors[i];
        let size = &mut points.sizes[i];
        let phase = &mut points.phases[i];

        if config.effect == "snow" {
            let x = mix(-1.0, 1.0, (point.x + sin(phase + time.elapsed_secs() * config.yspeed) * config.yspeed + time.elapsed_secs() * config.yspeed * rng.gen_range(0.0, 1.0)).fract());
            let y = mix(-1.0, 1.0, (point.y - time.elapsed_secs() * config.yspeed * mix(0.2, 1.0, size).fract()).fract());
            point.x = x;
            point.y = y;
        } else {
            let x = mix(-1.0, 1.0, (point.x + sin(phase + time.elapsed_secs() * config.yspeed) * config.yspeed + time.elapsed_secs() * config.yspeed * rng.gen_range(0.0, 1.0)).fract());
            let y = mix(-1.0, 1.0, (point.y - time.elapsed_secs() * config.yspeed * mix(0.2, 1.0, size).fract()).fract());
            point.x = x;
            point.y = y;
        }

        *size = if *size < config.min_size { config.min_size } else { config.max_size };
    }

    Ok(())
}

fn create_points(
    mut points: ResMut<Points>,
    config: Res<Configuration>,
) -> Result<(), anyhow::Error> {
    for i in 0..points.positions.len() {
        let point = &mut points.positions[i];
        let color = &mut points.colors[i];
        let size = &mut points.sizes[i];
        let phase = &mut points.phases[i];

        point.x = rng.gen_range(-1.0, 1.0);
        point.y = rng.gen_range(-1.0, 1.0);
        point.z = -1.0;
        color.set_hsl(rng.gen_range(0.0, 1.0), 1.0, 0.5).as_rgb_vec();
        size = rng.gen_range(config.min_size, config.max_size);
        phase = rng.gen_range(0.0, f32::PI);
    }

    Ok(())
}

fn animate_points(
    mut points: ResMut<Points>,
    time: Res<Time>,
) -> Result<(), anyhow::Error> {
    let mut rng = thread_rng();

    for i in 0..points.positions.len() {
        let point = &mut points.positions[i];
        let color = &mut points.colors[i];
        let size = &mut points.sizes[i];
        let phase = &mut points.phases[i];

        if config.effect == "snow" {
            let x = mix(-1.0, 1.0, (point.x + sin(phase + time.elapsed_secs() * config.yspeed) * config.yspeed + time.elapsed_secs() * config.yspeed * rng.gen_range(0.0, 1.0)).fract());
            let y = mix(-1.0, 1.0, (point.y - time.elapsed_secs() * config.yspeed * mix(0.2, 1.0, size).fract()).fract());
            point.x = x;
            point.y = y;
        } else {
            let x = mix(-1.0, 1.0, (point.x + sin(phase + time.elapsed_secs() * config.yspeed) * config.yspeed + time.elapsed_secs() * config.yspeed * rng.gen_range(0.0, 1.0)).fract());
            let y = mix(-1.0, 1.0, (point.y - time.elapsed_secs() * config.yspeed * mix(0.2, 1.0, size).fract()).fract());
            point.x = x;
            point.y = y;
        }

        *size = if *size < config.min_size { config.min_size } else { config.max_size };
    }

    Ok(())
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Points {})
        .insert_resource(Configuration { effect: "snow", yspeed: 0.1 })
        .run();
}
```