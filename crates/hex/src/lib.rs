use std::ops;

const SQRT_3: f32 = 1.73205080757;
const HALF_SQRT_3: f32 = SQRT_3 / 2.0;

pub struct HexAxial {
    q: i32,
    r: i32,
}

impl HexAxial {
    pub fn new(q: i32, r: i32) -> Self {
        HexAxial { q, r }
    }

    pub fn to_cube(&self) -> HexCube {
        HexCube { q: self.q, r: self.r, s: -self.q - self.r }
    }

    pub fn to_cartesian(&self) -> (f32, f32) {
        (SQRT_3 * self.q as f32 + HALF_SQRT_3 * self.r as f32, 1.5 * self.r as f32)
    }
}

impl ops::Add for HexAxial {
    type Output = HexAxial;

    fn add(self, rhs: Self) -> Self::Output {
        HexAxial { q: self.q + rhs.q, r: self.r + rhs.r }
    }
}

impl ops::AddAssign for HexAxial {
    fn add_assign(&mut self, rhs: Self) {
        self.q += rhs.q;
        self.r += rhs.r;
    }
}


pub struct HexCube {
    q: i32,
    r: i32,
    s: i32,
}

impl HexCube {
    pub fn to_axial(&self) -> HexAxial {
        HexAxial { q: self.q, r: self.r }
    }
}