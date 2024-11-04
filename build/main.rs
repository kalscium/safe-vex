
use std::{
    env, io,
    path::{Path, PathBuf},
    process, str,
};

use zip_extensions::zip_extract;

// Path to PROS release zip (relative to project root)
const PROS_ZIP_STR: &str = "build/kernel@4.1.0.zip";

// Path to PROS wrapper.h (relative to project root)
const PROS_WRAPPER_STR: &str = "build/wrapper.h";

// Types to be included by bindgen
const WHITELISTED_TYPES: &[&str] = &[];

// Enums to be treated as bitfields/bitflags by bindgen
const BITFIELD_ENUMS: &[&str] = &["ext_adi_port_config_e"];

// Functions to be included by bindgen
const WHITELISTED_FUNCS: &[&str] = &[
    "battery_get_capacity",
    "battery_get_current",
    "battery_get_temperature",
    "battery_get_voltage",
    "controller_clear",
    "controller_clear_line",
    "controller_get_analog",
    "controller_get_battery_capacity",
    "controller_get_battery_level",
    "controller_get_digital",
    "controller_is_connected",
    "controller_rumble",
    "controller_set_text",
    "distance_get",
    "distance_get_confidence",
    "distance_get_object_size",
    "distance_get_object_velocity",
    "ext_adi_analog_calibrate",
    "ext_adi_analog_read",
    "ext_adi_analog_read_calibrated",
    "ext_adi_analog_read_calibrated_HR",
    "ext_adi_digital_read",
    "ext_adi_digital_write",
    "ext_adi_encoder_get",
    "ext_adi_encoder_init",
    "ext_adi_encoder_reset",
    "ext_adi_encoder_shutdown",
    "ext_adi_gyro_init",
    "ext_adi_gyro_get",
    "ext_adi_gyro_reset",
    "ext_adi_gyro_shutdown",
    "ext_adi_port_set_config",
    "ext_adi_ultrasonic_get",
    "ext_adi_ultrasonic_init",
    "ext_adi_ultrasonic_shutdown",
    "adi_analog_calibrate",
    "adi_analog_read",
    "adi_analog_read_calibrated",
    "adi_analog_read_calibrated_HR",
    "adi_digital_read",
    "adi_digital_write",
    "adi_encoder_get",
    "adi_encoder_init",
    "adi_encoder_reset",
    "adi_encoder_shutdown",
    "adi_gyro_init",
    "adi_gyro_get",
    "adi_gyro_reset",
    "adi_gyro_shutdown",
    "adi_port_set_config",
    "adi_ultrasonic_get",
    "adi_ultrasonic_init",
    "adi_ultrasonic_shutdown",
    "imu_reset",
    "imu_get_rotation",
    "imu_get_heading",
    "imu_get_quaternion",
    "imu_get_euler",
    "imu_get_pitch",
    "imu_get_roll",
    "imu_get_yaw",
    "imu_get_gyro_rate",
    "imu_get_accel",
    "imu_get_status",
    "imu_tare_heading",
    "imu_tare_rotation",
    "imu_tare_pitch",
    "imu_tare_roll",
    "imu_tare_yaw",
    "imu_tare_euler",
    "imu_tare",
    "imu_set_euler",
    "imu_set_rotation",
    "imu_set_heading",
    "imu_set_pitch",
    "imu_set_roll",
    "imu_set_yaw",
    "millis",
    "micros",
    "motor_get_actual_velocity",
    "motor_get_brake_mode",
    "motor_get_current_draw",
    "motor_get_current_limit",
    "motor_get_direction",
    "motor_get_efficiency",
    "motor_get_encoder_units",
    "motor_get_gearing",
    "motor_get_position",
    "motor_get_power",
    "motor_get_target_position",
    "motor_get_target_velocity",
    "motor_get_temperature",
    "motor_get_torque",
    "motor_get_velocity",
    "motor_get_voltage",
    "motor_get_voltage_limit",
    "motor_is_over_current",
    "motor_is_over_temp",
    "motor_is_reversed",
    "motor_modify_profiled_velocity",
    "motor_move",
    "motor_move_absolute",
    "motor_move_relative",
    "motor_move_velocity",
    "motor_move_voltage",
    "motor_set_brake_mode",
    "motor_set_current_limit",
    "motor_set_encoder_units",
    "motor_set_gearing",
    "motor_set_reversed",
    "motor_set_voltage_limit",
    "motor_set_zero_position",
    "motor_tare_position",
    "mutex_delete",
    "mutex_recursive_create",
    "mutex_recursive_give",
    "mutex_recursive_take",
    "registry_get_plugged_type",
    "rotation_get_angle",
    "rotation_get_position",
    "rotation_get_reversed",
    "rotation_get_velocity",
    "rotation_reset",
    "rotation_reset_position",
    "rotation_reverse",
    "rotation_set_position",
    "rotation_set_reversed",
    "sem_create",
    "sem_delete",
    "sem_get_count",
    "sem_post",
    "sem_wait",
    "serial_enable",
    "serial_flush",
    "serial_get_read_avail",
    "serial_get_write_free",
    "serial_peek_byte",
    "serial_read",
    "serial_read_byte",
    "serial_set_baudrate",
    "serial_write",
    "serial_write_byte",
    "task_create",
    "task_delay",
    "task_delay_until",
    "task_delete",
    "task_get_by_name",
    "task_get_current",
    "task_get_name",
    "task_get_priority",
    "task_get_state",
    "task_notify",
    "task_notify_take",
    "usd_is_installed",
    "usd_list_files",
];

