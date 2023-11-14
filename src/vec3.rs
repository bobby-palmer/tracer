use std::ops::{Add, Sub};

type Unit = f32;

#[derive(PartialEq, Clone)]
struct Vec3 (Unit, Unit, Unit);

macro_rules! derive_binary_ops {
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

derive_binary_ops!(Add add +);
derive_binary_ops!(Sub sub -);

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
