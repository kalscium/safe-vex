use vex_rt::{motor::{EncoderUnits, Gearset, Motor as VexMotor, MotorError}, rtos::Mutex};
use crate::{context::Context, log::Log, bind::Bind};

/// A safe wrapper & abstraction of a vex vrc motor
pub struct Motor<'a> {
    context: &'a Mutex<Context>,
    motor: Option<VexMotor>,
    port: u8,
    gear_ratio: Gearset,
    unit: EncoderUnits,
    reverse: bool,
    logged_disconnect: bool,
}

impl<'a> Motor<'a> {
    /// Builds a new motor safely from a context, port, gear_ratio, encoder unit and if it is reversed or not.
    #[inline]
    pub fn build_motor(context: &'a Mutex<Context>, port: u8, gear_ratio: Gearset, unit: EncoderUnits, reverse: bool) -> Self {
        let mut this = Self {
            context,
            port,
            gear_ratio,
            unit,
            reverse,
            motor: None,
            logged_disconnect: false,
        };

        this.build_inner_motor();
        this
    }

    /// Tries to build motor, logs if it can't
    #[inline]
    fn build_inner_motor(&mut self) {
        if self.motor.is_none() {
            if let Ok(x) = unsafe { VexMotor::new(self.port, self.gear_ratio, self.unit, self.reverse) } {
                self.motor = Some(x);
                self.context.lock().log(Log::MotorConnect(self.port));
                self.logged_disconnect = false;
            }
        }
    }

    /// Moves the motor by a specific voltage safely
    #[inline]
    pub fn move_voltage(&mut self, voltage: i32) {
        if let Some(x) = &mut self.motor {
            if x.move_voltage(voltage).is_err() && !self.logged_disconnect {
                self.logged_disconnect = true;
                self.context.lock().log(Log::MotorDisconnect(self.port));
            } else if self.logged_disconnect {
                self.context.lock().log(Log::MotorConnect(self.port));
            }
        } else { self.build_inner_motor() }
    }
}

impl Bind for Motor<'_> {
    type Input = VexMotor;
    type Output = Result<(), MotorError>;

    /// Safely gives access to the underlying vex motor safely
    #[inline]
    fn bind(&mut self, f: &'static mut impl FnMut(&Self::Input) -> Self::Output) {
        if let Some(x) = &mut self.motor {
            if f(x).is_err() && !self.logged_disconnect {
                self.logged_disconnect = true;
                self.context.lock().log(Log::MotorDisconnect(self.port));
            } else if self.logged_disconnect {
                self.context.lock().log(Log::MotorConnect(self.port));
            }
        } else { self.build_inner_motor() }
    }
}