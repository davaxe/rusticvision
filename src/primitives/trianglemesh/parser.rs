use crate::primitives::Position;

use super::{Normal, TexCoord, TriangleMesh};

use glam::{Vec2, Vec3};

use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::{
        complete::{line_ending, space1},
        is_newline,
    },
    multi::separated_list1,
    number::complete::float,
    sequence::terminated,
    IResult,
};

enum VertexParseResult {
    Position(Position),
    Normal(Normal),
    TexCoord(TexCoord),
}

/// Parses a single vertex. Format: `v x y z`.
fn parse_vertex_position(input: &str) -> IResult<&str, VertexParseResult> {
    let (input, _) = tag("v ")(input)?;
    let (input, a) = separated_list1(space1, float)(input)?;
    Ok((input, VertexParseResult::Position(Vec3::from_slice(&a))))
}

/// Parses a single normal. Format: `vn x y z`.
fn parse_vertex_normal(input: &str) -> IResult<&str, VertexParseResult> {
    let (input, _) = tag("vn ")(input)?;
    let (input, a) = separated_list1(space1, float)(input)?;
    Ok((input, VertexParseResult::Normal(Vec3::from_slice(&a))))
}

/// Parses a single texture coordinate. Format: `vt x y`.
fn parse_vertex_texture_coordinate(input: &str) -> IResult<&str, VertexParseResult> {
    let (input, _) = tag("vt ")(input)?;
    let (input, a) = separated_list1(space1, float)(input)?;
    Ok((input, VertexParseResult::TexCoord(Vec2::from_slice(&a))))
}

fn parse_single_vertex_data(input: &str) -> IResult<&str, VertexParseResult> {
    alt((
        parse_vertex_position,
        parse_vertex_normal,
        parse_vertex_texture_coordinate,
    ))(input)
}

fn parse_vertex_data_list(input: &str) -> IResult<&str, Vec<VertexParseResult>> {
    terminated(
        separated_list1(line_ending, parse_single_vertex_data),
        line_ending,
    )(input)
}

pub fn parse_vertex_data(input: &str) -> IResult<&str, (Vec<Position>, Vec<Normal>)> {
    let (input, parsed_vertex_data) = parse_vertex_data_list(input)?;

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    //let mut tex_coords = Vec::new();
    parsed_vertex_data.into_iter().for_each(|vertex_data| {
        match vertex_data {
            VertexParseResult::Position(position) => positions.push(position),
            VertexParseResult::Normal(normal) => normals.push(normal),
            VertexParseResult::TexCoord(_) => {} //tex_coords.push(tex_coord)
        }
    });

    Ok((input, (positions, normals)))
}
