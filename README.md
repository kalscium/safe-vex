# safe-vex
A modular, safe and data-orientated rust wrapper over the Purdue PROS library for Vex Robotics

## Disclamer
`safe-vex` is an open-source project by [kalscium](https://github.com/kalscium). `safe-vex` is neither endorsed by or affiliated with Innovation First, Inc. VEX and VEX Robotics are trademarks or service marks of Innovation First, Inc. `safe-vex` is also not developed by nor endorsed by the developers of the [Purdue PROS library](https://github.com/purduesigbots/pros).

## Quickstart *(Nix)*
you will need:
1. nix
2. nix flakes

then:
1. Clone the `safe-vex` project [template](https://github.com/kalscium/safe-vex-template) by running the following command:
  ```sh
    git clone https://github.com/kalscium/safe-vex-template.git
  ```
2. Enter the newly cloned directory
3. Enter the *nix* dev-environment with
  ```sh
    nix develop
  ```
4. Turn on and connect to the vex v5 brain
5. Give permission to upload code to the robot with: (doesn't matter if this fails)
  ```sh
    sudo chmod a+rw /dev/ttyACM0 || sudo chmod a+rw /dev/ttyACM1
  ```
6. While connected to the v5 brain run:
  ```sh
    cargo run --release
  ```
7. Your robot should now be up and running :D

## Quickstart *(Debian)*
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
1. Clone the `safe-vex` project [template](https://github.com/kalscium/safe-vex-template) by running the following command:
  ```sh
    git clone https://github.com/kalscium/safe-vex-template.git
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

# Updating the PROS Library Version
---
> *(For future reference for the maintainence of this library)*
To update the pros library version used by `safe-vex`, follow the following steps (with the correct toolchains installed)
1. Enter the `build` directory of the library
## *While inside the `build` dir*
2. Create an empty pros project
  ```sh
    pros conductor new kernel
  ```
3. Delete the unnecessary bloat libraries and files that are irrelevant to `PROS`
  ```sh
    rm -rf kernel/bin kernel/firmware/{okapilib.a,squiggles.mk} kernel/okapi
  ```
4. Rename the `project.pros` file to `template.pros` and take note of the version
  ```sh
    mv kernel/project.pros kernel/template.pros
  ```
  ```json
    {
      "py/state": {
        "templates": {
          "kernel": {
            "target": "v5",
            "version": "<version>, eg 1.0.0 (TAKE NOTE OF THIS FOR LATER STEPS)",
          }
        }
      }
    }
  ```
5. Zip and package the kernel for compilation use
  ```sh
    (cd kernel && zip ../kernel@VERSION.zip -r *)
    rm -rf kernel
  ```
6. Update references to the kernel package in bindgen build code (*still in the build dir* `main.rs`)
  ```rust
    // Path to PROS release zip (relative to project root)
    const PROS_ZIP_STR: &str = "build/kernel@VERSION.zip"; // update the vesrion
  ```
## *In the project root*
7. Clean the project and publish it *(given you have changed the project version in `Cargo.toml` to something appropriate to this update to the kernel)*
  ```sh
    cargo clean
    cargo publish
  ```
