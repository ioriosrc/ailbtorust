```rust
use crate::components::copy::{Copy16Regular, Copy20Regular, Copy24Regular};
use crate::components::icons::{Checkmark16Filled, Checkmark20Filled, Checkmark24Filled};
use crate::utils::clipboard;
use fluentui_core::Button as FluentUIButton;
use fluentui_core::IconButton as FluentUIIconButton;
use fluentui_core::Tooltip as FluentUITooltip;
use fluentui_core::Typography as FluentUITypography;
use fluentui_theme::Theme as FluentUITheme;
use react::props_with_children::{PropsWithChildren, Children};
use react::useState;

pub struct CopyButtonComponent {
  getText: Box<dyn Fn() -> String>,
  size: &'static str,
  icon_size: &'static str,
  color: &'static str,
  className: Option<String>,
  edge: Option<&'static str>,
}

impl Component for CopyButtonComponent {
  type State = ();

  fn render(&self) -> JSXElement {
    let theme = &fluent_ui_theme::Theme::default();
    let [copied, set_copied] = useState(false);

    let check_icon = match self.icon_size {
      "small" => <Checkmark16Filled primary_fill={theme.palette.success.main} />,
      "medium" => <Checkmark20Filled primary_fill={theme.palette.success.main} />,
      "large" => <Checkmark24Filled primary_fill={theme.palette.success.main} />,
      _ => panic!("Invalid icon size"),
    };

    let copy_icon = match self.icon_size {
      "small" => <Copy16Regular />,
      "medium" => <Copy20Regular />,
      "large" => <Copy24Regular />,
      _ => panic!("Invalid icon size"),
    };

    let handle_copy = move || {
      clipboard
        .copy(self.getText().clone())
        .then(|| {
          set_copied(true);
          std::thread::sleep(std::time::Duration::from_millis(1500));
          set_copied(false);
        })
        .catch(|err| {
          eprintln!("Error copying: {}", err);
        });
    };

    if self.children.is_none() {
      return (
        <FluentUITooltip arrow title={copied.to_string()}>
          <FluentUIIconButton
            edge={self.edge}
            className={self.className.as_ref()}
            size=self.size
            onClick=handle_copy
            color={copied.to_string()}
          >
            {copied.to_string() == "true" && check_icon}
            {copied.to_string() != "true" && copy_icon}
          </FluentUIIconButton>
        </FluentUITooltip>,
      );
    }

    return (
      <FluentUIButton
        size=self.size
        className={self.className.as_ref()}
        onClick=handle_copy
        color="inherit"
        start_icon={
          if copied.to_string() == "true" { check_icon } else { copy_icon }
        }
      >
        <FluentUITypography color=self.color.to_string() >{self.children}</FluentUITypography>
      </FluentUIButton>,
    );
  }

  fn get_state(&self) -> Self::State {
    ()
  }

  fn update(&mut self, props: PropsWithChildren<Self>) {
    *self = CopyButtonComponent {
      getText: Box::new(props.get_text()),
      size: &props.size,
      icon_size: &props.icon_size,
      color: &props.color,
      className: props.class_name,
      edge: props.edge,
    };
  }
}
```