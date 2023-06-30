struct Material {
    diffuse_color: vec3<f32>
    // TODO: add more properties
}

struct TriangleIndex {
    v0: u32,
    v1: u32,
    v2: u32,
    normal: u32,
    material: u32
}

struct AABB {
    min: vec4<f32>,
    max: vec4<f32>
}

struct Object {
    aabb_index: u32,
    triangles_start: u32,
    triangles_count: u32
}

struct ObjectData {
    count: u32,
    objects: array<Object>
}

struct Camera {
    position: vec4<f32>,
    inverse_projection: mat4x4<f32>,
    inverse_view: mat4x4<f32>,
    width: u32,
    height: u32,
    temp: array<u32>
}


// Data buffers for the scene, loaded from CPU
@group(0)
@binding(0)
var<storage, read> vertex_positions: array<vec4<f32>>;

@group(0)
@binding(1)
var<storage, read> vertex_normals: array<vec4<f32>>;

// @group(0)
// @binding(2)
// var<storage, read> materials: array<Material>;

@group(0)
@binding(2)
var<storage, read> triangle_indices: array<TriangleIndex>;

@group(0)
@binding(3)
var<storage, read> aabbs: array<AABB>;

@group(0)
@binding(4)
var<storage, read> objects_data: ObjectData;

@group(0)
@binding(5)
var<storage, read_write> pixels: array<vec4<f32>>;


// @group(0)
// @binding(7)
// var<storage, read> random_numbers: array<f32>;

// Camera data, loaded from CPU
@group(0)
@binding(6)
var<storage, read> camera: Camera;

@group(0)
@binding(7)
var<storage, read> start_rng_state: array<u32>;

// Structs used in compute shader
struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>
}

struct Hit {
    hit: bool,
    distance: f32,
    triangle_index: TriangleIndex,
}


/// Update state after generating a random value with any of the random functions below.
fn next_random_state(state: u32) -> u32 {
    var state = state * 747796405u + 2891336453u;
    var result: u32 = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    result = (result >> 22u) ^ result;
    return result;
}

fn random_value(state: u32) -> f32 {
    return f32(state) / 4294967295.0;
}

fn random_value_normal_distribution(state: u32) -> f32 {
    var theta: f32 = 2.0 * 3.1415926 * random_value(state);
    var state: u32 = next_random_state(state);
    var rho: f32 = sqrt(-2.0 * log(random_value(state)));
    return rho * cos(theta);
}

fn random_direction(state: u32) -> vec3<f32> {
    var x: f32 = random_value_normal_distribution(state);
    var state: u32 = next_random_state(state);
    var y: f32 = random_value_normal_distribution(state);
    state = next_random_state(state);
    var z: f32 = random_value_normal_distribution(state);
    return normalize(vec3<f32>(x, y, z));
}

fn max_element(v: vec3<f32>) -> f32 {
    return max(v.x, max(v.y, v.z));
}

fn min_element(v: vec3<f32>) -> f32 {
    return min(v.x, min(v.y, v.z));
}

fn intersect_ray_aabb(ray: Ray, aabb: AABB, t_min: f32, t_max: f32) -> bool {
    var inv_dir: vec3<f32> = 1.0 / ray.direction;
    var t0: vec3<f32> = (aabb.min.xyz - ray.origin.xyz) * inv_dir;
    var t1: vec3<f32> = (aabb.max.xyz - ray.origin.xyz) * inv_dir;

    var t_small: vec3<f32> = min(t0, t1);
    var t_big: vec3<f32> = max(t0, t1);

    var t_min: f32 = max(t_min, max_element(t_small));
    var t_max: f32 = min(t_max, min_element(t_big));

    return t_max >= t_min;
}

