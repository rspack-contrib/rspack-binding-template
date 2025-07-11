mod plugin;

use napi::bindgen_prelude::*;
use rspack_binding_builder_macros::register_plugin;
use rspack_core::BoxPlugin;

#[macro_use]
extern crate napi_derive;
extern crate rspack_binding_builder;

register_plugin!("ExamplePlugin", |_env: Env, _options: Unknown<'_>| {
  Ok(Box::new(plugin::ExamplePlugin) as BoxPlugin)
});
