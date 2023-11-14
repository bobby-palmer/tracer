use crate::hittable::Hittable;
use crate::vec3::{Vec3, Color};
use crate::util;

pub struct Renderer {
    height: i32,
    width: i32,
    world: Box<dyn Hittable>,
}
impl Renderer {
    pub fn build(height: i32, width: i32, world: impl Hittable + 'static) -> Self {
        Renderer {height, width, world: Box::new(world)}
    }
    pub fn render(&self) {
        println!("P3");
        println!("{0} {1}", self.height, self.width);
        for row in 0..self.height {
            for col in 0..self.width {
                util::write_color(&Self::get_pixel_color(row, col));
            }
        }
    }
    fn get_pixel_color(row: i32, col: i32) -> Color {
        todo!();
    }
}
