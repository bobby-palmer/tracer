use std::ops::{Add, Sub, Mul, Div};

type Unit = f32;

#[derive(PartialEq, Clone, Copy, Debug)]
struct Vec3 (Unit, Unit, Unit);

macro_rules! derive_self_ops {
    ($trait_name: ident $op: ident $op_symbol: tt) => {
        impl $trait_name for Vec3 {
            type Output = Vec3;

            fn $op(self, rhs: Vec3) -> Vec3 {
                Vec3(
                    self.0 $op_symbol rhs.0,
                    self.1 $op_symbol rhs.1,
                    self.2 $op_symbol rhs.2,
                    )
            }
        }
    }
}

derive_self_ops!(Add add +);
derive_self_ops!(Sub sub -);

macro_rules! derive_unit_ops {
    ($trait_name: ident $op: ident $op_symbol: tt) => {
        impl $trait_name<Unit> for Vec3 {
            type Output = Vec3;

            fn $op(self, rhs: Unit) -> Self::Output {
                Vec3(
                    self.0 $op_symbol rhs,
                    self.1 $op_symbol rhs,
                    self.2 $op_symbol rhs,
                    )
            }
        }
        impl $trait_name<Vec3> for Unit {
            type Output = Vec3;

            fn $op(self, rhs: Vec3) -> Self::Output {
                Vec3 (
                    self $op_symbol rhs.0,
                    self $op_symbol rhs.1,
                    self $op_symbol rhs.2,
                    )
            }
        }
    }
}

derive_unit_ops!(Mul mul *);
derive_unit_ops!(Div div /);
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
