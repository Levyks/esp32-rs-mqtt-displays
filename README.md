# esp32-tokio-boilerplate

Minimal project boilerplate for running async rust programs with tokio on an ESP32.

* Make sure you have all the tools and dependencies mentioned in [The Rust on ESP Book](https://docs.esp-rs.org/book/installation/index.html) for std development on your required target.
* Make sure the correct target for your ESP is uncommented in the `.cargo/config.toml` and `rust-toolchain.toml` files.
* Run `cargo run` and have fun!


Based on [esp-rs/esp-idf-template](https://github.com/esp-rs/esp-idf-template)