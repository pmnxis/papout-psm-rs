# `papout-psm-rs`
![PCB 3D MODEL](./docs/pcb_image.png)

### Experimental firmware project with **RUST** and **STM32G0**.

Parallel to Serial Adapter for Korean Paper Dispenser Machine.
Some wellknown korean paper dispenser have two type of device.
Serial model communicate to host with RS232.
Parallel model communicate with open-drain based on to pull-up lines (3in/3out).

- HW repo : `release asap` (Kicad 5.99)
- HW Schematics : [Schematic](docs/PapoutPSM-HW-Schematics.pdf)
- Used MCU : `STM32G030F6P6` (Cortex-M0+, 32KiB Flash, 8KiB SRAM, TSSOP-20)

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

## License
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)