// Variables to be included by bindgen
const WHITELISTED_VARS: &[&str] = &[
    "INTERNAL_ADI_PORT",
    "PROS_ERR_",
    "PROS_ERR_F_",
    "TASK_PRIORITY_DEFAULT",
    "TASK_PRIORITY_MAX",
    "TASK_STACK_DEPTH_DEFAULT",
];

fn main() -> Result<(), io::Error> {
    // tell cargo to rerun this script if it's dependent files change
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed={}", PROS_ZIP_STR);
    println!("cargo:rerun-if-changed={}", PROS_WRAPPER_STR);

    // define input paths
    let pros_zip_path = PathBuf::from(PROS_ZIP_STR);
    let wrapper_h_path = PathBuf::from(PROS_WRAPPER_STR);

    // define output paths
    let out_dir_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let pros_extract_path = out_dir_path.join("pros");
    let bindings_gen_path = out_dir_path.join("bindings.rs");

    // extract pros firmware
    zip_extract(&pros_zip_path, &pros_extract_path)?;

    // tell cargo where to find pros link scripts and libraries
    println!(
        "cargo:rustc-link-search={}",
        pros_extract_path.join("firmware").display()
    );

    let args = get_args(&pros_extract_path);
    generate_bindings(&args, &wrapper_h_path, &bindings_gen_path)?;

    Ok(())
}

/// Finds the GCC sysroot.
fn get_sysroot() -> String {
    let output = process::Command::new("arm-none-eabi-gcc")
        .args(["--print-sysroot"])
        .output()
        .expect("failed to execute arm-none-eabi-gcc. is the arm-none-eabi toolchain installed?");

    let sysroot = str::from_utf8(&output.stdout).unwrap_or("").trim();

    if sysroot.is_empty() {
        for dir in ["/usr/lib/arm-none-eabi", "/usr/arm-none-eabi"] {
            if Path::new(dir).is_dir() {
                return dir.to_string();
            }
        }

        panic!("Could not determine GCC sysroot path!")
    } else {
        sysroot.to_string()
    }
}

/// Generates required compiler args for `arm-none-eabi` and pros.
fn get_args(pros_extract_path: &Path) -> Vec<String> {
    let sysroot = get_sysroot();

    vec![
        format!("-I{}", pros_extract_path.join("include").to_str().unwrap()),
        "-isystem".to_string(),
        format!("{}/include", sysroot),
        "-cxx-isystem".to_string(),
        format!("{}/include/c++", sysroot),
    ]
}

/// Generates bindings using bindgen.
fn generate_bindings(
    args: &[String],
    wrapper_h_path: &Path,
    bindings_gen_path: &Path,
) -> Result<(), io::Error> {
    let mut bindings = bindgen::Builder::default()
        .header(wrapper_h_path.to_str().unwrap())
        .clang_arg("-target")
        .clang_arg("arm-none-eabi")
        .clang_args(args)
        .ctypes_prefix("libc")
        .use_core()
        .layout_tests(false);

    for t in WHITELISTED_TYPES {
        bindings = bindings.allowlist_type(t);
    }

    for t in BITFIELD_ENUMS {
        bindings = bindings.bitfield_enum(t);
    }

    for f in WHITELISTED_FUNCS {
        bindings = bindings.allowlist_function(f);
    }

    for v in WHITELISTED_VARS {
        bindings = bindings.allowlist_var(v);
    }

    bindings
        .generate()
        .expect("Could not generate bindings.")
        .write_to_file(bindings_gen_path)?;

    Ok(())
}
