use crate::ray::Ray;
use crate::vec3::{Color};
pub trait Hittable {
    fn hit(ray: &Ray) -> Option<(Color, Ray, f32)>;
}
