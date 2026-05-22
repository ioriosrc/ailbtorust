```rust
use mui::components::MessagePipeline::MockMessagePipelineProvider;
use mui::material::{TimelineInteractionStateContextProvider};
use playback_controls_tooltip_content::{PlaybackControlsTooltipContent};
use playback_interaction_state_context::use_timeline_interaction_state;
use react::html::{div, use_effect};

fn Wrapper(Wrapped: fn() -> JSX.Element) -> JSX.Element {
    let theme = mui::theme();

    div(
        style!({
            maxWidth: "16rem",
            marginInline: "auto",
            backgroundColor: theme.palette.background.paper,
        }),
        TimelineInteractionStateContextProvider,
        MockMessagePipelineProvider,
        Wrapped(),
    )
}

export default {
    component: PlaybackControlsTooltipContent,
    title: "components/PlaybackControls/TooltipContent",
    decorators: [Wrapper],
};

export const Default: JSX.Element = () => {
    return <PlaybackControlsTooltipContent stamp={{ sec: 1, nsec: 1 }} />;
};

export const WithEvents: JSX.Element = () => {
    let set_events = use_timeline_interaction_state((store) => store.set_events_at_hover_value);

    use_effect(() => {
        set_events([
            {
                event: {
                    createdAt: "1",
                    id: "1",
                    deviceId: "dev1",
                    durationNanos: "1",
                    endTime: { sec: 1, nsec: 1 },
                    endTimeInSeconds: 1,
                    metadata: {
                        "meta 1": "value 1",
                        "meta 2": "value 2",
                        "long event metadata key that might overflow":
                            "long event metadata value that might also overflow",
                    },
                    startTime: { sec: 0, nsec: 0 },
                    startTimeInSeconds: 1,
                    timestampNanos: "1",
                    updatedAt: "1",
                },
                startPosition: 0,
                endPosition: 0.1,
                secondsSinceStart: 0,
            },
        ]);
    }, [set_events]);

    return <PlaybackControlsTooltipContent stamp={{ sec: 1, nsec: 1 }} />;
};
```