fn intersect_ray_triangle(ray: Ray, triangle_index: TriangleIndex, t_min: f32, t_max: f32) -> Hit {
    var v0: vec3<f32> = vertex_positions[triangle_index.v0].xyz;
    var v1: vec3<f32> = vertex_positions[triangle_index.v1].xyz;
    var v2: vec3<f32> = vertex_positions[triangle_index.v2].xyz;
    var edge0: vec3<f32> = v1 - v0;
    var edge1: vec3<f32> = v2 - v0;
    var h: vec3<f32> = cross(ray.direction, edge1);
    var a: f32 = dot(edge0, h);

    if abs(a) < 0.00001 {
        return Hit(false, 0.0, triangle_index);
    }

    var f: f32 = 1.0 / a;
    var s: vec3<f32> = ray.origin - v0;
    var u: f32 = f * dot(s, h);

    if (u < 0.0 || u > 1.0) {
        return Hit(false, 0.0, triangle_index);
    }

    var q: vec3<f32> = cross(s, edge0);
    var v: f32 = f * dot(ray.direction, q);

    if (v < 0.0 || u + v > 1.0) {
        return Hit(false, 0.0, triangle_index);
    }

    var t: f32 = f * dot(edge1, q);

    if (t < t_min || t > t_max) {
        return Hit(false, 0.0, triangle_index);
    }

    return Hit(true, t, triangle_index);
}

fn intersect_ray_object(ray: Ray, object: Object, t_min: f32, t_max: f32) -> Hit {
    var aabb: AABB = aabbs[object.aabb_index];
    if !intersect_ray_aabb(ray, aabb, t_min, t_max) {
        return Hit(false, 0.0, TriangleIndex(0u, 0u, 0u, 0u, 0u));
    }

    var closest_hit: Hit = Hit(false, 0.0, TriangleIndex(0u, 0u, 0u, 0u, 0u));
    var closest_distance: f32 = t_max;

    for (var i: u32 = 0u; i < object.triangles_count; i = i + 1u) {
        var triangle_index: TriangleIndex = triangle_indices[object.triangles_start + i];
        var hit: Hit = intersect_ray_triangle(ray, triangle_index, t_min, closest_distance);
        if (hit.hit) && (hit.distance < closest_distance) {
            closest_hit = hit;
            closest_distance = hit.distance;
        }
    }
    return closest_hit;
}

fn intersect(ray: Ray, start_rng_state: u32) -> Hit {
    var closest_hit: Hit = Hit(false, 0.0, TriangleIndex(0u, 0u, 0u, 0u, 0u));
    var closest_distance: f32 = 1000000.0;

    for (var i: u32 = 0u; i < objects_data.count; i = i + 1u) {
        var object: Object = objects_data.objects[i];
        var hit: Hit = intersect_ray_object(ray, object, 0.001, closest_distance);
        if (hit.hit) && (hit.distance < closest_distance) {
            closest_hit = hit;
            closest_distance = hit.distance;
        }
    }
    return closest_hit;
}

fn get_ray(x: u32, y: u32) -> Ray {
    // Normalize pixel coordinates to [-1, 1]
    var x: f32 = f32(x) / f32(camera.width);
    var y: f32 = f32(y) / f32(camera.height);
    var coord: vec2<f32> = vec2<f32>(x, y);
    coord = coord * 2.0 - 1.0;
    var t: vec4<f32> = camera.inverse_projection * vec4<f32>(coord, 0.0, 1.0);
    var a: vec3<f32> = normalize(t.xyz / t.w);
    let ray_dir: vec4<f32> = camera.inverse_view * vec4<f32>(a, 0.0);

    return Ray(camera.position.xyz, ray_dir.xyz);
}

fn trace(ray: Ray, rng_state: u32) -> vec3<f32> {
    var hit: Hit = intersect(ray, rng_state);
    if (!hit.hit) {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    var normal: vec3<f32> = vertex_normals[hit.triangle_index.normal].xyz;
    var color: vec3<f32> = normal;
    return color;
}

fn color(x: u32, y: u32, rng_state: u32) -> vec3<f32> {
    var ray: Ray = get_ray(x, y);
    let color: vec3<f32> = trace(ray, rng_state);
    return color;
}

fn get_pixel_coord(index: u32) -> vec2<u32> {
    var x: u32 = index % camera.width;
    var y: u32 = index / camera.width;
    return vec2<u32>(x, y);
}

@compute
@workgroup_size(144)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var coord = get_pixel_coord(global_id.x);
    var rng_state = start_rng_state[global_id.x];
    var color = color(coord.x, coord.y, rng_state);
    pixels[global_id.x] = vec4<f32>(color, 1.0);
}