```rust
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

struct Renderer {
  // Define the renderer fields here
}

impl Renderer {
  pub fn new(canvas: &HtmlCanvasElement) -> Self {
    // Initialize the renderer with the provided canvas element
    Renderer { /* implementation */ }
  }

  fn handle_settings_action(&mut self, action: SettingsTreeAction) {
    match action.action {
      "reorder-node" => {
        // Handle reorder-node action here
        if let Some(payload) = action.payload {
          // Perform necessary logic to reorder layers or nodes in the scene extension
        }
      },
      _ => {/* Handle other actions if needed */},
    }
  }

  fn update_config(&mut self, config: &mut Config) {
    // Update the renderer's configuration with the provided settings
  }
}

struct Grids {
  // Define the grids fields here
}

impl Grids {
  pub fn new() -> Self {
    // Initialize the grids
    Grids { /* implementation */ }
  }

  fn update_settings_tree(&mut self, config: &Config) {
    // Update the settings tree for the grids extension
  }

  fn save_setting(&mut self, layer_id: String, key: &str, value: &serde_json::Value) {
    // Save a setting in the settings tree for the specified layer
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_handle_settings_action_reorder_node() {
    // Test the handle_settings_action function with reorder-node action
  }

  #[test]
  fn test_handle_settings_action_other_actions() {
    // Test other possible actions in handle_settings_action
  }

  #[test]
  fn test_update_config() {
    // Test updating the renderer's configuration
  }

  #[test]
  fn test_update_settings_tree() {
    // Test updating the settings tree for the grids extension
  }

  #[test]
  fn test_save_setting() {
    // Test saving a setting in the settings tree
  }
}
```