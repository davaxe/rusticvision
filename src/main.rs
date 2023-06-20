use rusticvision::scene;

fn main() {
    let directory = "test";
    let obj_file = "test_multiple_mats.obj";
    let (mesh, mat_map, obj_map) =
        scene::obj_parser::get_triangle_mesh_and_obj_map(directory, obj_file);
    let scene = scene::Scene::new(&mesh, &obj_map, &mat_map);
    scene.debug_print_objects();
}
