[![crates.io](https://img.shields.io/crates/v/cargo-rdme.svg)](https://crates.io/crates/safe-vex)
[![Downloads crates.io](https://img.shields.io/crates/d/safe-vex.svg?label=crates.io%20downloads)](https://crates.io/crates/safe-vex)
[![License](https://img.shields.io/crates/l/safe-vex.svg)](./LICENSE.md)

# safe-vex
A safe, reliable and comprehensive wrapper around the vex-rt library that will never panic!

## Disclamer
`safe-vex` is an open-source community project. `safe-vex` is neither endorsed by or affiliated with Innovation First, Inc. VEX and VEX Robotics are trademarks or service marks of Innovation First, Inc. `safe-vex` is also not developed by the same developers at [vex-rt](https://crates.io/crates/vex-rt) rather it is an independant project.

## Quickstart *(Natively)*
you will need:
1. A rust toolchain managed by `rustup`:
2. An `arm-none-eabi` toolchain
3. `gcc` and `libclang-dev`/`libclang`
4. (optional) `libc6-dev` if rustc is throwing the error:
```
--- stderr
/usr/lib/arm-none-eabi/include/sys/reent.h:14:10: fatal error: 'stddef.h' file not found
thread 'main' panicked at /home/dev/.cargo/registry/src/index.crates.io-6f17d22bba15001f/vex-rt-0.15.1/build/main.rs:266:10:
Could not generate bindings.: ClangDiagnostic("/usr/lib/arm-none-eabi/include/sys/reent.h:14:10: fatal error: 'stddef.h' file not found\n")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
4. The `pros-cli` installed through `pip`

then:
1. Clone the `safe-vex` project [template](https://github.com/GreenChild04/safe-vex-template) by running the following command:
```sh
git clone https://github.com/GreenChild04/safe-vex-template.git
```
2. Enter the newly cloned directory
3. Turn on and connect to the vex v5 brain
4. Give permission to upload code to the robot with: (doesn't matter if this fails)
```sh
sudo chmod a+rw /dev/ttyACM0 || sudo chmod a+rw /dev/ttyACM1
```
5. While connected to the v5 brain run:
```sh
cargo run --release
```
6. Your robot should now be up and running :D

## Quickstart *(with `Docker`)*
you will need:
1. `docker`

then:
1. Clone the `safe-vex` project [template](https://github.com/GreenChild04/safe-vex-template) by running the following command:
```sh
git clone https://github.com/GreenChild04/safe-vex-template.git
```
2. Enter the newly cloned directory
3. Build the docker image with:
```sh
docker build -t safe-vex-template
```
4. Turn on and connect to the vex v5 brain
5. Give permission to upload code to the robot with: (doesn't matter if this fails)
```sh
sudo chmod a+rw /dev/ttyACM0 || sudo chmod a+rw /dev/ttyACM1
```
6. Run the docker container interactively with:
```sh
docker run -it --rm --device=/dev/$(ls /dev/ttyACM*) -v .:/project -v $HOME/.cargo/registry:/home/dev/.cargo/registry safe-vex-template
```
7. Run `cargo run --release` in the docker container while connected to the v5 brain
8. Your robot should now be up and running :D
