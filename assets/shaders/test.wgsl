@group(0)
@binding(0)
var<storage, read> vertex_positions: array<vec3<f32>>;

@group(0)
@binding(1)
var<storage, read> vertex_normals: array<vec3<f32>>;

@group(0)
@binding(2)
var<storage, read> materials: array<Material>;

@group(0)
@binding(3)
var<storage, read> aabbs: array<AABB>;

@group(0)
@binding(4)
var<storage, read> objects: array<Object>;

struct Material {
    diffuse_color: vec3<f32>
}

struct AABB {
    min: vec3<f32>,
    max: vec3<f32>
}

struct Object {
    material_index: u32,
    aabb_index: u32,
    triangles_start: u32,
    triangles_count: u32
}