use core::panic;
use std::collections::HashMap;

use crate::primitives::{
    material,
    trianglemesh::{self, TriangleMesh},
    Material, TriangleIndex,
};
use crate::scene::object::Object;

use glam::Vec3;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::{complete::line_ending, is_newline},
    multi::separated_list1,
    sequence::{pair, preceded, terminated},
    IResult,
};

use super::{object, Scene};

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

fn line(input: &str) -> IResult<&str, &str> {
    let (input, line) = take_till(|c| is_newline(c as u8))(input)?;
    Ok((input, line))
}

fn parse_vertex_position(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("v "), line)(input)?;
    Ok((input, ParseLineResult::VertexPosition(pos)))
}

fn parse_vertex_normal(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("vn "), line)(input)?;
    Ok((input, ParseLineResult::VertexNormal(pos)))
}

fn parse_vertex_texture(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("vt "), line)(input)?;
    Ok((input, ParseLineResult::VertexTexture(pos)))
}

fn parse_object_name(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("o "), line)(input)?;
    Ok((input, ParseLineResult::ObjectName(pos)))
}

fn parse_use_material(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("usemtl "), line)(input)?;
    Ok((input, ParseLineResult::UseMaterial(pos)))
}

fn parse_face(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("f "), line)(input)?;
    Ok((input, ParseLineResult::Face(pos)))
}

fn parse_material_file(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("mtllib "), line)(input)?;
    Ok((input, ParseLineResult::MaterialFile(pos)))
}

fn parse_comment(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = preceded(tag("#"), line)(input)?;
    Ok((input, ParseLineResult::Comment(pos)))
}

fn parse_other(input: &str) -> IResult<&str, ParseLineResult> {
    let (input, pos) = line(input)?;
    Ok((input, ParseLineResult::Other(pos)))
}

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

fn get_vertex_data(
    input: &str,
) -> IResult<&str, (Vec<Vec3>, Vec<Vec3>, String, HashMap<String, String>)> {
    let (input, (vertex_data_str, material_file_str, object_map)) = extract_parts_obj(input)?;

    if let Ok((_, (vp, vn))) = trianglemesh::parser::parse_vertex_data(&vertex_data_str) {
        Ok((input, (vp, vn, material_file_str, object_map)))
    } else {
        panic!("Failed to parse vertex data");
    }
}

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
    let (_, (vp, vn, material_file, object_map)) = get_vertex_data(&obj_file).unwrap();

    let mat_path = format!("{}/{}", directory, material_file);
    let mat_file =
        std::fs::read_to_string(mat_path.trim_end()).expect("PARSE_OBJ: Unable to read material file");
    let (_, (materials, material_map)) = material::parser::materials(&mat_file).unwrap();

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
        let (_, indices) = object::parser::parse_triangle_indices(faces_str, material_map)
            .expect("PARSE: Failed to parse triangle faces");
        let object = Object::new(name.to_owned(), indices, triangle_mesh);
        objects.push(object);
    }
    objects
}