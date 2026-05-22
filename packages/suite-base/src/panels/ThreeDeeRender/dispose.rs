```rust
use web_sys::{ThreeObject3D, ThreeMesh, ThreeGeometry, ThreeTexture};

pub fn dispose_material(material: &mut ThreeMaterial) {
    let mut dispose_list = Vec::new();
    
    if material.is<MeshStandardMaterial>() {
        dispose_list.push(&material.map);
        dispose_list.push(&material.light_map);
        dispose_list.push(&material.ao_map);
        dispose_list.push(&material.emissive_map);
        dispose_list.push(&material.bump_map);
        dispose_list.push(&material.normal_map);
        dispose_list.push(&material.displacement_map);
        dispose_list.push(&material.roughness_map);
        dispose_list.push(&material.metalness_map);
        dispose_list.push(&material.alpha_map);
        dispose_list.push(&material.env_map);
    }

    for tex in &mut dispose_list {
        if let Some(tex) = tex.as_mut() {
            tex.dispose();
        }
    }

    material.dispose();
}

pub fn dispose_meshes_recursive(object: &mut ThreeObject3D) {
    object.traverse(|child| {
        if child.is<Mesh>() {
            let mesh = child.downcast::<ThreeMesh>().unwrap();
            mesh.geometry.dispose();

            if let Some(materials) = &mesh.materials {
                for material in materials {
                    if let Some(material) = material.as_mut() {
                        dispose_material(material);
                    }
                }
            } else if let Some(material) = mesh.material.as_mut() {
                dispose_material(material);
            }
        }
    });
}
```