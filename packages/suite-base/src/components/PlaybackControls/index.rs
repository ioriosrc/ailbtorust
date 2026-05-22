```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use leptos::*;
use wasm_bindgen::JsCast;

#[allow(unused_imports)]
use {
    @fluentui/react-icons as fluent,
    @mui/material as mui,
    @lichtblick/rostime as rostime,
    @lichtblick/suite-base/components/CreateEventDialog as CreateEventDialog,
    @lichtblick/suite-base/components/DataSourceInfoView as DataSourceInfoView,
    @lichtblick/suite-base/components/EventIcon as EventIcon,
    @lichtblick/suite-base/components/EventOutlinedIcon as EventOutlinedIcon,
    @lichtblick/suite-base/components/HoverableIconButton as HoverableIconButton,
    @lichtblick/suite-base/components/KeyListener as KeyListener,
    @lichtblick/suite-base/components/PlaybackControls/SwitchSyncInstances/SyncInstanceToggle as SyncInstanceToggle,
    @lichtblick/suite-base/components/PlaybackControls/index.style as styles,
    @lichtblick/suite-base/components/PlaybackControls/Scrubber as Scrubber,
    @lichtblick/suite-base/players/types as player_types,
    @lichtblick/suite-base/util/broadcast/BroadcastManager as BroadcastManager,

    playback_control_actions::PlaybackControlActions,
    workspace_context_store::WorkspaceContextStore,
    workspace_actions::use_workspace_actions,
};

type PlaybackControlsProps = {
    play: FunctionRef<()>,
    pause: FunctionRef<()>,
    seek: FunctionRef<()>,
    play_until?: Date | null,
    is_playing: boolean,
    get_time_info: () => {
        startTime: rostime::Time;
        endTime: rostime::Time;
        current_time: rostime::Time;
    };
};

export default function PlaybackControls(props: PlaybackControlsProps): Component<PlaybackControlsProps> {
    let presence = use_message_pipeline(select_presence);

    const { classes, cx } = useStyles();

    let repeat = use_workspace_context(select_playback_repeat);
    let [create_event_dialog_open, set_create_event_dialog_open] = create_signal(false);
    let { current_user_type } = use_current_user();
    let events_supported = use_events(select_events_supported);

    let {
        playback_control_actions: { set_repeat },
    } = use_workspace_actions();

    const toggle_repeat = useCallback(() => {
        set_repeat((old) => !old);
    }, [set_repeat]);

    const toggle_play_pause = useCallback(() => {
        let { startTime, endTime, current_time } = props.get_time_info();

        if (props.is_playing) {
            props.pause();
            BroadcastManager::new().post({
                type: "pause",
                time: current_time,
            });
        } else {
            // if we are at the end, we need to go back to start
            if (current_time && endTime && startTime && rostime::compare(current_time, endTime) >= 0) {
                props.seek();
            }
            props.play();

            BroadcastManager::new().post({
                type: "play",
                time: current_time,
            });
        }
    }, [props.is_playing, props.pause, props.get_time_info, props.seek]);

    let { seek_forward_action, seek_backward_action } = use_directional_seek(props);

    const key_down_handlers = useMemo(() => ({
        " ": toggle_play_pause,
        ArrowLeft: (ev: KeyboardEvent) => {
            seek_backward_action(ev);
        },
        ArrowRight: (ev: KeyboardEvent) => {
            seek_forward_action(ev);
        },
    }), [seek_backward_action, seek_forward_action, toggle_play_pause]);

    let toggle_create_event_dialog = useCallback(() => {
        props.pause();
        set_create_event_dialog_open((open) => !open);
    }, [props.pause]);

    let disable_controls = presence === player_types::PlayerPresence::ERROR;

    return (
        <>
            <RepeatAdapter play={props.play} seek={props.seek} repeat_enabled={repeat} />
            <KeyListener global key_down_handlers={key_down_handlers} />
            <div class={classes.root}>
                <div class={classes.scrubber_wrapper}>
                    <Scrubber on_seek={props.seek} />
                </div>
                <Stack direction="row" alignItems="center" flex={1} gap={1}>
                    <Stack direction="row" alignItems="center" flex={1} gap={0.5}>
                        {current_user_type != "unauthenticated" && events_supported && (
                            <HoverableIconButton
                                size="small"
                                title="Create event"
                                icon={<EventOutlinedIcon />}
                                active_icon={<EventIcon />}
                                onclick={toggle_create_event_dialog}
                            />
                        )}
                        <Tooltip
                            disable_focus_listener
                            classes={{ popper: classes.popper }}
                            title={
                                <Stack padding_y={0.75}>
                                    <DataSourceInfoView disable_source />
                                </Stack>
                            }
                        >
                            <HoverableIconButton
                                class={cx(classes.dataSource_info_button, {
                                    [classes.disabled]: disable_controls,
                                })}
                                size="small"
                                icon={<Info20Regular />}
                            />
                        </Tooltip>
                        <PlaybackTimeDisplay on_seek={props.seek} onPause={props.pause} />
                    </Stack>
                    <Stack direction="row" alignItems="center" gap={1}>
                        <HoverableIconButton
                            disabled={disable_controls}
                            size="small"
                            title={props.is_playing ? "Pause" : "Play"}
                            onclick={toggle_play_pause}
                            icon={props.is_playing ? <Pause20Regular /> : <Play20Regular />}
                            active_icon={props.is_playing ? <Pause20Filled /> : <Play20Filled />}
                            data-testid="play-button"
                        />
                        <HoverableIconButton
                            disabled={disable_controls}
                            size="small"
                            title="Seek forward"
                            icon={<Next20Regular />}
                            active_icon={<Next20Filled />}
                            onclick={() => {
                                seek_forward_action();
                            }}
                            data-testid="seek-forward-button"
                        />
                    </Stack>
                    <Stack direction="row" flex={1} alignItems="center" justifyContent="flex-end" gap={0.5}>
                        <SyncInstanceToggle />
                        <HoverableIconButton
                            size="small"
                            title="Loop playback"
                            color={repeat ? "primary" : "inherit"}
                            onclick={toggle_repeat}
                            icon={repeat ? <ArrowRepeatAll20Regular /> : <ArrowRepeatAllOff20Regular />}
                        />
                        <PlaybackSpeedControls />
                    </Stack>
                </Stack>
                {create_event_dialog_open && events_supported && (
                    <CreateEventDialog onClose={toggle_create_event_dialog} />
                )}
            </div>
        </>
    );
}
```