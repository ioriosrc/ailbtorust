```rust
use three::{Geometry, MeshStandardMaterial, Material, Object3D};
use std::vec::Vec;

pub type GltfMesh = Mesh<Geometry, Material>;

pub fn remove_lights(model: &mut LoadedModel) {
    let mut lights: Vec<&Object3D> = Vec::new();
    model.traverse(|child| {
        if child.is::<Light>() {
            lights.push(child);
        }
    });
    for light in lights {
        light.remove_from_parent();
        light.dispose();
    }
}

pub fn replace_materials(model: &mut LoadedModel, material: Material) {
    model.traverse_mut(|child| {
        if let Some(mesh_child) = child.downcast_ref::<Mesh<Geometry, Material>>() {
            // Dispose of any allocated textures and the material and swap it with
            // our own material
            mesh_child.material = material.clone();
            if !mesh_child.geometry.attributes.normal.is_empty() {
                mesh_child.compute_vertex_normals();
            }
        } else if let Some(mesh_child) = child.downcast_ref::<Mesh<Geometry, Material>>() {
            for embedded_material in mesh_child.material.as_slice() {
                dispose_standard_material(embedded_material);
            }
            mesh_child.material = material.clone();
            if !mesh_child.geometry.attributes.normal.is_empty() {
                mesh_child.compute_vertex_normals();
            }
        } else {
            println!("Unsupported child type: {:?}", child);
        }
    });
}

/// Generic MeshStandardMaterial dispose function for materials loaded from an external source
fn dispose_standard_material(material: &MeshStandardMaterial) {
    material.map.as_ref().map(|map| map.dispose());
    material.light_map.as_ref().map(|light_map| light_map.dispose());
    material.ao_map.as_ref().map(|ao_map| ao_map.dispose());
    material.emissive_map.as_ref().map(|emissive_map| emissive_map.dispose());
    material.bump_map.as_ref().map(|bump_map| bump_map.dispose());
    material.normal_map.as_ref().map(|normal_map| normal_map.dispose());
    material.displacement_map.as_ref().map(|displacement_map| displacement_map.dispose());
    material.roughness_map.as_ref().map(|roughness_map| roughness_map.dispose());
    material.metalness_map.as_ref().map(|metalness_map| metalness_map.dispose());
    material.alpha_map.as_ref().map(|alpha_map| alpha_map.dispose());
    material.env_map.as_ref().map(|env_map| env_map.dispose());
    material.dispose();
}
```