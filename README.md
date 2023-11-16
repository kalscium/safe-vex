# safe-vex
A safe, reliable and comprehensive wrapper around the vex-rt library that will never panic!

## Disclamer
`safe-vex` is an open-source community project. `safe-vex` is neither endorsed by or affiliated with Innovation First, Inc. VEX and VEX Robotics are trademarks or service marks of Innovation First, Inc. `safe-vex` is also not developed by the same developers at [vex-rt](https://crates.io/crates/vex-rt) rather it is an independant project.

## Quickstart
you will need:
1. A rust toolchain managed by `rustup`
2. An `arm-none-eabi` toolchain
3. The `pros-cli` installed throught `pip`
then:
1. Clone the `safe-vex` project [template](https://github.com/GreenChild04/safe-vex-template) by running this command: ```sh
git clone https://github.com/GreenChild04/safe-vex-template.git
```
2. Enter the newly cloned directory
3. Turn on and connect to the vex v5 brain
4. Run `cargo run --release` while connected to the v5 brain