# rhole: Rust local DNS adblocker

## Cross-compilation for Raspberry Pi

Add this to your `.cargo/config` file:

```toml
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

Install armv7 toolchain: 

```bash
rustup target add armv7-unknown-linux-gnueabihf
```

And finally compile it:

```bash
cargo build --release --target armv7-unknown-linux-gnueabihf
```