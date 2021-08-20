# Binary Size Notes

## Get Program Size
```shell
// with debug build
cargo size --bin papout-psm-rs -- -A
// with release build
cargo size --bin papout-psm-rs --release -- -A
```

## STM32G030 with Debug version.
STM3G030 32KiB flash model cannot contain default unopimization option for dev build.
When build dev binrary with RTIC example for Cortex-M0+ , the binary size is 33KiB. So I added `opt-level = 1` option for dev build.
- Reference : https://doc.rust-lang.org/cargo/reference/profiles.html
