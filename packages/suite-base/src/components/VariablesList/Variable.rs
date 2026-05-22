```rust
use react::prelude::*;
use materialui::{icons::ArrowDropDownIcon, icons::ErrorIcon, icons::MoreVertIcon};
use materialui::{components::{Divider, IconButton, Menu, MenuItem, ListItem, ListItemButton, ListItemText, Typography}, styled::*};
use lodash::Map;
use suite_base::services::AppEvent;
use suite_base::hooks::useGlobalVariables;
use suite_base::context::AnalyticsContext;

const useStyles = make_styles! {
  root: {
    "@media (pointer: fine)": {
      [`&:not(:hover) .${classes.copyButton}`]: {
        visibility: "hidden",
      },
    },
  },
  copyButton: {
    top: 0,
    right: 0,
    zIndex: theme.z_index.mobile_stepper,

    "&.MuiButton-root": {
      position: "absolute",
      paddingLeft: theme.spacing(1),
      paddingRight: theme.spacing(1),
      margin: theme.spacing(0.75),
      minWidth: "auto",
    },
  },
  input: {
    font: "inherit",
    flex: "auto",

    ".MuiInputBase-input": {
      padding: 0,
    },
    "&.Mui-error": {
      color: theme.palette.error.main,
    },
  },
  edgeEnd: {
    marginRight: theme.spacing(-1.625),
  },
  editorWrapper: {
    position: "relative",
    backgroundColor: theme.palette.grey[50],
  },
  listItemButton: {
    "&:focus-within": {
      backgroundColor: "transparent",
    },
    "&.Mui-selected": {
      color: theme.palette.primary.main,
      transition: `background-color 300ms ease-in-out`,
    },
  },
  listItemText: {
    marginTop: theme.spacing(0.125),
    marginBottom: theme.spacing(0.125),
  },
};

fn change_global_key(
  new_key: &str,
  old_key: &str,
  global_variables: Map<String, serde_json::Value>,
  idx: usize,
  overwrite_global_variables: impl FnOnce(Map<String, serde_json::Value>) -> (),
) {
  let keys = global_variables.keys().cloned().collect::<Vec<&str>>();
  overwrite_global_variables(keys.iter().filter(|k| k != old_key).map(|k| (*k, *global_variables.get(k).unwrap())).collect());
}

#[function_component(Variable)]
fn Variable(props: Props) -> JSXElement {
  let { name, selected } = props;

  let classes = use_styles();

  // When editing the variable name, the new name might collide with an existing variable name
  // If the name matches an existing name, we set the edited name and show an error to the user
  // indicating there is a name conflict. The user must resolve the name conflict or their edited
  // name will be reset on blur.
  let [edited_name, set_edited_name] = useState<Option<String>>();

  let [expanded, set_expanded] = useState(true);
  let [anchor_el, set_anchor_el] = React.use_state<MaybeElement>(None);
  let [copied, set_copied] = useState(false);
  let menu_open = React.use_ref(false);

  use_context::<AnalyticsContext>().subscribe_to_event(AppEvent::VARIABLE_DELETE, |_e| {
    set_expanded(false);
  });

  let global_variables = use_global_variables();

  let handleClick = (event: MouseEvent<HTMLButtonElement>) => {
    set_anchor_el(event.currentTarget);
  };
  let handleClose = () => {
    set_anchor_el(None);
  };

  let analytics = use_context::<AnalyticsContext>().clone();

  let delete_variable = useCallback(() => {
    overwrite_global_variables(Map::from([(name, None)]));
    void analytics.log_event(AppEvent::VARIABLE_DELETE);
    handleClose();
  }, [analytics, name]);

  let value = useMemo(
    () => global_variables.get(name).map(serde_json::Value::as_object).unwrap_or_default(),
    [global_variables, name],
  );

  let onChange_value = useCallback(
    (new_val: serde_json::Value) => {
      overwrite_global_variables(Map::from([(name, new_val)]));
      set_copied(false);
    },
    [name, overwrite_global_variables],
  );

  let onBlur = () => {
    if (
      edited_name != None &&
      global_variables.get(edited_name.unwrap()).is_none() &&
      name != edited_name.unwrap()
    ) {
      change_global_key(&edited_name.unwrap(), name, global_variables.clone().unwrap(), props.index as usize, |new_var| overwrite_global_variables(new_var));
    }
    set_edited_name(None);
  };

  let root_ref = React.use_ref<HTMLElement>(None);

  let active_element_is_child = if let Some(ref el) = root_ref.current {
    el.contains(document.active_element)
  } else {
    false
  };

  let isSelected = selected && !active_element_is_child;
  let is_duplicate =
    edited_name != None && edited_name.unwrap() != name && global_variables.get(edited_name.unwrap()).is_some();

  let get_text = useCallback(() => serde_json::to_string_pretty(&value).unwrap_or_default(), [value]);

  return (
    <Stack className={classes.root} ref={root_ref}>
      <ListItem
        dense
        disablePadding
        secondaryAction={
          <Stack className={classes.edgeEnd} direction="row" alignItems="center" gap={0.25}>
            <IconButton
              size="small"
              id="variable-action-button"
              data-testid="variable-action-button"
              aria-controls={expanded ? "variable-action-menu" : undefined}
              aria-haspopup="true"
              aria-expanded={expanded ? "true" : undefined}
              onClick={handleClick}
            >
              <MoreVertIcon fontSize="small" />
            </IconButton>
            <Menu
              id="variable-action-menu"
              anchorEl={anchor_el}
              open={menu_open.current}
              onClose={handleClose}
              slotProps={{
                list: {
                  "aria-labelledby": "variable-action-button",
                  dense: true,
                },
              }}
            >
              <MenuItem onClick={delete_variable}>
                <Typography color="error.main" variant="inherit">
                  Delete variable
                </Typography>
              </MenuItem>
            </Menu>
          </Stack>
        }
      >
        <ListItemButton
          className={classes.listItemButton}
          selected={isSelected}
          onClick={() => {
            set_expanded(!expanded);
          }}
        >
          <ListItemText
            className={classes.listItemText}
            primary={
              <Stack direction="row" alignItems="center" style={{ marginLeft: -12 }}>
                <ArrowDropDownIcon
                  style={{ transform: !expanded ? "rotate(-90deg)" : undefined }}
                />
                <InputBase
                  className={classes.input}
                  autoFocus={name == ""}
                  error={is_duplicate}
                  value={edited_name.unwrap_or(name)}
                  placeholder="variable_name"
                  data-testid={`global-variable-key-input-${name}`}
                  onClick={(e) => {
                    e.stopPropagation();
                  }}
                  onFocus={() => {
                    if (edited_name.is_none()) {
                      set_expanded(true);
                    }
                  }}
                  onChange={(event) => {
                    set_edited_name(Some(event.target.value));
                  }}
                  onBlur={onBlur}
                  endAdornment={
                    is_duplicate && (
                      <Tooltip
                        arrow
                        title="A variable with this name already exists. Please select a unique variable name to save changes."
                      >
                        <ErrorIcon className={classes.edgeEnd} fontSize="small" color="error" />
                      </Tooltip>
                    )
                  }
                />
              </Stack>
            }
            slotProps={{
              primary: {
                component: "div",
                fontWeight: 600,
                variant: "body2",
              },
            }}
          />
        </ListItemButton>
      </ListItem>
      {expanded && (
        <div className={classes.editorWrapper}>
          <Divider />
          <CopyButton
            className={classes.copyButton}
            size="small"
            color={copied ? "primary" : "inherit"}
            getText={get_text}
          >
            {copied ? "Copied" : "Copy"}
          </CopyButton>
          <JsonInput
            data-testid="global-variable-value-input"
            value={value}
            onChange={onChange_value}
          />
        </div>
      )}
      <Divider />
    </Stack>
  );
}

impl Default for Variable {
  fn default() -> Self {
    Self {
      name: "example".to_string(),
      selected: false,
    }
  }
}
```