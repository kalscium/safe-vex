/// Represents a state of the controller
#[derive(Debug, Clone)]
pub struct JoyStick {
    pub x: i8,
    pub y: i8,
}

impl JoyStick {
    /// Clamps the joystick so it has to be a minimum amount of activation before being registered (to avoid stick-drift)
    #[inline]
    pub fn clamp(mut self, min: u8) -> Self {
        if self.x.unsigned_abs() < min { self.x = 0 }
        if self.y.unsigned_abs() < min { self.y = 0 }
        self
    }

    /// Checks if the `x` of the stick has more activation than the `y` of the stick.
    #[inline]
    pub fn x_larger(&self) -> bool {
        self.x.unsigned_abs() > self.y.unsigned_abs()
    }

    /// Returns the more activated stick from two
    #[inline]
    pub fn get_larger(self, other: JoyStick) -> JoyStick {
        let largest = if self.x_larger() { self.x.unsigned_abs() } else { self.y.unsigned_abs() };
        let other_largest = if other.x_larger() { other.x.unsigned_abs() } else { other.y.unsigned_abs() };

        if largest > other_largest {
            self
        } else { other }
    }
}
