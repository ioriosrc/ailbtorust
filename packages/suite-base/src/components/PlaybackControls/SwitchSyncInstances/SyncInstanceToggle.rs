```rust
use crate::components::{Button, Stack, Typography};
use crate::context::workspace::{PlaybackControlsContext, WorkspaceActionsContext};

pub struct SyncInstanceToggle;

impl SyncInstanceToggle {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn render(
        &self,
        ctx: WorkspaceControlsContext,
        actions_ctx: WorkspaceActionsContext,
    ) -> anyhow::Result<jsx! {
        <Button
            className={classes.button}
            onClick={async move {
                let sync_instances = ctx.playback_control_actions().set_sync_instances(!ctx.playback_control_actions().sync_instances());
            }}
        >
            <Stack className={classes.textWrapper}>
                <Typography className={classes.syncText}>Sync</Typography>
                <Typography className={classes.onOffText}>{ctx.playback_control_actions().sync_instances()}; syncInstances: {sync_instances}</Typography>
            </Stack>
        </Button>
    }}
}

fn main() -> anyhow::Result<()> {
    Ok(())
}
```