```rust
use crate::components::{ExtensionDetails as ExtensionDetailsComponent, TextContent};
use crate::domain::{
  ExtensionDetail,
  InstallationStatus,
  OperationStatus,
  ReadmeMarkdownRequest,
  ChangelogMarkdownRequest,
};

// Implement the ExtensionDetails component in Rust
impl ExtensionDetails {
  fn new(
    extension: ExtensionDetail,
    on_close: impl Fn(),
    installed: bool,
  ) -> Self {
    Self {
      extension,
      on_close,
      is_installed: installed,
      active_tab: 0,
      is_mounted: false,
    }
  }

  // Implement the render method in Rust
  fn render(&self) -> String {
    let classes = ExtensionDetailsComponent::get_classes();
    let readme_content = self.get_readme_content().await;
    let changelog_content = self.get_changelog_content().await;

    format!(
      r#"
        <Stack fullHeight flex="auto" gap={1}>
          <div>
            <Button
              className={classes.backButton}
              onClick={self.on_close}
              size="small"
              startIcon={<ChevronLeftIcon />}
            >
              Back
            </Button>
            <Typography variant="h3" fontWeight={500}>
              {self.extension.id}
            </Typography>
          </div>

          <Stack gap={1} alignItems="flex-start">
            <Stack gap={0.5} paddingBottom={1}>
              <Stack direction="row" gap={1} alignItems="baseline">
                <Link
                  variant="body2"
                  color="primary"
                  href={self.extension.homepage}
                  target="_blank"
                  underline="hover"
                >
                  {self.extension.id}
                </Link>
                <Typography
                  variant="caption"
                  color="text.secondary"
                >{`v${self.extension.version}`}</Typography>
                <Typography variant="caption" color="text.secondary">
                  {self.extension.license}
                </Typography>
                {self.extension.size != None && (
                  <Typography variant="caption" color="text.secondary">
                    {format_byte_size(self.extension.size.unwrap())}
                  </Typography>
                )}
              </Stack>
              <Typography variant="subtitle2" gutterBottom>
                {self.extension.publisher}
              </Typography>
              <Typography variant="body2" gutterBottom>
                {self.extension.description}
              </Typography>
            </Stack>
            {if self.is_installed {
              <UninstallButton
                extension=self.extension
                on_action={self.handle_uninstall}
                is_operating=self.operation_status != OperationStatus::IDLE
                operation_status=self.operation_status
                stop_propagation=true
                label=ExtensionActionsLabel::UNINSTALL
                loading_label=ExtensionOperationStatusLabel::UNINSTALLING
              />
            } else {
              <InstallButton
                extension=self.extension
                on_action={self.handle_install}
                is_operating=self.operation_status != OperationStatus::IDLE
                operation_status=self.operation_status
                stop_propagation=true
                label=ExtensionActionsLabel::INSTALL
                loading_label=ExtensionOperationStatusLabel::INSTALLING
              />
            }}
          </Stack>

          <Stack paddingTop={2} style={{ marginLeft: -16, marginRight: -16 }}>
            <Tabs
              textColor="inherit"
              value=self.active_tab
              onChange={|event, new_value| {
                self.active_tab = new_value;
              }}
            >
              <Tab disableRipple label="README" value=0 />
              <Tab disableRipple label="CHANGELOG" value=1 />
            </Tabs>
            <Divider />
          </Stack>

          <Stack flex="auto" paddingY={2}>
            {if self.active_tab == 0 {
              <TextContent>{readme_content}</TextContent>
            } else if self.active_tab == 1 {
              <TextContent>{changelog_content}</TextContent>
            }
          </Stack>
        </Stack>
      "#,
    )
  }

  // Implement the handle_install and handle_uninstall methods in Rust
  async fn handle_install(&self) {
    // Handle installation logic here
  }

  async fn handle_uninstall(&self) {
    // Handle uninstallation logic here
  }

  // Implement the get_readme_content and get_changelog_content methods in Rust
  async fn get_readme_content(&self) -> String {
    let request = ReadmeMarkdownRequest::new(self.extension.readme);
    self.marketplace.get_markdown(request).await.unwrap()
  }

  async fn get_changelog_content(&self) -> String {
    let request = ChangelogMarkdownRequest::new(self.extension.changelog);
    self.marketplace.get_markdown(request).await.unwrap()
  }
}
```