use std::collections::HashMap;

use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{digit0, line_ending, space1},
    multi::separated_list1,
    number::complete::float,
    IResult,
};

use glam::Vec3A;

use super::Material;

enum MaterialProperty {
    AmbientColor(Vec3A),
    DiffuseColor(Vec3A),
    SpecularColor(Vec3A),
    EmissiveColor(Vec3A),
    SpecularHighlight(f32),
    Transparency(f32),
    IndexOfRefraction(f32),
    IlluminationModel(u32),
}

/// Parses multiple materials from string slice using [MTL file
/// format](https://en.wikipedia.org/wiki/Wavefront_.obj_file).
///
/// ### Arguments
/// * `input` - The string slice to parse. This should be the contents of a MTL
/// file.
///
/// ### Returns
/// If successful, returns a tuple containing a vector of materials and a
/// hashmap mapping material names to their indices in the vector. The map can
/// be used to look up a material index by name.
pub fn materials(input: &str) -> IResult<&str, (Vec<Material>, HashMap<String, usize>)> {
    let mut materials = Vec::new();
    let mut material_index_map = HashMap::new();
    let mut input = input;
    loop {
        if input.starts_with("newmtl ") {
            let (remaining, (name, matprop)) = parse_material(input)?;
            material_index_map.insert(name.to_string(), materials.len());
            materials.push(material_from_properties(&matprop));
            input = remaining;
        }
        if input.is_empty() {
            break;
        }
        let (remaining, _) = take_till(|c| c == '\n')(input)?;
        let (remaining, _) = line_ending(remaining)?;
        input = remaining;
    }
    Ok((input, (materials, material_index_map)))
}

/// Constructs a material from a list of material properties. Missing properties
/// are set to their default values.
fn material_from_properties(properties: &Vec<MaterialProperty>) -> Material {
    use MaterialProperty::*;
    let mut material = Material::default();
    for prop in properties {
        match &prop {
            AmbientColor(c) => material.ambient_color = *c,
            DiffuseColor(c) => material.diffuse_color = *c,
            SpecularColor(c) => material.specular_color = *c,
            EmissiveColor(c) => material.emissive_color = *c,
            SpecularHighlight(h) => material.specular_highlight = *h,
            Transparency(t) => material.transparency = *t,
            IndexOfRefraction(i) => material.index_of_refraction = *i,
            IlluminationModel(_m) => (),
        }
    }
    material
}

fn parse_material(input: &str) -> IResult<&str, (&str, Vec<MaterialProperty>)> {
    let (input, material_name) = parse_material_name(input)?;
    let (input, _) = line_ending(input)?;
    let (input, material_properties) = separated_list1(line_ending, parse_material_property)(input)?;
    Ok((input, (material_name, material_properties)))
}

/// Parse material name.
fn parse_material_name(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("newmtl ")(input)?;
    let (input, name) = take_till(|c| c == '\n')(input)?;
    Ok((input, name))
}

/// Pase ambient color property line.
fn parse_ambient_color(input: &str) -> IResult<&str, MaterialProperty> {
    let (input, _) = tag("Ka ")(input)?;
    let (input, a) = separated_list1(space1, float)(input)?;
    Ok((input, MaterialProperty::AmbientColor(Vec3A::from_slice(&a))))
}

/// Parse diffuse color property.
fn parse_diffuse_color(input: &str) -> IResult<&str, MaterialProperty> {
    let (input, _) = tag("Kd ")(input)?;
    let (input, a) = separated_list1(space1, float)(input)?;
    Ok((input, MaterialProperty::DiffuseColor(Vec3A::from_slice(&a))))
}

/// Parse specular color property.
fn parse_specular_color(input: &str) -> IResult<&str, MaterialProperty> {
    let (input, _) = tag("Ks ")(input)?;
    let (input, a) = separated_list1(space1, float)(input)?;
    Ok((input, MaterialProperty::SpecularColor(Vec3A::from_slice(&a))))
}

/// Parse specular highlight property.
fn parse_specular_highlight(input: &str) -> IResult<&str, MaterialProperty> {
    let (input, _) = tag("Ns ")(input)?;
    let (input, a) = float(input)?;
    Ok((input, MaterialProperty::SpecularHighlight(a)))
}

/// Parse emissive color property.
fn parse_emissive_color(input: &str) -> IResult<&str, MaterialProperty> {
    let (input, _) = tag("Ke ")(input)?;
    let (input, a) = separated_list1(space1, float)(input)?;
    Ok((input, MaterialProperty::EmissiveColor(Vec3A::from_slice(&a))))
}

/// Parse transparency property.
fn parse_transparency(input: &str) -> IResult<&str, MaterialProperty> {
    let (input, _) = tag("d ")(input)?;
    let (input, a) = float(input)?;
    Ok((input, MaterialProperty::Transparency(a)))
}

/// Parse index of refraction property.
fn parse_index_of_refraction(input: &str) -> IResult<&str, MaterialProperty> {
    let (input, _) = tag("Ni ")(input)?;
    let (input, a) = float(input)?;
    Ok((input, MaterialProperty::IndexOfRefraction(a)))
}

/// Parse illumination model property.
fn parse_illumination_model(input: &str) -> IResult<&str, MaterialProperty> {
    let (input, _) = tag("illum ")(input)?;
    let (input, a) = digit0(input)?;
    Ok((
        input,
        MaterialProperty::IlluminationModel(a.parse::<u32>().unwrap()),
    ))
}

/// Parse any material property.
fn parse_material_property(input: &str) -> IResult<&str, MaterialProperty> {
    alt((
        parse_ambient_color,
        parse_diffuse_color,
        parse_specular_color,
        parse_specular_highlight,
        parse_emissive_color,
        parse_transparency,
        parse_index_of_refraction,
        parse_illumination_model,
    ))(input)
}
