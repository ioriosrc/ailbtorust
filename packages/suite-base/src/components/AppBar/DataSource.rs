```rust
use crate::components::WssErrorModal;
use crate::components::TextMiddleTruncate;
use crate::players::PlayerPresence;
use crate::state::MessagePipelineContext;
use crate::utils::{icon_size, cx};
use lichtblick_base::{message_pipeline::select_player_name, select_player_presence, select_player_alerts, select_seek};
use lichtblick_workspace::WorkspaceActions;

const ICON_SIZE: usize = 18;

pub fn DataSource() -> JSX.Element {
    let { t } = use_translation("appBar");

    let playerName = use_message_pipeline(select_player_name);
    let playerPresence = use_message_pipeline(select_player_presence);
    let playerAlerts = use_message_pipeline(select_player_alerts) ?? [];
    let seek = use_message_pipeline(select_seek);

    let sidebar_actions = use_workspace_actions();

    // A crude but correct proxy (for our current architecture) for whether a connection is live
    let is_live_connection = seek.is_none();

    let reconnecting = player_presence == PlayerPresence::RECONNECTING;
    let initializing = player_presence == PlayerPresence::INITIALIZING;
    let error =
        player_presence == PlayerPresence::ERROR ||
        player_alerts.iter().any(|alert| alert.severity == "error");
    let loading = reconnecting || initializing;

    let player_display_name = if initializing && playerName.is_none() {
        "Initializing..."
    } else {
        &player_name
    };

    if player_presence == PlayerPresence::NOT_PRESENT {
        return <div className={cx("sourceName")}>{t("noDataSource")}</div>;
    }

    return (
        <>
            <WssErrorModal player_alerts={player_alerts} />
            <Stack direction="row" alignItems="center">
                <div className={cx("sourceName", "textTruncate")}>
                    <TextMiddleTruncate text={player_display_name.unwrap_or("<unknown>")} />
                </div>
                <div className={cx("adornment", { "adornmentError": error })}>
                    {loading && (
                        <CircularProgress
                            size={ICON_SIZE}
                            color="inherit"
                            className={cx("spinner")}
                            variant="indeterminate"
                        />
                    )}
                    {error && (
                        <IconButton
                            color="inherit"
                            className={cx("iconButton")}
                            onClick={() => {
                                sidebar_actions.left.set_open(true);
                                sidebar_actions.left.select_item("alerts");
                            }}
                        >
                            <ErrorCircle16Filled />
                        </IconButton>
                    )}
                </div>
            </Stack>
        </>
    );
}
```