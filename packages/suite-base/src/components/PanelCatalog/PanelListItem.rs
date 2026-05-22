```rust
use react::prelude::*;
use react_dnd::{DragHandle, DragSource, DropTarget};
use react_mosaic_component::{MosaicDragType, MosaicPath, MosaicDropResult, PanelConfig};
use tss_react::make_styles;

use crate::suite_base::components::Stack;
use crate::suite_base::context::PanelCatalogContext;
use crate::suite_base::types::panels::PanelInfo;
use crate::suite_base::types::panels::MosaicDropTargetPosition;

#[style_function]
fn useStyles() -> impl Fn(&Theme, &<_ as ComponentProps<'_>>::State) -> &'static str {
  makeStyles((theme, _) => {
    return {
      listItemButton: {
        height: theme.spacing(4), // hard coded here because the parent element of this changes based on context
        cursor: "grab",

        [`&:not(:hover) .${classes.dragIcon}`]: {
          visibility: "hidden",
        },
      },
      dragIcon: {
        cursor: "grab",
        marginRight: theme.spacing(-1),
        color: theme.palette.text.disabled,
      },
    };
  })
}

#[function_component]
pub fn PanelListItem(props: &Props) -> HtmlElement {
  let (
    search_query,
    panel,
    onClick,
    on_drag_start,
    on_drop,
    checked = false,
    highlighted = false,
    mosaic_id,
  ) = props;
  let (classes, _) = useStyles();

  let scroll_ref = Rc::new(RefCell::new(None));
  let (_, connect_drag_source) = use_drag(
    MosaicDragSource::new(MosaicDragType::WINDOW)
      .item(move || {
        on_drag_start.unwrap();
        Some(mosaic_id.clone())
      })
      .options(|_| {
        Default::default()
      })
      .end(|_, monitor| {
        let drop_result = monitor.get_drop_result();

        // do nothing when the user wants to cancel a dragged panel
        if drop_result == None || !monitor.did_drop() {
          return;
        }

        let { position, path, tab_id } = drop_result.unwrap();
        // dropping outside mosaic does nothing. If we have a tabId, but no
        // position or path, we're dragging into an empty tab.
        if (position == None || path == None) && tab_id.is_none() {
          // when dragging a panel into an empty layout treat it link clicking the panel
          // mosaic doesn't give us a position or path to invoke onDrop
          onClick();
          return;
        }
        let { type, config } = panel;
        on_drop(type.clone(), config.clone(), position.unwrap(), path.clone());
      }),
  );

  useEffect(() => {
    if (highlighted && scroll_ref.borrow().is_some()) {
      let highlighted_item = scroll_ref.borrow().clone().unwrap();
      let scroll_container = highlighted_item.parent();
      if let Some(scroll_container) = scroll_container {
        let scroll_container_to_top = scroll_container
          .parent()
          .unwrap()
          .parent()
          .unwrap()
          .getBoundingClientRect()
          .top;

        let is_in_view =
          highlighted_item.getBoundingClientRect().top >= 0 &&
          highlighted_item.getBoundingClientRect().top >= scroll_container_to_top &&
          highlighted_item.getBoundingClientRect().top + 50 <= window.innerHeight;

        if (!is_in_view) {
          scroll_container.scrollIntoView();
        }
      }
    }
  }, [highlighted]);

  let merged_ref = useCallback(
    (el: Element | Null) => {
      connect_drag_source(el);
      *scroll_ref.borrow_mut() = Some(el);
    },
    [connect_drag_source, scroll_ref],
  );

  let target_string = panel.extension_namespace
    .unwrap_or_default()
    .to_owned();

  let onClick_with_stop_propagation = useCallback(
    (event: MouseEvent) => {
      event.stopPropagation();
      onClick();
    },
    [onClick],
  );

  html! {
    <Tooltip
      placement="right"
      enterDelay={500}
      leaveDelay={0}
      slots={{ transition: Fade }}
      title={
        <Stack paddingTop={0.25} style={{ width: 200 }}>
          {panel.thumbnail.is_some()
            .then(|thumbnail| {
              if let Some(thumbnail) = thumbnail {
                html! {
                  <img src={thumbnail} alt={panel.title} />
                }
              } else {
                null
              }
            })
            .or_else(|| {
              html! {
                <Typography variant="body2" fontWeight="bold">
                  {panel.title}
                </Typography>
              }
            })},
          <Stack padding={1} gap={0.5}>
            <Typography variant="body2" style={{ opacity: 0.6 }}>
              {panel.description}
            </Typography>
          </Stack>
        </Stack>
      }
    >
      <ListItem dense disablePadding>
        <ListItemButton
          selected={highlighted}
          className={classes.listItemButton}
          disabled={checked}
          ref={merged_ref}
          onClick={onClick_with_stop_propagation}
        >
          <ListItemText>
            <span data-testid={`panel-menu-item ${panel.title}`}>
              <TextHighlight target_str={target_string} searchText={search_query} />
            </span>
          </ListItemText>

          <ReOrderDotsVertical16Filled className={classes.dragIcon} />
        </ListItemButton>
      </ListItem>
    </Tooltip>
  }
}
```