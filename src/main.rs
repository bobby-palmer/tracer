mod vec3;
mod ray;
mod hittable;
mod renderer;
mod util;
mod object_list;

fn main() {
    let mut world = object_list::ObjectList::new();
    let c = renderer::Renderer::build(256, 256, world);
    c.render();
}
