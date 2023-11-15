/// A Representation of the current state of a controller joystick
pub struct JoyStick {
    pub x: i8,
    pub y: i8,
}

impl JoyStick {
    /// Steps/Limits/Claps whatever you call it; it makes it so that the joystick has a minimum amount of activation or else it is dropped to zero (to advoid stick-drift).
    #[inline]
    pub fn step(&mut self, min: u8) -> &mut Self {
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