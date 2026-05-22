```rust
use wasm_bindgen::prelude::*;
use storybook::testing::{expect, within};

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    fn user_event_setup() -> js_sys::JsValue;
    #[no_mangle]
    fn keyboard(event: *const js_sys::Object) -> js_sys::JsValue;
    #[no_mangle]
    fn action(event: *const js_sys::Object) -> js_sys::JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    fn type(element: *const js_sys::Object, value: &str);
}

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    async fn step<T>(callback: T) -> T;
}

#[wasm_bindgen]
pub struct PublishConfig {
    pub topic_name: String,
    pub datatype: String,
    pub advanced_view: bool,
    pub value: String,
}

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    async fn get_fixture(allow_publish: bool) -> js_sys::JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    async fn empty_fixture() -> js_sys::JsValue;
}

#[wasm_bindgen]
pub struct Fixture {
    pub topics: Vec<FixtureTopic>,
    pub datatypes: Map<String, DatatypeDefinition>,
    pub frame: Frame,
    pub capabilities: Vec<PlayerCapability>,
    pub publish: Option<(u32, String)>,
    pub set_publishers: Option<(u32, String)>,
}

#[wasm_bindgen]
pub struct FixtureTopic {
    pub name: String,
    pub schema_name: String,
}

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    async fn publish(config: &PublishConfig, publish: Option<(&u32, &str)>) -> js_sys::JsValue;
    #[no_mangle]
    async fn set_publishers(config: &PublishConfig, publishers: Option<(&u32, &str)>) -> js_sys::JsValue;
}

#[wasm_bindgen]
pub struct PlayerCapability {
    pub name: String,
}

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    async fn within(element: *const js_sys::Object) -> js_sys::JsValue;
}

#[wasm_bindgen]
async fn step<T>(callback: T) -> T {
    user_event_setup().await
}

#[wasm_bindgen]
pub struct Frame {}

#[wasm_bindgen]
pub struct DatatypeDefinition {
    pub definitions: Vec<DatatypeDefinitionDefinition>,
}

#[wasm_bindgen]
pub struct DatatypeDefinitionDefinition {
    pub name: String,
    pub type: String,
}

#[derive(Serialize, Deserialize)]
struct TestData {
    value: String,
}

#[wasm_bindgen]
#[repr(u32)]
enum PlayerCapabilities {
    Advertise = 0,
    Subscribe = 1,
}

type StoryArgs = PublishConfig;

#[wasm_bindgen]
pub struct Meta {}

#[wasm_bindgen]
pub struct StoryObj<T> {
    args: T,
}

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    fn get_story_meta() -> js_sys::JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    async fn get_story_params(story_name: &str) -> js_sys::JsValue;
}

#[wasm_bindgen]
pub struct PanelSetup {
    include_settings: bool,
    fixture: Fixture,
}

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    async fn default_panel_setup(include_settings: bool, fixture: &Fixture) -> js_sys::JsValue;
}

#[wasm_bindgen]
async fn step<T>(callback: T) -> T {
    user_event_setup().await
}

#[wasm_bindgen]
pub struct Publish {}

#[wasm_bindgen]
pub struct Default {}

#[wasm_bindgen]
pub struct PublishEnabled {}

#[wasm_bindgen]
pub struct WhenSelectingATopicSchemaIsSuggested {}

#[wasm_bindgen]
pub struct PublishEnabledWithTopicAndSchema {}

#[wasm_bindgen]
pub struct PublishEnabledWithCustomButtonSettings {}

#[wasm_bindgen]
pub struct PublishDisabledWithTopicAndSchema {}

#[wasm_bindgen]
pub struct WithValidJSON {}

#[wasm_bindgen]
pub struct WithInvalidJSON {}

#[wasm_bindgen]
pub struct WithSchemaThatNoLongerExists {}

#[wasm_bindgen]
pub struct DefaultEditingModeOff {}

#[wasm_bindgen]
pub struct PublishEnabledEditingOff {}

#[wasm_bindgen]
pub struct PublishEnabledWithTopicAndSchemaEditingOff {}

#[wasm_bindgen]
pub struct PublishEnabledWithCustomButtonSettingsEditingOff {}

#[wasm_bindgen]
pub struct PublishDisabledEditingModeOff {}
```