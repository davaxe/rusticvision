use std::collections::HashMap;

use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::{
        complete::{self, line_ending, space0},
        is_newline,
    },
    multi::{count, separated_list1},
    sequence::terminated,
    IResult,
};

use crate::data_structures::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum VertexIndexGroup {
    PosNormTex(usize, usize, usize),
    PosNorm(usize, usize),
    PosTex(usize, usize),
    Pos(usize),
}

#[derive(Debug)]
enum TriangleIndexData<'a> {
    Face([VertexIndexGroup; 3]),
    MaterialName(&'a str),
}

/// Parse all the triangle index data in the input. This includes the material
/// name and all triangles.  Input should be a string of the form:
/// ```text
/// usemtl <material_name>
/// f <vertex_index_group> <vertex_index_group> <vertex_index_group>
/// ...
/// usemtl <material_name>
/// f <vertex_index_group> <vertex_index_group> <vertex_index_group>
/// ...
/// ```
/// where `<vertex_index_group>` is of the form: `v/vt/vn` or `v//vn` or `v/vt`
/// or `v`.  where `v` is the position index, `vt` is the texture index and `vn`
/// is the normal index.
///
/// ### Panics
/// When the input is not of the form specified above.  
pub fn parse_triangles<'a>(
    input: &'a str,
    material_map: &'a HashMap<String, usize>,
) -> IResult<&'a str, Vec<TriangleData>> {
    let (input, data) = parse_triangle_index_data(input)?;
    let mut triangle_indices = Vec::new();
    let mut current_material_index = 0;

    data.into_iter().for_each(|d| match d {
        TriangleIndexData::MaterialName(name) => {
            current_material_index = *material_map.get(name).expect("Material not found");
        }
        TriangleIndexData::Face([v1, v2, v3]) => {
            let triangle_index = convert_to_triangle(&v1, &v2, &v3, current_material_index);
            triangle_indices.push(triangle_index);
        }
    });
    Ok((input, triangle_indices))
}

/// Parse an u64 and decrement it by 1, and convert it to usize.
///
/// ### Panics
/// Panics if the parsed u64 is 0.
fn usize_decrement(input: &str) -> IResult<&str, usize> {
    let (input, num) = complete::u32(input)?;
    Ok((input, (num - 1) as usize))
}

/// Parse a face vertex index group of type `a/b/c`, ie position, texture and normal.
fn parse_pos_norm_tex_index(input: &str) -> IResult<&str, VertexIndexGroup> {
    let (input, pos) = terminated(usize_decrement, tag("/"))(input)?;
    let (input, tex) = terminated(usize_decrement, tag("/"))(input)?;
    let (input, norm) = usize_decrement(input)?;
    Ok((input, VertexIndexGroup::PosNormTex(pos, tex, norm)))
}

/// Parse a face vertex index group of type `a//c`, ie only position and normal.
fn parse_pos_norm_index(input: &str) -> IResult<&str, VertexIndexGroup> {
    let (input, pos) = terminated(usize_decrement, tag("//"))(input)?;
    let (input, norm) = usize_decrement(input)?;
    Ok((input, VertexIndexGroup::PosNorm(pos, norm)))
}

/// Parse a face vertex index group of type `a/b`, ie only position and texture.
fn parse_pos_tex_index(input: &str) -> IResult<&str, VertexIndexGroup> {
    // 1/2
    let (input, pos) = terminated(usize_decrement, tag("/"))(input)?;
    let (input, tex) = usize_decrement(input)?;
    Ok((input, VertexIndexGroup::PosTex(pos, tex)))
}

/// Parse a face vertex index group of type `a`, ie only position.
fn parse_pos_index(input: &str) -> IResult<&str, VertexIndexGroup> {
    // 1
    let (input, pos) = usize_decrement(input)?;
    Ok((input, VertexIndexGroup::Pos(pos)))
}

/// Parse any of the supported face vertex index groups.
fn parse_vertex_index_group(input: &str) -> IResult<&str, VertexIndexGroup> {
    alt((
        parse_pos_norm_tex_index,
        parse_pos_norm_index,
        parse_pos_tex_index,
        parse_pos_index,
    ))(input)
}

/// Parse a triangle face of type `f a/b/c d/e/f g/h/i` into TriangleIndexData
fn parse_triangle_face(input: &str) -> IResult<&str, TriangleIndexData> {
    let (input, _) = tag("f ")(input)?;
    let (input, index_group_vec) = count(terminated(parse_vertex_index_group, space0), 3)(input)?;
    let index_group: [VertexIndexGroup; 3] =
        index_group_vec
            .try_into()
            .unwrap_or_else(|v: Vec<VertexIndexGroup>| {
                panic!("Expected a Vec of length 3 but got {}", v.len())
            });
    Ok((input, TriangleIndexData::Face(index_group)))
}

/// Parse a material name of type `usemtl name` into TriangleIndexData.
fn parse_material_name(input: &str) -> IResult<&str, TriangleIndexData> {
    let (input, _) = tag("usemtl ")(input)?;
    let (input, material) = take_till(|c| is_newline(c as u8))(input)?;
    Ok((input, TriangleIndexData::MaterialName(material)))
}

/// Parse a line separated list of faces (triangles) indices and material names.
fn parse_triangle_index_data(input: &str) -> IResult<&str, Vec<TriangleIndexData>> {
    let (input, data) =
        separated_list1(line_ending, alt((parse_triangle_face, parse_material_name)))(input)?;
    Ok((input, data))
}

/// Converts three vertex index groups and material index into a triangle index.
///
/// ### Panics
/// Panics if the three vertex index groups are not of the same type or if the
/// vertex index groups are not valid or supported.
fn convert_to_triangle(
    v1: &VertexIndexGroup,
    v2: &VertexIndexGroup,
    v3: &VertexIndexGroup,
    material_index: usize,
) -> TriangleData {
    use VertexIndexGroup::*;
    let m = material_index;
    match (v1, v2, v3) {
        (PosNormTex(p1, _, n), PosNormTex(p2, _, _), PosNormTex(p3, _, _)) => {
            TriangleData::new(*p1 as u32, *p2 as u32, *p3 as u32, *n as u32, m as u32)
        }
        (PosNorm(p1, n), PosNorm(p2, _), PosNorm(p3, _)) => {
            TriangleData::new(*p1 as u32, *p2 as u32, *p3 as u32, *n as u32, m as u32)
        }
        // All other cases are valid but not currently supported.
        _ => panic!("Invalid vertex index group"),
    }
}
