use std::ops;

type Unit = f32;

#[derive(PartialEq)]
struct Vec3(Unit, Unit, Unit);

impl Vec3 {
    fn x(&self) -> Unit {
        self.0
    } 
    fn y(&self) -> Unit {
        self.1
    }
    fn z(&self) ->Unit {
        self.2
    }
}
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
            )
    }
}
impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self (
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
            )
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec3(5.0, 6.0, 7.0);
        let v2 = Vec3(2.0, 3.0, 4.0);
        let v3 = Vec3(7.0, 9.0, 11.0);
        assert!(v1 + v2 == v3);
    }

    #[test]
    fn test_subtract() {
        let v1 = Vec3(10.0, 10.0, 10.0);
        let v2 = Vec3(3.0, 7.0, 5.0);
        let v3 = Vec3(7.0, 3.0, 5.0);
        assert!(v1 - v2 == v3);
    }
}
