use std::collections::HashMap;

use crate::primitives::TriangleIndex;

use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::{
        complete::{self, line_ending},
        is_newline,
    },
    multi::count,
    sequence::preceded,
    IResult, Parser,
};

pub fn parse_triangle_indices<'a>(
    input: &'a str,
    material_map: &'a HashMap<String, usize>,
) -> IResult<&'a str, Vec<TriangleIndex>> {
    let mut triangle_indices = Vec::new();
    let mut material_index = 0;
    let mut input = input;
    loop {
        if input.starts_with("usemtl ") {
            let (remaining, material_name) =
                preceded(tag("usemtl "), take_till(|c| is_newline(c as u8)))(input)?;
            material_index = *material_map.get(material_name).expect("Material not found");
            input = remaining;
        } else if input.starts_with("f ") {
            let (remaining, triangle_index) = parse_triangle_index(&input[2..], material_index)?;
            input = remaining;
            triangle_indices.push(triangle_index);
        }

        if input.is_empty() {
            break;
        }
        let (remaining, _) = line_ending(input)?;
        input = remaining;
    }
    Ok((input, triangle_indices))
}

fn parse_triangle_index(input: &str, material_index: usize) -> IResult<&str, TriangleIndex> {
    let (input, indices) = count(index_group, 3)(input)?;
    let vertex_position_indices = (
        indices[0][0].expect("Vertex position index is required") - 1,
        indices[1][0].expect("Vertex position index is required") - 1,
        indices[2][0].expect("Vertex position index is required") - 1,
    );
    let normal_index = indices[0][2].expect("Vertex normal index is required") - 1;
    // let vertex_texture_index = (
    //     indices[0][1].map(|a| a - 1),
    //     indices[1][1].map(|a| a - 1),
    //     indices[2][1].map(|a| a - 1),
    // );
    Ok((
        input,
        TriangleIndex::new(vertex_position_indices, normal_index, material_index),
    ))
}

fn index_group(input: &str) -> IResult<&str, [Option<usize>; 3]> {
    let input = input.trim_start();
    let (input, index_group_vec) = count(slash_sep_number, 3)(input)?;
    let mut index_group = [None; 3];
    for (i, index) in index_group_vec.into_iter().enumerate() {
        index_group[i] = index;
    }
    Ok((input, index_group))
}

fn slash_sep_number(input: &str) -> IResult<&str, Option<usize>> {
    let (mut input, number) = alt((
        tag("/").map(|_| None),
        complete::u64.map(|a| Some(a as usize)),
    ))(input)?;
    if number.is_some() && input.starts_with('/') {
        input = &input[1..];
    }
    Ok((input, number))
}
