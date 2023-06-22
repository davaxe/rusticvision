use rusticvision::scene::{self, renderer::render};

fn main() {
    let directory = "test";
    let obj_file = "monkey.obj";
    render(directory, obj_file);
}
