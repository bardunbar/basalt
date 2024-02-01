use std::ops;
use std::ops::Neg;

use num_traits::Num;
use num_traits::ToPrimitive;

const SQRT_3: f32 = 1.73205080757;
const HALF_SQRT_3: f32 = SQRT_3 / 2.0;

pub trait HexNum: Num + ToPrimitive + Copy + ops::Neg<Output = Self> {}
impl<T> HexNum for T where T: Num + ToPrimitive + Copy + ops::Neg<Output = Self> {}

pub struct Axial<T: HexNum> {
    q: T,
    r: T,
}

pub struct Cube<T: HexNum> {
    q: T,
    r: T,
    s: T,
}

impl<T: HexNum> Axial<T> {
    pub fn new(q: T, r: T) -> Self {
        Axial { q, r }
    }

    pub fn to_cube(&self) -> Cube<T> {
        Cube { q: self.q, r: self.r, s: -self.q - self.r }
    }

    pub fn to_cartesian(&self) -> (f32, f32) {

        let q = self.q.to_f32().unwrap_or_default();
        let r = self.r.to_f32().unwrap_or_default();

        (SQRT_3 * q + HALF_SQRT_3 * r, 1.5 * r)
    }
}

impl<T: HexNum> ops::Add for Axial<T> {
    type Output = Axial<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Axial { q: self.q + rhs.q, r: self.r + rhs.r }
    }
}

impl<T: HexNum> ops::AddAssign for Axial<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.q = self.q + rhs.q;
        self.r = self.r + rhs.r;
    }
}