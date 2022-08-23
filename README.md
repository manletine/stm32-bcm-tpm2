# STM32 BCM TPM2
This repository contains STM32 BCM TPM2 light controller software.

# Features
- Utilizes the STM32F103C8T6 microcontroller

# Getting Started

## Dependencies
To build and flash this project you will need:

- OpenOCD. [Installation instructions](http://openocd.org/getting-openocd/).
- Rust toolchain. [Installation instructions](https://www.rust-lang.org/learn/get-started). After installation run:
    ```
    $ cd stm32-bcm-tpm2/
    $ rustup override set nightly
    ```
- `rust-std` components for the `thumbv7m-none-eabi` target. Run:
    ```
    $ rustup target add thumbv7m-none-eabi
    ```
- `binutils`. [Installation instructions](https://www.gnu.org/software/binutils/).

## Building
To build this project, run:
```
$ cargo build --release
```

## Flashing
To flash this project, run:
```
$ ./flash.sh
```

# License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
