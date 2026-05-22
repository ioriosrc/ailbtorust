```rust
pub fn three_dee() -> Vec<&'static str> {
    vec![
        "color", "colorMode", "frame", "lineWidth", "position", "reset", "rotation", "scale", "gradient", "type", "topic",

        // Frame
        "age", "axisScale", "displayFrame", "displayFrameHelp",
        "editable", "enablePreloading", "enablePreloadingTooltip",
        "maxPreloadMessages", "maxPreloadMessagesTooltip",
        "clearPreloadBuffer", "fixed", "followMode", "followModeHelp",
        "frameNotFound", "hideAll", "historySize", "labels", "labelSize", "lineColor", "noCoordinateFramesFound",
        "parent", "pose", "rotationOffset", "settings", "showAll", "transforms", "translation", "translationOffset",

        // Scene
        "background", "debugPicking", "ignoreColladaUpAxis", "ignoreColladaUpAxisHelp",
        "labelScale", "labelScaleHelp", "meshUpAxis", "meshUpAxisHelp",
        "renderStats", "scene", "takeEffectAfterReboot", "YUp", "ZUp",

        // Camera
        "distance", "far", "fovy", "near", "perspective", "phi", "planarProjectionFactor", "syncCamera", "syncCameraHelp",
        "target", "theta", "view",

        // Topics
        "topics",

        // Custom layers
        "addGrid", "addURDF", "customLayers", "delete", "divisions", "grid", "size",

        // Image annotations
        "imageAnnotations", "resetView",

        // Images
        "cameraInfo",

        // Occupancy Grids
        "colorModeCustom", "colorModeRaw", "colorModeRvizCostmap", "colorModeRvizMap", "frameLock",
        "invalidColor", "maxColor", "minColor", "unknownColor",

        // Point Extension Utils
        "decayTime", "decayTimeDefaultZeroSeconds", "pointShape", "pointShapeCircle", "pointShapeSquare",
        "pointSize", "colorBy", "colorModeBgraPacked", "colorModeBgrPacked", "colorModeColorMap",
        "colorModeFlat", "colorModeRgbaSeparateFields", "ColorFieldComputedDistance", "flatColor",
        "opacity", "valueMax", "valueMin",

        // Markers
        "selectionVariable", "selectionVariableHelp", "showOutline",

        // Poses
        "covariance", "covarianceColor", "poseDisplayTypeArrow", "poseDisplayTypeAxis", "poseDisplayTypeLine",

        // Publish
        "publish", "publishTopicHelp", "publishTypeHelp", "publishTypePoint", "publishTypePose",
        "publishTypePoseEstimate", "thetaDeviation", "thetaDeviationHelp", "xDeviation", "xDeviationHelp",
        "yDeviation", "yDeviationHelp",

        // HUD Items and empty states
        "noImageTopicsAvailable", "imageTopicDNE", "calibrationTopicDNE", "imageAndCalibrationDNE",
        "waitingForCalibrationAndImages", "waitingForCalibration", "waitingForImages", "waitingForSyncAnnotations",
    ]
}
```