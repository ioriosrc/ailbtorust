```rust
use crate::suite::layout_state::*;
use crate::suite_base::context::current_layout_context::*;
use crate::suite_base::core::{VariableValue};
use std::collections::HashMap;

pub type GlobalVariables = HashMap<String, VariableValue>;

const EMPTY_GLOBAL_VARIABLES: GlobalVariables = HashMap::new();

fn global_variables_selector(state: &LayoutState) -> &GlobalVariables {
  state.selected_layout.as_ref().map_or(&EMPTY_GLOBAL_VARIABLES, |layout| layout.data.global_variables)
}

pub fn use_global_variables() -> (
  GlobalVariables,
  fn(arg0: GlobalVariables) -> (),
  fn(arg0: GlobalVariables) -> (),
) {
  let set_global_variables = use_current_layout_actions(|actions| actions.set_global_variables);
  let overwrite_global_variables = use_current_layout_actions(|actions| actions.overwrite_global_variables);

  let global_variables = use_current_layout_selector(global_variables_selector);

  (global_variables, set_global_variables, overwrite_global_variables)
}
```