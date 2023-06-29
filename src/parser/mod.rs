use std::{collections::HashMap, sync::Arc};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::{complete::line_ending, is_newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use crate::{primitives::TriangleMesh, scene::object::Object};

mod face_parser;
mod material_parser;
mod vertex_parser;

struct ObjParts {
    // A string containing the vertex data (positions, normals, textures
    // coordinates)
    pub vertex_data: String,

    // The name of the material file to be used.
    pub material_file_name: String,

    // Map from object name to the faces of the object (material included)
    pub object_map: HashMap<String, String>,
}

#[derive(Debug)]
enum ParseLineResult<'original> {
    VertexPosition(&'original str),
    VertexNormal(&'original str),
    VertexTexture(&'original str),
    ObjectName(&'original str),
    UseMaterial(&'original str),
    Face(&'original str),
    MaterialFile(&'original str),
    Comment(&'original str),
    Other(&'original str),
}

/// Get triangle mesh and an object map from an obj file. The obj file is
/// expected to be in the triangle format.
///
/// ### Arguments
/// - `directory` - The directory where the obj file is located.
/// - `obj_file` - The name of the obj file.
///
/// ### Returns
/// A tuple containing the triangle mesh, object map and material map. The
/// object map maps the object name to the faces (string format) that belong to
/// that object. The material map maps the material name to the index of the
/// material in the triangle mesh.
///
/// ### Panics
/// The function panics when:
/// 1. The obj file cannot be read.
/// 2. The obj file cannot be parsed.
/// 3. The material file cannot be read.
/// 4. The material file cannot be parsed.
///
pub fn get_triangle_mesh_and_obj_map(
    directory: &str,
    obj_file: &str,
) -> (
    TriangleMesh,
    HashMap<String, usize>,
    HashMap<String, String>,
) {
    let obj_path = format!("{}/{}", directory, obj_file);
    let obj_file = std::fs::read_to_string(obj_path).expect("PARSE_OBJ: Unable to read obj file");

    // Extract the vertex data, material file and object map from the obj file
    let (_, object_parts) =
        extract_parts_obj(&obj_file).expect("PARSE_OBJ: Failed to extract parts from obj file");

    // Get vertex data from the vertex data string
    let (_, (vp, vn, _)) = vertex_parser::parse_vertex_data(&object_parts.vertex_data).unwrap();

    // Load the materials from the material file.
    let mat_path = format!("{}/{}", directory, object_parts.material_file_name);
    let mat_file = std::fs::read_to_string(mat_path.trim_end())
        .expect("PARSE_OBJ: Unable to read material file");
    let (_, (materials, material_map)) = material_parser::materials(&mat_file).unwrap();

    // Create the triangle mesh that stores the vertex data and materials
    let triangle_mesh = TriangleMesh::new(vp, vn, materials, vec![]);

    (triangle_mesh, material_map, object_parts.object_map)
}

pub fn get_objects(
    triangle_mesh: TriangleMesh,
    object_map: &HashMap<String, String>,
    material_map: &HashMap<String, usize>,
) -> (Vec<Object>, Arc<TriangleMesh>) {
    // Save data for later use, so we can create the objects when we have the
    // triangle full triangle mesh.
    let mut objects_data: Vec<(String, usize, usize)> = Vec::new();
    let mut triangle_mesh = triangle_mesh;
    for (name, faces_str) in object_map {
        let (_, indices) = face_parser::parse_triangle_indices(faces_str, material_map)
            .expect("PARSE: Failed to parse triangle faces");
        objects_data.push((
            name.clone(),
            triangle_mesh.triangle_indices().len(),
            indices.len(),
        ));
        triangle_mesh.extend_triangle_indices(&indices);
    }

    let triangle_mesh = Arc::new(triangle_mesh);

    let objects = objects_data
        .into_iter()
        .map(|(name, start, count)| Object::new(name, start, count, Arc::clone(&triangle_mesh)))
        .collect();

    (objects, triangle_mesh)
}

/// Parse a line of the input string slice.
fn line(input: &str) -> IResult<&str, &str> {
    let (input, line) = take_till(|c| is_newline(c as u8))(input)?;
    Ok((input, line))
}

/// Parse the vertex position data (string format) from the input string slice.
fn parse_vertex_position(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("v "), line)(input)?;
    Ok((input, ParseLineResult::VertexPosition(pos)))
}

/// Parse the vertex normal data (string format) from the input string slice.
fn parse_vertex_normal(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, normal) = preceded(tag("vn "), line)(input)?;
    Ok((input, ParseLineResult::VertexNormal(normal)))
}

/// Parse the vertex texture data (string format) from the input string slice.
fn parse_vertex_texture(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, texture) = preceded(tag("vt "), line)(input)?;
    Ok((input, ParseLineResult::VertexTexture(texture)))
}

/// Parse the object name from the input string slice.
fn parse_object_name(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, object_name) = preceded(tag("o "), line)(input)?;
    Ok((input, ParseLineResult::ObjectName(object_name)))
}

/// Parse the material name from the input string slice.
fn parse_use_material(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, use_mtl) = preceded(tag("usemtl "), line)(input)?;
    Ok((input, ParseLineResult::UseMaterial(use_mtl)))
}

/// Parse the face data (string format) from the input string slice.
fn parse_face(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, face) = preceded(tag("f "), line)(input)?;
    Ok((input, ParseLineResult::Face(face)))
}

/// Parse the material file name from the input string slice.
fn parse_material_file(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, mtl_name) = preceded(tag("mtllib "), line)(input)?;
    Ok((input, ParseLineResult::MaterialFile(mtl_name)))
}

/// Parse the comment from the input string slice.
fn parse_comment(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, comment) = preceded(tag("#"), line)(input)?;
    Ok((input, ParseLineResult::Comment(comment)))
}

/// Parse the other data from the input string slice.
fn parse_other(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, other) = line(input)?;
    Ok((input, ParseLineResult::Other(other)))
}

