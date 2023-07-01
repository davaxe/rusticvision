use std::collections::HashMap;

use crate::data_structures::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::{complete::line_ending, is_newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use vertex_parser::VertexData;

mod material_parser;
mod triangle_parser;
mod vertex_parser;

struct ObjParts {
    /// A string containing the vertex data (positions, normals, textures
    /// coordinates)
    pub vertex_data: String,

    /// The name of the material file to be used.
    pub material_file_name: String,

    /// Map from object name to the faces of the object (material included)
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

pub struct ObjData {
    pub vertex_data: VertexData,
    pub material_data: Vec<MaterialData>,
    pub object_data: Vec<ObjectData>,
    pub triangle_data: Vec<TriangleData>,
    pub bounding_boxes: Vec<BoundingBoxData>,
}

/// Get triangle mesh and an object map from an obj file. The obj file is
/// expected to be in the triangle format.
///
/// ### Arguments
/// - `directory` - The directory where the obj file is located.
/// - `obj_file` - The name of the obj file.
///
/// ### Returns
/// All data parsed from the obj file. This includes the vertex data, material
/// data, object data and triangle data. Also returns vector of bounding boxes
/// calculated from the vertex data and triangle data.
///
/// ### Panics
/// The function panics when:
/// 1. The obj file cannot be read.
/// 2. The obj file cannot be parsed.
/// 3. The material file cannot be read.
/// 4. The material file cannot be parsed.
///
pub fn parse_obj(directory: &str, obj_file: &str) -> ObjData {
    let obj_path = format!("{}/{}", directory, obj_file);
    let obj_file = std::fs::read_to_string(obj_path).expect("PARSE_OBJ: Unable to read obj file");

    // Extract the vertex data, material file and object map from the obj file
    let (_, object_parts) =
        extract_parts_obj(&obj_file).expect("PARSE_OBJ: Failed to extract parts from obj file");

    // Get vertex data from the vertex data string
    let (_, vertex_data) = vertex_parser::parse_vertex_data(&object_parts.vertex_data).unwrap();

    // Load the materials from the material file.
    let mat_path = format!("{}/{}", directory, object_parts.material_file_name);
    let mat_file = std::fs::read_to_string(mat_path.trim_end())
        .expect("PARSE_OBJ: Unable to read material file");
    let (_, (material_data, material_map)) = material_parser::materials(&mat_file).unwrap();

    let (object_data, triangle_data) =
        get_object_and_triangle_data(&object_parts.object_map, &material_map);

    let bounding_boxes =
        calculate_bounding_boxes(&vertex_data.positions, &triangle_data, &object_data);

    ObjData {
        vertex_data,
        material_data,
        object_data,
        triangle_data,
        bounding_boxes,
    }
}

/// Extract object data and triangle data from the object map and material map.
fn get_object_and_triangle_data(
    object_map: &HashMap<String, String>,
    material_map: &HashMap<String, usize>,
) -> (Vec<ObjectData>, Vec<TriangleData>) {
    // Save data for later use, so we can create the objects when we have the
    // triangle full triangle mesh.
    let mut objects_data: Vec<ObjectData> = Vec::new();
    let mut triangle_data: Vec<TriangleData> = Vec::with_capacity(500);
    for (idx, (_, faces_str)) in object_map.iter().enumerate() {
        let (_, indices) = triangle_parser::parse_triangles(faces_str, material_map)
            .expect("PARSE: Failed to parse triangle faces");
        let object_data =
            ObjectData::new(idx as u32, triangle_data.len() as u32, indices.len() as u32);
        objects_data.push(object_data);
        triangle_data.extend(indices);
    }
    (objects_data, triangle_data)
}

/// Calculates bounding boxes for all objects.
fn calculate_bounding_boxes(
    vertex_positions: &[Vec3Data],
    triangles: &[TriangleData],
    objects: &[ObjectData],
) -> Vec<BoundingBoxData> {
    objects
        .iter()
        .map(|obj| {
            let start = obj.triangle_start_index as usize;
            let end = start + obj.triangle_count as usize;
            let mut min_pos = [f32::INFINITY; 3];
            let mut max_pos = [f32::NEG_INFINITY; 3];
            triangles[start..end].iter().for_each(|triangle| {
                let (min, max) = extract_min_max_pos(triangle, vertex_positions);
                for i in 0..3 {
                    min_pos[i] = min_pos[i].min(min[i]);
                    max_pos[i] = max_pos[i].max(max[i]);
                }
            });
            BoundingBoxData::new(min_pos, max_pos)
        })
        .collect()
}

fn extract_min_max_pos(
    triangle_data: &TriangleData,
    vertex_positions: &[Vec3Data],
) -> ([f32; 3], [f32; 3]) {
    let v0 = vertex_positions[triangle_data.v0_index as usize].data;
    let v1 = vertex_positions[triangle_data.v1_index as usize].data;
    let v2 = vertex_positions[triangle_data.v2_index as usize].data;
    let mut min_pos = [f32::INFINITY; 3];
    let mut max_pos = [f32::NEG_INFINITY; 3];
    for i in 0..3 {
        min_pos[i] = min_pos[i].min(v0[i]).min(v1[i]).min(v2[i]);
        max_pos[i] = max_pos[i].max(v0[i]).max(v1[i]).max(v2[i]);
    }
    (min_pos, max_pos)
}

/*=============================PARSER_RELATED================================*/

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
