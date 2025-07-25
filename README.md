# Rspack binding template

📚 [Rspack Custom Binding Guide](https://rspack-contrib.github.io/rspack-rust-book/custom-binding/getting-started/index.html)

Rspack achieves high performance by being written in Rust, but using its JavaScript API introduces overhead due to cross-language calls. This can limit performance and access to native Rust features.

_Rspack Custom Binding_ allows you to extend Rspack directly with native Rust code, avoiding the JavaScript layer and unlocking full performance and flexibility.

With custom binding, you can still use the familiar JavaScript API (`@rspack/core`), but your custom logic runs natively, combining the best of both worlds.
