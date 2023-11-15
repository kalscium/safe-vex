pub struct JoyStick {
    pub x: i8,
    pub y: i8,
}

impl JoyStick {
    #[inline]
    pub fn step(&mut self, min: u8) -> &mut Self {
        if self.x.unsigned_abs() < min { self.x = 0 }
        if self.y.unsigned_abs() < min { self.y = 0 }
        self
    }

    #[inline]
    pub fn x_larger(&self) -> bool {
        self.x.unsigned_abs() > self.y.unsigned_abs()
    }

    #[inline]
    pub fn get_larger(self, other: JoyStick) -> JoyStick {
        let largest = if self.x_larger() { self.x.unsigned_abs() } else { self.y.unsigned_abs() };
        let other_largest = if other.x_larger() { other.x.unsigned_abs() } else { other.y.unsigned_abs() };

        if largest > other_largest {
            self
        } else { other }
    }
}