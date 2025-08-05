mod loader;
mod plugin;

use napi::bindgen_prelude::*;
use rspack_binding_builder_macros::register_plugin;
use rspack_core::BoxPlugin;

#[macro_use]
extern crate napi_derive;
extern crate rspack_binding_builder;

// Export a plugin named `MyBannerPlugin`.
//
// The plugin needs to be wrapped with `require('@rspack/core').experiments.createNativePlugin`
// to be used in the host.
//
// Check out `lib/index.js` for more details.
//
// `register_plugin` is a macro that registers a plugin.
//
// The first argument to `register_plugin` is the name of the plugin.
// The second argument to `register_plugin` is a resolver function that is called with `napi::Env` and the options returned from the resolver function from JS side.
//
// The resolver function should return a `BoxPlugin` instance.
register_plugin!("MyBannerPlugin", |_env: Env, options: Unknown<'_>| {
  let banner = options
    .coerce_to_string()?
    .into_utf8()?
    .as_str()?
    .to_string();
  Ok(Box::new(plugin::MyBannerPlugin::new(banner)) as BoxPlugin)
});

// Export a plugin named `MyBannerLoaderPlugin`.
register_plugin!(
  "MyBannerLoaderPlugin",
  |_env: Env, _options: Unknown<'_>| {
    Ok(Box::new(loader::MyBannerLoaderPlugin::new()) as BoxPlugin)
  }
);
