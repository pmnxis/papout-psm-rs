# Binary Size Notes

## Get Program Size
```shell
// with debug build
cargo size --bin papout-psm-rs -- -A
// with release build
cargo size --bin papout-psm-rs --release -- -A
```

## If you failed to get info.
```shell
Failed to execute tool: size
No such file or directory (os error 2)
```
If you get `Failed to execute tool: size` from cargo install llvm-tools-preview
```shell
rustup component add llvm-tools-preview
```

## STM32G030 with Debug version.
STM3G030 32KiB flash model cannot contain default unopimization option for dev build.
When build dev binrary with RTIC example for Cortex-M0+ , the binary size is 33KiB. So I added `opt-level = 1` option for dev build.
- Reference : https://doc.rust-lang.org/cargo/reference/profiles.html
