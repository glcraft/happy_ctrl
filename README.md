# Happy Lighting Controller

A simple command-line utility to control "Happy Lighting" Bluetooth LED strips.

## How it works

This program uses the `btleplug` library to scan for and connect to a specified Bluetooth Low Energy (BLE) device. It then sends specific byte commands to a characteristic (`UUID 0xFFD5`) to control the LED strip's power and color.

## Prerequisites

- Rust and Cargo installed.
- A working Bluetooth adapter on your system.

## Building

To build the project, run the following command:

```sh
cargo build --release
```

The executable will be located at `target/release/happy_ctrl`.

## Usage

The basic command structure is as follows:

```sh
happy_ctrl [OPTIONS] <NAME> <COMMAND>
```

### Arguments

- `<NAME>`: The name of the Bluetooth device to connect to (e.g., "HappyLighting"). The program will connect to the first device found containing this name.

### Options

- `-a, --adapter <ADAPTER>`: Specify the name of the Bluetooth adapter to use. If not provided, the first available adapter will be used.
- `-v, --verbose`: Increase the verbosity of the output. Can be used multiple times (e.g., `-v`, `-vv`, `-vvv`).
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

### Commands

- `power-on`: Turns the LED strip on.
- `power-off`: Turns the LED strip off.
- `set-rgb <R> <G> <B>`: Sets a custom color using RGB values.
  - `<R>`: Amount of red (0-255)
  - `<G>`: Amount of green (0-255)
  - `<B>`: Amount of blue (0-255)

## Examples

Turn the device named "HappyLighting" on:
```sh
./target/release/happy_ctrl "HappyLighting" power-on
```

Turn the device off:
```sh
./target/release/happy_ctrl "HappyLighting" power-off
```

Set the color to pure red:
```sh
./target/release/happy_ctrl "HappyLighting" set-rgb 255 0 0
```

Use a specific bluetooth adapter and show more detailed logs:
```sh
./target/release/happy_ctrl -vv --adapter "hci0" "HappyLighting" power-on
```
