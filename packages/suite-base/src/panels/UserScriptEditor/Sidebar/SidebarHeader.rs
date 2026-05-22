```rust
use fluentui::react::{Icon, IconButton};
use mui::material::{CardHeader, CardHeaderProps};

pub fn sidebar_header(props: SidebarHeaderProps) -> ReactElement {
  let title = props.title;
  let subheader = props.subheader;
  let on_close = props.onClose;

  <CardHeader
    title={title}
    slotProps={{
      title: {
        variant: "h6",
      },
      subheader: {
        variant: "body2",
        color: "text.secondary",
      },
    }}
    subheader={subheader}
    action={
      <IconButton size="small" onClick={on_close} title="Collapse">
        <Icon icon="dismiss_20_filled" />
      </IconButton>
    }
  />
}
```