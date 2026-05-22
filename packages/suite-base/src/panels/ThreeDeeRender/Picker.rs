```rust
use std::ops::{AddAssign, BitXorAssign};
use three::{Camera, Material, Object3D, PerspectiveCamera};

// The width and height of the output viewport. This could be 1 to sample a
// single pixel, but GL_POINTS with a >1 point size would be clipped
const PIXEL_WIDTH: usize = 31;
const TEMP_RESOLUTION = (0, 0);

const WHITE_COLOR = THREE.Color::new(0xffffff);

// This works around an incorrect method definition, where passing null is not allowed
pub struct NullScene;

impl Object3D for NullScene {}

// A trait to be used by different types of materials
trait MaterialWithObjectId {
    fn set_object_id(&mut self, object_id: f64);
}

// The picking material for meshes
struct PickingMaterial {
    uniform_values: HashMap<f64, f64>,
}

impl PickingMaterial {
    fn new() -> Self {
        Self {
            uniform_values: HashMap::new(),
        }
    }

    fn set_object_id(&mut self, object_id: f64) {
        // Set the objectId uniform in the material
        self.uniform_values.insert("object_id", object_id);
    }
}

// The picking material for instanced meshes
struct InstancePickingMaterial {
    uniform_values: HashMap<f64, f64>,
}

impl InstancePickingMaterial {
    fn new() -> Self {
        Self {
            uniform_values: HashMap::new(),
        }
    }

    fn set_object_id(&mut self, object_id: f64) {
        // Set the objectId uniform in the material
        self.uniform_values.insert("object_id", object_id);
    }
}

// A trait to handle rendering buffers directly
trait RenderBufferDirect<Material> {
    fn render_buffer_direct(
        &self,
        camera: PerspectiveCamera,
        scene: NullScene,
        geometry: &three::geometry::Geometry,
        material: &Material,
        object: &Object3D,
        group: Option<&Object3D>,
    );
}

// A trait to handle rendering items
trait RenderItem<Material> {
    fn set_object_id(&mut self, object_id: f64);
}

// The picking renderer
pub struct PickingRenderer {
    camera: PerspectiveCamera,
    scene: NullScene,
    render_materials: HashMap<f64, PickingMaterial>,
    instance_picking_material: InstancePickingMaterial,
}

impl PickingRenderer {
    pub fn new(camera: PerspectiveCamera) -> Self {
        Self {
            camera,
            scene: NullScene,
            render_materials: HashMap::new(),
            instance_picking_material: InstancePickingMaterial::new(),
        }
    }

    // Set the object id for a material
    pub fn set_object_id(&mut self, material: &mut MaterialWithObjectId, object_id: f64) {
        if let MaterialWithObjectId::set_object_id = material {
            material.set_object_id(object_id);
        }
    }

    // Render a single item in the picking pass
    pub fn render_item(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.render_materials[i_object_id as usize], object, None);
    }

    // Render an instance item in the picking pass
    pub fn render_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the opaque pass
    fn render_opaque_item(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.render_materials[i_object_id as usize], object, None);
    }

    // Render an instance item in the transparent pass
    fn render_transparent_item(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.render_materials[i_object_id as usize], object, None);
    }

    // Render an instance item in the transparent pass
    fn render_transparent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_item(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.render_materials[i_object_id as usize], object, None);
    }

    // Render an instance item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking_material, object, None);
    }

    // Render a single item in the translucent pass
    fn render_translucent_instance(
        &self,
        material: &MaterialWithObjectId,
        geometry: &three::geometry::Geometry,
        object: &Object3D,
    ) {
        let mut i_object_id = f64::NAN;
        if let MaterialWithObjectId::set_object_id = material {
            i_object_id = material.get_object_id().unwrap_or(f64::NAN);
        }
        self.render_buffer_direct(self.camera, self.scene, geometry, &self.instance_picking