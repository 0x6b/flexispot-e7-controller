# DEVELOPMENT.md

## Tested Environment

- MacBook Pro 14-inch, 2021 (M1 Max)
- macOS Sonoma 14.2.1 (23C71)
- Rust 1.77.0-nightly (f688dd684 2024-01-04)
- [M5Stamp C3 Mate with Pin Headers](https://shop.m5stack.com/products/m5stamp-c3-mate-with-pin-headers) (ESP32-C3)

## Setup and Verify Development Environment and Hardware

As a first step, install prerequisites.

```console
$ brew install cmake ninja dfu-util
$ cargo install ldproxy
$ cargo install espflash
```

### Test Your Setup

Now simple project will work. Create a new with `cargo-generate`.

```console
$ cargo install cargo-generate
$ cargo generate esp-rs/esp-idf-template cargo
...
$ cd your-starter-project-name
```

Edit `rust-toolchain.toml` to specify the channel to `nightly-2024-01-04`.

```console
$ $EDITOR rust-toolchain.toml
```

You have to specify `nightly-2024-01-04` explicitly since, both `nightly-2024-01-{05,06,07}` won't work for me with following error message, as of 2024-01-07.

```
error[E0425]: cannot find value `SOMAXCONN` in crate `libc`
  --> /.../.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/os/unix/net/listener.rs:87:48
   |
87 |             const backlog: libc::c_int = libc::SOMAXCONN;
   |                                                ^^^^^^^^^ not found in `libc`
```

Connect your device to MacBook and build/install the program with `cargo`. Serial port on my setup is `/dev/cu.usbserial-54F70047301 - USB Single Serial`.

```console
$ cargo run
...
```

Once you have determined the serial port, you can specify it with `ESPFLASH_PORT` environment variable.

```console
$ ESPFLASH_PORT=/dev/cu.usbserial-54F70047301 cargo run
```

### Test Your ESP32-C3

Based on https://github.com/esp-rs/std-training/tree/127e6dc1e40194c0473975315bfa4643011e69cc/intro/hardware-check.

```console
$ ESP32C3_SSID=... ESP32C3_PSK=... cargo run --bin hardware_check
```

## Debugging

```console
$ espflash monitor
```

## References

- [The Rust on ESP Book](https://esp-rs.github.io/book/introduction.html)
- [esp-rs/std-training](https://github.com/esp-rs/std-training/tree/main)
