```rust
use crate::ThreeDeePanel;
use foxglove::{FromSec, FrameTransforms, MessageEvent};
use foxglove_schemas::{
    visualization_msgs::Marker,
    suite_base::players::types::Topic,
    suite_base::stories::PanelSetup,
};
use storybook::{create_test_fixture, with_playwright};

fn main() {
    let topics = vec![
        Topic::new("markers", "visualization_msgs/Marker"),
        Topic::new("/tf", "foxglove.FrameTransforms"),
    ];

    let tf_t1: MessageEvent<FrameTransforms> = MessageEvent::new(
        "tf",
        FromSec(1),
        "foxglove.FrameTransforms",
        vec![
            {
                timestamp: FromSec(1),
                parent_frame_id: "base_link".to_string(),
                child_frame_id: "sensor_1".to_string(),
                translation: Vec3::new(1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
            {
                timestamp: FromSec(2),
                parent_frame_id: "base_link".to_string(),
                child_frame_id: "sensor_2".to_string(),
                translation: Vec3::new(2.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
            {
                timestamp: FromSec(3),
                parent_frame_id: "base_link".to_string(),
                child_frame_id: "sensor_3".to_string(),
                translation: Vec3::new(3.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        ],
    );

    let pass1 = make_pass(1, "sensor_1", FromSec(1), TEST_COLORS::MARKER_GREEN1);
    let pass2 = make_pass(2, "sensor_2", FromSec(2), TEST_COLORS::MARKER_GREEN2);
    let pass3 = make_pass(3, "sensor_3", FromSec(3), TEST_COLORS::MARKER_GREEN3);

    let fixture = create_test_fixture(
        topics,
        vec![&tf_t1],
        &vec![],
        &vec![],
        Some(vec![FromSec(2)]),
    );

    with_playwright(|playwright| {
        playwright.test("ThreeDeePanel", async () => {
            await playwright.request.set_base_url("http://localhost:3000");

            let page = await playwright.chromium.new_page();
            await page.goto("/panels/ThreeDeeRender");
            await page.click("#settings__nodeHeaderToggle__general");
            await page.click("#settings__nodeHeaderToggle__transforms");
            await page.click("#settings__nodeHeaderToggle__transforms-settings");
            await page.click("#settings__nodeHeaderToggle__transforms-frame:base_link");
            await page.click("#settings__nodeHeaderToggle__transforms-frame:sensor_1");
        });
    });
}
```