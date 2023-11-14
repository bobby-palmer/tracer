mod vec3;
mod ray;
mod hittable;
mod renderer;

fn main() {
    let c = renderer::Renderer::build(256, 256);
    c.render();
}
