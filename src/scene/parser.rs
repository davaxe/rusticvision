use std::collections::HashMap;

use crate::primitives::{
    material,
    triangle,
    trianglemesh::{self, TriangleMesh},
};
use crate::scene::object::Object;

use glam::{Vec2, Vec3};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::{complete::line_ending, is_newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
enum ParseLineResult<'a> {
    VertexPosition(&'a str),
    VertexNormal(&'a str),
    VertexTexture(&'a str),
    ObjectName(&'a str),
    UseMaterial(&'a str),
    Face(&'a str),
    MaterialFile(&'a str),
    Comment(&'a str),
    Other(&'a str),
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
    let (_, (v_data, material_file, object_map)) =
        extract_parts_obj(&obj_file).expect("PARSE_OBJ: Failed to extract parts from obj file");

    // Get vertex data from the vertex data string
    let (_, (vp, vn, _)) = get_vertex_data(&v_data).unwrap();

    // Load the materials from the material file.
    let mat_path = format!("{}/{}", directory, material_file);
    let mat_file = std::fs::read_to_string(mat_path.trim_end())
        .expect("PARSE_OBJ: Unable to read material file");
    let (_, (materials, material_map)) = material::parser::materials(&mat_file).unwrap();

    // Create the triangle mesh that stores the vertex data and materials
    let triangle_mesh = TriangleMesh::new(vp, vn, materials);

    (triangle_mesh, material_map, object_map)
}

pub fn get_objects<'a, 'b>(
    triangle_mesh: &'a TriangleMesh,
    object_map: &'b HashMap<String, String>,
    material_map: &'b HashMap<String, usize>,
) -> Vec<Object<'a>> {
    let mut objects = Vec::new();
    for (name, faces_str) in object_map {
        let (_, indices) = triangle::parser::parse_triangle_indices(faces_str, material_map)
            .expect("PARSE: Failed to parse triangle faces");
        let object = Object::new(name.to_owned(), indices, triangle_mesh);
        objects.push(object);
    }
    objects
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
    let (input, pos) = preceded(tag("vn "), line)(input)?;
    Ok((input, ParseLineResult::VertexNormal(pos)))
}

/// Parse the vertex texture data (string format) from the input string slice.
fn parse_vertex_texture(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("vt "), line)(input)?;
    Ok((input, ParseLineResult::VertexTexture(pos)))
}

/// Parse the object name from the input string slice.
fn parse_object_name(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("o "), line)(input)?;
    Ok((input, ParseLineResult::ObjectName(pos)))
}

/// Parse the material name from the input string slice.
fn parse_use_material(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("usemtl "), line)(input)?;
    Ok((input, ParseLineResult::UseMaterial(pos)))
}

/// Parse the face data (string format) from the input string slice.
fn parse_face(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("f "), line)(input)?;
    Ok((input, ParseLineResult::Face(pos)))
}

/// Parse the material file name from the input string slice.
fn parse_material_file(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("mtllib "), line)(input)?;
    Ok((input, ParseLineResult::MaterialFile(pos)))
}

/// Parse the comment from the input string slice.
fn parse_comment(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("#"), line)(input)?;
    Ok((input, ParseLineResult::Comment(pos)))
}

/// Parse the other data from the input string slice.
fn parse_other(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = line(input)?;
    Ok((input, ParseLineResult::Other(pos)))
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
fn extract_parts_obj(input: &str) -> IResult<&str, (String, String, HashMap<String, String>)> {
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
            _ => (),
        }
    });
    Ok((input, (vertex_data, material_file, object_map)))
}

type VertexData = (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>);

/// Input an obj file and return the vertex data, material file name and object
/// map.
///
/// ### Returns
/// A tuple containing vertex data:
/// - vertex positions (Vec<Vec3>). A vector containing the vertex positions.
/// - vertex normals (Vec<Vec3>). A vector containing the vertex normals.
/// - vetex texture coordinates (Vec<Vec2>). A vector containing the vertex
fn get_vertex_data(input: &str) -> IResult<&str, VertexData> {
    if let Ok((_, (vp, vn, vt))) = trianglemesh::parser::parse_vertex_data(input) {
        Ok((input, (vp, vn, vt)))
    } else {
        panic!("Failed to parse vertex data");
    }
}
