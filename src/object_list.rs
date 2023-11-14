use crate::hittable::Hittable;
pub struct ObjectList {
    objects: Vec<Box<dyn Hittable>>
}
impl Hittable for ObjectList {
    fn hit(&self, ray: &crate::ray::Ray) -> Option<(crate::vec3::Color, crate::ray::Ray, f32)> {
        self.objects.iter().filter_map(|obj| {
            obj.hit(ray)
        }).min_by(|t1, t2| t1.2.partial_cmp(&t2.2).unwrap())
    }
}
impl ObjectList {
    pub fn new() -> Self {
        ObjectList{
            objects: vec!()
        }
    }
}
