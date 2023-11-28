use vex_rt::motor::{EncoderUnits, Gearset, Motor as VexMotor, MotorError};
use crate::{context::Context, log::Log};

/// A safe wrapper & abstraction of a vex vrc motor
pub struct Motor {
    motor: Option<VexMotor>,
    port: u8,
    gear_ratio: Gearset,
    unit: EncoderUnits,
    reverse: bool,
    is_motor_disconnected: bool,
}

impl Motor {
    /// Builds a new motor safely from a context, port, gear_ratio, encoder unit and if it is reversed or not.
    #[inline]
    pub fn build_motor(context: &Context, port: u8, gear_ratio: Gearset, unit: EncoderUnits, reverse: bool) -> Self {
        let mut this = Self {
            port,
            gear_ratio,
            unit,
            reverse,
            motor: None,
            is_motor_disconnected: false,
        };

        this.build_inner_motor(context);
        this
    }

    /// Tries to build motor, logs if it can't
    #[inline]
    fn build_inner_motor(&mut self, context: &Context) {
        if self.motor.is_none() {
            if let Ok(x) = unsafe { VexMotor::new(self.port, self.gear_ratio, self.unit, self.reverse) } {
                self.motor = Some(x);
                context.log(Log::MotorConnect(self.port));
                self.is_motor_disconnected = false;
            }
        }
    }

    /// Moves the motor by a specific voltage safely
    #[inline]
    pub fn move_voltage(&mut self, context: &Context, voltage: i32) {
        if let Some(x) = &mut self.motor {
            if x.move_voltage(voltage).is_err() && !self.is_motor_disconnected {
                self.is_motor_disconnected = true;
                context.log(Log::MotorDisconnect(self.port));
            } else if self.is_motor_disconnected {
                context.log(Log::MotorConnect(self.port));
            }
        } else { self.build_inner_motor(context) }
    }

    /// Moves the motor by a specific voltage safely
    #[inline]
    pub fn inner_motor(&mut self, context: &Context, mut f: impl FnMut(&mut VexMotor) -> Result<(), MotorError>) {
        if let Some(x) = &mut self.motor {
            if f(x).is_err() && !self.is_motor_disconnected {
                self.is_motor_disconnected = true;
                context.log(Log::MotorDisconnect(self.port));
            } else if self.is_motor_disconnected {
                context.log(Log::MotorConnect(self.port));
            }
        } else { self.build_inner_motor(context) }
    }
}