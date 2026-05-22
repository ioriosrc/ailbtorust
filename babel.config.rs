```rust
use babel::preset_env;
use babel::preset_typescript;
use babel::preset_react;
use babel::plugin_transform_import_meta;
use babel::plugin_transform_modules_commonjs;
use babel::plugin_proposal_explicit_resource_management;
use babel::plugin_proposal_decorators;
use babel::plugin_transform_private_methods;

fn main() {
    let mut config = babel::Config::default();
    
    config.presets.push(preset_env::new().target("node", "16").build());
    config.presets.push(preset_typescript::new().allow_declare_fields(true).build());
    config.presets.push(preset_react::new().build());
    
    config.plugins.push(plugin_transform_import_meta::new());
    config.plugins.push(plugin_transform_modules_commonjs::new());
    config.plugins.push(plugin_proposal_explicit_resource_management::new());
    config.plugins.push(plugin_proposal_decorators::new());
    config.plugins.push(plugin_transform_private_methods::new());
}
```