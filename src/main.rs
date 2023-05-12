use std::env;

fn main() {
    raytracer::render_image(&env::args().nth(1).unwrap());
}