/// Parse a line of an obj file. Returns the line as ParseLineResult
fn parse_obj_line(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, result) = alt((
        parse_vertex_position,
        parse_vertex_normal,
        parse_vertex_texture,
        parse_object_name,
        parse_use_material,
        parse_face,
        parse_material_file,
        parse_comment,
        parse_other,
    ))(input)?;
    Ok((input, result))
}

/// Extract different parts of the obj file.
///
/// ### Returns
/// A tuple containing:
/// - vertex data (String). A string containing the vertex data in the obj file
///   format.
/// - material file name (String). The name of the material file to be used.
/// - object map (HashMap<String, String>) A map from object name to the faces
///   of the object.
fn extract_parts_obj(input: &str) -> IResult<&str, ObjParts> {
    let mut vertex_data = String::new();
    let mut material_file = String::new();
    let mut object_map = HashMap::new();

    let (input, lines) = separated_list1(line_ending, parse_obj_line)(input)?;

    let mut current_object = "";
    lines.into_iter().for_each(|l| {
        use ParseLineResult::*;
        match l {
            VertexPosition(pos) => vertex_data.push_str(&format!("v {}\n", pos)),
            VertexNormal(pos) => vertex_data.push_str(&format!("vn {}\n", pos)),
            VertexTexture(pos) => vertex_data.push_str(&format!("vt {}\n", pos)),
            ObjectName(name) => {
                current_object = name;
                object_map.insert(current_object.to_owned(), String::new());
            }
            UseMaterial(material) => {
                object_map
                    .get_mut(current_object)
                    .unwrap()
                    .push_str(&format!("usemtl {}\n", material));
            }
            Face(face) => {
                object_map
                    .get_mut(current_object)
                    .unwrap()
                    .push_str(&format!("f {}\n", face));
            }
            MaterialFile(file) => material_file.push_str(&format!("{}\n", file)),
            _ => {}
        }
    });
    Ok((
        input,
        ObjParts {
            vertex_data,
            material_file_name: material_file,
            object_map,
        },
    ))
}
