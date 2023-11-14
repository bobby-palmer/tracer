use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

type Unit = f32;

#[derive(PartialEq, Clone, Copy, Debug)]
struct Vec3 (Unit, Unit, Unit);

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut v = self;
        v += rhs;
        v
    }
}
impl Neg for Vec3 {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Vec3(
            -self.0,
            -self.1,
            -self.2,
            )
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut v = self;
        v -= rhs;
        v
    }
}
impl MulAssign<Unit> for Vec3 {
    fn mul_assign(&mut self, rhs: Unit) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}
impl Mul<Unit> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Unit) -> Self::Output {
        let mut v = self;
        v *= rhs;
        v
    }
}
impl Mul<Vec3> for Unit {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
impl DivAssign<Unit> for Vec3 {
    fn div_assign(&mut self, rhs: Unit) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}
impl Div<Unit> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Unit) -> Self::Output {
        let mut v = self;
        v /= rhs;
        v
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let v1 = Vec3(5.0, 6.0, 7.0);
        let v2 = Vec3(2.0, 3.0, 4.0);
        let v3 = Vec3(7.0, 9.0, 11.0);
        assert!(v1 + v2 == v3);
    }

    #[test]
    fn subtract() {
        let v1 = Vec3(10.0, 10.0, 10.0);
        let v2 = Vec3(3.0, 7.0, 5.0);
        let v3 = Vec3(7.0, 3.0, 5.0);
        assert!(v1 - v2 == v3);
    }

    #[test]
    fn mult() {
        let factor = 5.0;
        let v1 = Vec3(1.0, 3.0, 5.0);
        let v2 = Vec3(5.0, 15.0, 25.0);
        assert_eq!(v1 * factor, v2);
        assert_eq!(factor * v1, v2);
    }

    #[test]
    fn div() {
        let factor = 5.0;
        let v1 = Vec3(1.0, 3.0, 5.0);
        let v2 = Vec3(5.0, 15.0, 25.0);
        assert_eq!(v2 / factor, v1);
    }
}
