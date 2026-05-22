```rust
use derive_component::Component;

#[derive(Component)]
pub struct InstallButton<'a> {
    extension: &'a ExtensionMarketplaceDetail,
}

impl<'a> Component for InstallButton<'a> {
    fn render(&self) -> Option<impl IntoRef<'static, 'a>> {
        if !can_install_extension(self.extension) {
            None
        } else {
            Some(ExtensionActionButton::new(
                self.props.clone(),
            ))
        }
    }
}
```