```rust
use std::fmt;
use crate::gui::{
    components::{AccountSyncGraphic, Typography},
    styling::styles as mui_styling,
};

pub struct SigninForm {}

impl SigninForm {
    pub fn view(&self) -> String {
        format!(
            r#"
          <div class="flex flex-col gap-2.5">
            <div class="flex items-center justify-center color-primary">
              <AccountSyncGraphic width={192} />
            </div>
            <Typography variant="body1">
              <>
                Create a Foxglove account to:
                <ul>
                  <li>Sync your layouts across multiple devices</li>
                  <li>Share your layouts with others</li>
                  <li>Manage and store your robotics data</li>
                </ul>
              </>
            </Typography>

            <Button variant="contained" color="primary">
              Sign in
            </Button>
          </div>
        "#,
        )
    }
}

impl fmt::Display for SigninForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        self.view().fmt(f)
    }
}
```