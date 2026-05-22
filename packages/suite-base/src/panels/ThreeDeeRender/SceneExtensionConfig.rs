```rust
use std::collections::HashMap;

struct Cameras;
struct FoxgloveGrid;
struct FrameAxes;
struct Grids;
struct ImageMode;
struct Images;
struct LaserScans;
struct Markers;
struct OccupancyGrids;
struct PointClouds;
struct Polygons;
struct PoseArrays;
struct Poses;
struct PublishSettings;
struct FoxgloveSceneEntities;
struct SceneSettings;
struct Urdfs;
struct VelodyneScans;

trait IRenderer {
    fn interface_mode(&self) -> InterfaceMode;
}

enum InterfaceMode {
    ThreeD,
    Image,
}

type ExtensionOverride<ExtensionType> = Box<dyn FnOnce(IRenderer) -> ExtensionType>;

#[derive(Default)]
struct SceneExtensionConfig {
    reserved: ReservedSceneExtensionConfig,
    extensions_by_id: HashMap<String, ExtensionOverride<SceneExtension>>,
}

#[derive(Default)]
struct ReservedSceneExtensionConfig {
    image_mode: ExtensionOverride<ImageMode>,
    measurement_tool: ExtensionOverride<MeasurementTool>,
    publish_click_tool: ExtensionOverride<PublishClickTool>,
}

type SceneExtension = dyn Any + Send + Sync;

#[derive(Default)]
pub struct PublishSettings;
impl SceneExtension for PublishSettings {}

#[derive(Default)]
struct Images;
impl SceneExtension for Images {}

#[derive(Default)]
struct Cameras;
impl SceneExtension for Cameras {}

#[derive(Default)]
struct SceneSettings;
impl SceneExtension for SceneSettings {}

#[derive(Default)]
struct FrameAxes;
impl SceneExtension for FrameAxes {
    fn new(renderer: &IRenderer) -> Self {
        Self {
            visible: renderer.interface_mode() == InterfaceMode::ThreeD,
        }
    }
}

#[derive(Default)]
struct Grids;
impl SceneExtension for Grids {}

#[derive(Default)]
struct Markers;
impl SceneExtension for Markers {}

#[derive(Default)]
struct FoxgloveSceneEntities;
impl SceneExtension for FoxgloveSceneEntities {}

#[derive(Default)]
struct FoxgloveGrid;
impl SceneExtension for FoxgloveGrid {}

#[derive(Default)]
struct LaserScans;
impl SceneExtension for LaserScans {}

#[derive(Default)]
struct OccupancyGrids;
impl SceneExtension for OccupancyGrids {}

#[derive(Default)]
struct PointClouds;
impl SceneExtension for PointClouds {}

#[derive(Default)]
struct Polygons;
impl SceneExtension for Polygons {}

#[derive(Default)]
struct Poses;
impl SceneExtension for Poses {}

#[derive(Default)]
struct PoseArrays;
impl SceneExtension for PoseArrays {}

#[derive(Default)]
struct Urdfs;
impl SceneExtension for Urdfs {}

#[derive(Default)]
struct VelodyneScans;
impl SceneExtension for VelodyneScans {}
```