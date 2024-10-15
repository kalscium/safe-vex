//! The Real-Time OS API

use crate::bindings;

/// Gets the number of milliseconds since PROS initialized
pub fn millis() -> u32 {
    unsafe {
        bindings::millis()
    }
}

/// Gets the number of microseconds since PROS initialized
pub fn micros() -> u64 {
    unsafe {
        bindings::micros()
    }
}

/// Delays the current task for a given number of milliseconds
pub fn task_delay(milliseconds: u32) {
    unsafe {
        bindings::task_delay(milliseconds);
    }
}

/// Delays the current task until a specified time
pub fn task_delay_until(prev_time: &mut u32, delta: u32) {
    unsafe {
        bindings::task_delay_until(prev_time as *mut u32, delta);
    }
}
