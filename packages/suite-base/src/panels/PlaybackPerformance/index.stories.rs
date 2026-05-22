```rust
use storybook::StoryObj;

#[story]
pub fn simple_example() -> impl FnOnce() -> Box<dyn FnOnce()> + 'static {
    async move {
        let panel_setup = PanelSetup::new();
        let playback_performance = PlaybackPerformance::new();

        Box::pin(move || async {
            panel_setup.render().await;
            playback_performance.render().await;
        })
    }
}
```