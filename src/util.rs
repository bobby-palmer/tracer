use crate::vec3::{Color, Vec3};
pub fn write_color(c: &Color) {
    let Vec3(r, g, b) = *c;
    println!("{} {} {}", 
             color_correct(r),
             color_correct(g),
             color_correct(b));
}
fn color_correct(val: f32) -> i32 {
    (val.sqrt().clamp(0.0, 0.999) * 256.0) as i32
}
