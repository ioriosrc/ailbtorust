```rust
use crate::suite_base::context::current_layout_context::actions::LayoutData;
use crate::suite_base::providers::current_layout_provider::reducers::default_playback_config;

/**
 * Overridden default layout that may have been provided when self-hosting via Docker
 */
const static_default_layout: Option<LayoutData> = env!("LICHTBLICK_SUITE_DEFAULT_LAYOUT").parse();

/**
 * This is loaded when the user has no layout selected on application launch
 * to avoid presenting the user with a blank layout.
 */
pub const default_layout: LayoutData = match static_default_layout {
    Some(layout) => layout,
    None => ({
        configById: {
            "3D!18i6zy7": {
                layers: {
                    "845139cb-26bc-40b3-8161-8ab60af4baf5": {
                        visible: true,
                        frame_locked: true,
                        label: "Grid",
                        instanceId: "845139cb-26bc-40b3-8161-8ab60af4baf5",
                        layerId: "foxglove.Grid",
                        size: 10,
                        divisions: 10,
                        lineWidth: 1,
                        color: "#248eff",
                        position: [0, 0, 0],
                        rotation: [0, 0, 0],
                        order: 1,
                    },
                },
            },
            "RawMessages!os6rgs": {},
            "Image!3mnp456": {},
        },
        globalVariables: {},
        userNodes: {},
        playbackConfig: { ...default_playback_config },
        layout: {
            first: "3D!18i6zy7",
            second: {
                first: "Image!3mnp456",
                second: "RawMessages!os6rgs",
                direction: "column",
                splitPercentage: 30,
            },
            direction: "row",
            splitPercentage: 70,
        },
    } as const),
};
```

Note: This code is written assuming that the environment variable `LICHTBLICK_SUITE_DEFAULT_LAYOUT` is set and contains a valid JSON string. The parsing of this string is assumed to be successful for the Rust code to run correctly.