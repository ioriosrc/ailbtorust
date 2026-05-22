```rust
use bevy::{prelude::*};
use std::f32;

#[derive(Component)]
struct CylinderPrimitive {
    color: Color,
    pose: Transform,
    size: Vec3,
}

#[derive(Component)]
struct RenderableCylinders {
    material: MaterialHandle<MeshStandardMaterialWithInstanceOpacity>,
    outline_material: MaterialHandle<CylinderOutlineMaterial>,
    picking_material: MaterialHandle<CylinderPickingMaterial>,
    instance_opacity: InstanceBufferAttribute,
    instance_bottom_scale: InstanceBufferAttribute,
    instance_top_scale: InstanceBufferAttribute,
}

#[derive(Component)]
struct LayerSettingsEntity {
    show_outlines: bool,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RenderPipelinesPlugin::default(),
            SpatialBundle,
            ColliderBundle,
            PhysicsPlugin,
        ))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let material_handle = MaterialHandle::<MeshStandardMaterialWithInstanceOpacity>::from("mesh_standard_material");
    let outline_material_handle = MaterialHandle::<CylinderOutlineMaterial>::from("cylinder_outline_material");
    let picking_material_handle = MaterialHandle::<CylinderPickingMaterial>::from("cylinder_picking_material");

    commands.spawn((RenderableCylinders {
        material,
        outline_material,
        picking_material,
        instance_opacity: InstanceBufferAttribute::new(),
        instance_bottom_scale: InstanceBufferAttribute::new(),
        instance_top_scale: InstanceBufferAttribute::new(),
    }));

    // Additional setup code to create entities, handle input, etc.
}
```