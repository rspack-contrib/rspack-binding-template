use rspack_core::{ApplyContext, CompilerOptions, Plugin, PluginContext};

#[derive(Debug)]
pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
  fn name(&self) -> &'static str {
    "ExamplePlugin"
  }

  fn apply(
    &self,
    _ctx: PluginContext<&mut ApplyContext>,
    _options: &CompilerOptions,
  ) -> rspack_error::Result<()> {
    Ok(())
  }
}
