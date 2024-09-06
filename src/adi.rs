use vex_rt::adi::{AdiDigitalOutput, AdiDigitalOutputError};

/// Represents an ADI port
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum AdiPort {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
}

/// Safely constructs a new adi digital output
#[inline]
pub unsafe fn new_adi_digital_output(port: AdiPort) -> Result<AdiDigitalOutput, AdiDigitalOutputError> {
    AdiDigitalOutput::new(port as u8, 22)
}
