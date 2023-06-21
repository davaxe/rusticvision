use rusticvision::scene;

fn main() {
    let directory = "test";
    let obj_file = "test_multiple_mats.obj";
    let (mesh, mat_map, obj_map) =
        scene::parser::get_triangle_mesh_and_obj_map(directory, obj_file);

    let objects = scene::parser::get_objects(&mesh, &obj_map, &mat_map);

    let scene = scene::Scene::with_default_camera(&mesh, objects);
    scene.debug_print_objects();
}
