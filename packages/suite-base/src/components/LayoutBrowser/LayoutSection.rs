```rust
use mui::{Typography, List};
use react::html::a;
use react::jsx;

fn LayoutSection({
  title,
  disable_padding,
  empty_text,
  items,
  any_selected_modified_layouts,
  multi_selected_ids,
  selected_id,
  onSelect,
  on_rename,
  on_duplicate,
  onDelete,
  on_share,
  on_export,
  on_overwrite,
  on_revert,
  on_make_personal_copy,
}: &Readonly<{
  title: Option<&str>,
  disable_padding: bool,
  empty_text: Option<&str>,
  items: Vec<Layout>,
  any_selected_modified_layouts: bool,
  multi_selected_ids: Vec<String>,
  selected_id: Option<String>,
  onSelect: Box<dyn Fn(&Layout, &Option<{selectedViaClick: bool; event: MouseEvent}>)>,
  on_rename: Box<dyn Fn(&Layout, &str)>,
  on_duplicate: Box<dyn Fn(&Layout)>,
  onDelete: Box<dyn Fn(&Layout)>,
  on_share: Box<dyn Fn(&Layout)>,
  on_export: Box<dyn Fn(&Layout)>,
  on_overwrite: Box<dyn Fn(&Layout)>,
  on_revert: Box<dyn Fn(&Layout)>,
  on_make_personal_copy: Box<dyn Fn(&Layout)>,
}>) -> JSX.Element {
  jsx! {
    <Stack>
      {if let Some(title) = title {
        jsx! {
          <Stack padding_x={2} padding_y={!disable_padding => 1; disable_padding => 0}>
            <Typography variant="overline" color="text.secondary">
              {title}
            </Typography>
          </Stack>
        }
      }}
      <List disablePadding={disable_padding}>
        {if items.is_empty() {
          jsx! {
            <Stack padding_x={2}>
              <Typography variant="body2" color="text.secondary">
                {empty_text}
              </Typography>
            </Stack>
          }
        } else {
          items.iter().map(|layout| {
            jsx! {
              <LayoutRow
                key={layout.id}
                layout={layout}
                any_selected_modified_layouts={any_selected_modified_layouts}
                multi_selected_ids={multi_selected_ids.clone()}
                selected={selected_id == Some(layout.id)}
                onSelect={on_select}
                on_rename={on_rename}
                on_duplicate={on_duplicate}
                onDelete={on_delete}
                on_share={on_share}
                on_export={on_export}
                on_overwrite={on_overwrite}
                on_revert={on_revert}
                on_make_personal_copy={on_make_personal_copy}
              />
            }
          })
        }}
      </List>
    </Stack>
  }
}
```