use crate::vec3::{Vec3, Color};
use crate::util;

pub struct Renderer {
    height: i32,
    width: i32,
}
impl Renderer {
    pub fn build(height: i32, width: i32) -> Self {
        Renderer {height, width}
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
