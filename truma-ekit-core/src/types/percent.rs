/// Represent a percentage with a precision of 0.01% .
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Percent(i32);

impl Percent {
    pub fn from_f32(val: f32) -> Self {
        Percent((val * 100.0) as i32)
    }

    pub fn to_f32(&self) -> f32 {
        self.0 as f32 / 100.0
    }
}

impl std::ops::Add for Percent {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Percent(self.0.add(rhs.0))
    }
}

impl std::ops::Sub for Percent {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Percent(self.0.sub(rhs.0))
    }
}
