```rust
use serde_json::Value;
use std::sync::Arc;

use crate::components::{AccountInfo, SidebarContent};
use crate::context::CurrentUserContext;
use crate::ui::signin_form::SigninForm;

pub fn account_settings() -> JSXElement {
    let user = use_current_user();

    let content: JSXElement = useMemo(
        move || {
            if user.is_none() {
                SigninForm {}
            } else {
                AccountInfo { user: Arc::clone(&user) }
            }
        },
        [user],
    );

    <SidebarContent title="Account">
        {content}
    </SidebarContent>
}
```