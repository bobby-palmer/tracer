use crate::vec3::{Vec3, Color};

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
                let r = row;
                let g = col;
                let b = 0;
                println!("{} {} {}", r, g, b);
            }
        }
    }
}
