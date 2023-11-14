use vex_rt::motor::{EncoderUnits, Gearset, Motor as VexMotor, MotorError};

pub struct Motor {
    motor: Option<VexMotor>,
    port: u8,
    gear_ratio: Gearset,
    unit: EncoderUnits,
    reverse: bool,
}

impl Motor {
    #[inline]
    pub fn build_motor(port: u8, gear_ratio: Gearset, unit: EncoderUnits, reverse: bool) -> Self {
        let mut this = Self {
            port,
            gear_ratio,
            unit,
            reverse,
            motor: None,
        };

        this.build_inner_motor();
        this
    }

    #[inline]
    fn build_inner_motor(&mut self) {
        if let None = self.motor {
            if let Ok(x) = unsafe { VexMotor::new(self.port, self.gear_ratio, self.unit, self.reverse) } {
                self.motor = Some(x);
            }
        }
    }

    #[inline]
    pub fn move_voltage(&mut self, voltage: i32) {
        if let Some(x) = &mut self.motor {
            if let Err(_) = x.move_voltage(voltage) {
                // error handling here
            }
        } else { self.build_inner_motor() }
    }

    #[inline]
    pub fn get_motor(&mut self, f: impl Fn(&mut VexMotor) -> Result<(), MotorError>) {
        if let Some(x) = &mut self.motor {
            if let Err(_) = f(x) {
                // error handling here
            }
        } else { self.build_inner_motor() }
    }
}
