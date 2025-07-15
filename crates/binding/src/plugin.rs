use std::sync::Arc;

use rspack_core::{
  ApplyContext, Compilation, CompilationProcessAssets, CompilerOptions, Plugin, PluginContext,
};
use rspack_error::Result;
use rspack_hook::{plugin, plugin_hook};
use rspack_sources::{ConcatSource, RawSource, SourceExt};

/// A plugin that adds a banner to the output `main.js`.
#[derive(Debug)]
#[plugin]
pub struct MyBannerPlugin {
  banner: String,
}

impl MyBannerPlugin {
  pub fn new(banner: String) -> Self {
    Self::new_inner(banner)
  }
}

#[plugin_hook(CompilationProcessAssets for MyBannerPlugin, stage = Compilation::PROCESS_ASSETS_STAGE_ADDITIONS, tracing = false)]
async fn process_assets(&self, compilation: &mut Compilation) -> Result<()> {
  let asset = compilation.assets_mut().get_mut("main.js");
  if let Some(asset) = asset {
    let original_source = asset.get_source().cloned();
    asset.set_source(Some(Arc::new(ConcatSource::new([
      RawSource::from(self.banner.as_str()).boxed(),
      original_source.unwrap().boxed(),
    ]))));
  }

  Ok(())
}

impl Plugin for MyBannerPlugin {
  fn name(&self) -> &'static str {
    "MyBannerPlugin"
  }

  fn apply(
    &self,
    ctx: PluginContext<&mut ApplyContext>,
    _options: &CompilerOptions,
  ) -> rspack_error::Result<()> {
    ctx
      .context
      .compilation_hooks
      .process_assets
      .tap(process_assets::new(self));
    Ok(())
  }
}
