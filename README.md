# ESP32-C3 embassy template
This project provides a working (as of 2023-05-13) template to run [embassy](https://embassy.dev) an an ESP32-C3 development board.

As there are no full examples for running embassy on an ESP32-C3, this project is intended to be a starting point for your own project. It includes all the necessary dependencies and a simple example that toggles GPIO 5 and therefore blinks the onboard LED (at least for my board).

## Getting started
To get started, you need to add the `riscv32imc-unknown-none-elf` target. You can do this by running the following command:
```bash
rustup target add riscv32imc-unknown-none-elf --toolchain nightly
```

For flashing the board, you need to install the `espflash` cargo extension (or use the `espflash` CLI tool). You can do this by running the following command:
```bash
cargo install cargo-espflash
```
The `cargo espflash` command will automatically build your project and flash it to the board.
