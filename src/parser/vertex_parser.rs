use crate::data_structures::*;

use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    multi::separated_list1,
    number::complete::float,
    IResult,
};

enum VertexParseResult {
    Position(Vec3Data),
    Normal(Vec3Data),
    TexCoord(Vec3Data),
}

pub struct VertexData {
    pub positions: Vec<Vec3Data>,
    pub normals: Vec<Vec3Data>,
    pub tex_coords: Vec<Vec3Data>,
}

/// Parse line separated list of vertex data into a vector of
/// `VertexParseResult`.  Vertex data includes position, normal and texture
/// coordinate, and each data is formatted in the following way:
/// - Vertex position: `v x y z`
/// - Vertex normal: `vn x y z`
/// - Texture coordinate: `vt x y`
///
/// ### Arguments
/// - `input` - The input string to parse. Should be a line separated list of
/// vertex data in the format specified above. The vertex data can be in any
/// order.
///
/// ### Returns
/// A tuple containing the remaining input string slice and a tuple of vectors
/// of positions and normals. The positions and normals are in the same order as
/// they were in the input string.
pub fn parse_vertex_data(input: &str) -> IResult<&str, VertexData> {
    let (input, parsed_vertex_data) = parse_vertex_data_list(input)?;
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut tex_coords = Vec::new();
    //let mut tex_coords = Vec::new();
    parsed_vertex_data
        .into_iter()
        .for_each(|vertex_data| match vertex_data {
            VertexParseResult::Position(position) => positions.push(position),
            VertexParseResult::Normal(normal) => normals.push(normal),
            VertexParseResult::TexCoord(tex_coord) => tex_coords.push(tex_coord),
        });

    Ok((
        input,
        VertexData {
            positions,
            normals,
            tex_coords,
        },
    ))
}

/// Parses a single vertex. Format: `v x y z`. Te x, y, z values are parsed as floats.
fn parse_vertex_position(input: &str) -> IResult<&str, VertexParseResult> {
    let (input, _) = tag("v ")(input)?;
    let (input, a) = separated_list1(space1, float)(input)?;
    let pos = Vec3Data::new([a[0], a[1], a[2]]);
    Ok((input, VertexParseResult::Position(pos)))
}

/// Parses a single normal vector with format: `vn x y z`. The x, y and z values
/// are parsed as floats and the resulting vector (x, y, z) is also normalized.
fn parse_vertex_normal(input: &str) -> IResult<&str, VertexParseResult> {
    let (input, _) = tag("vn ")(input)?;
    let (input, a) = separated_list1(space1, float)(input)?;
    let normal = Vec3Data::new([a[0], a[1], a[2]]);
    Ok((input, VertexParseResult::Normal(normal)))
}

/// Parses a single texture coordinate. Format: `vt x y`. The x and y values are
/// parsed as floats and should be in the range [0, 1].
fn parse_vertex_texture_coordinate(input: &str) -> IResult<&str, VertexParseResult> {
    let (input, _) = tag("vt ")(input)?;
    let (input, a) = separated_list1(space1, float)(input)?;
    Ok((
        input,
        VertexParseResult::TexCoord(Vec3Data::new([a[0], a[1], 0.0])),
    ))
}

/// Parses any vertex data and returns the parsed result as a `VertexParseResult`.
/// Vertex data includes:
/// - Vertex positions (v x y z)
/// - Vertex normals (vn x y z)
/// - Texture coordinates (vt x y)
fn parse_single_vertex_data(input: &str) -> IResult<&str, VertexParseResult> {
    alt((
        parse_vertex_position,
        parse_vertex_normal,
        parse_vertex_texture_coordinate,
    ))(input)
}

/// Parses a line separated list of vertex data into a vector of `VertexParseResult`.
fn parse_vertex_data_list(input: &str) -> IResult<&str, Vec<VertexParseResult>> {
    separated_list1(line_ending, parse_single_vertex_data)(input)
}
