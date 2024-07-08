// metadata.rs
// Copyright 2024 Patrick Meade.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//---------------------------------------------------------------------------

// This is a nom parser that I hacked together to parse the metadata format
// that appears in DreamMaker Icon (.dmi) files. It was tested on many icons,
// in several SpaceStation 13 code bases, including a few icons that looked
// suspiciously like the metadata was hand-edited at some point.
//
// This parser does not conform to an official standard. It should be good
// to handle most Version 4 icons that you'll find in the wild.
//
// Some of the fields are just lazy parses into Vec<String>; I didn't need
// those fields for my purpose. If you care about the field and improve
// the code, I am happy to accept a pull request on GitHub.

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{digit1, multispace0},
    combinator::{fail, success},
    error::ParseError,
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::error::{IconToolError::IncompleteParseError, Result};

#[derive(Debug)]
pub struct DreamMakerIconMetadata {
    pub version: String,
    pub width: u32,
    pub height: u32,
    pub states: Vec<DreamMakerIconState>,
}

#[derive(Debug)]
pub struct DreamMakerIconState {
    pub name: String,
    pub delay: Option<Vec<String>>,
    pub dirs: u32,
    pub frames: u32,
    pub hotspot: Option<Vec<String>>,
    pub _loop: Option<String>, // 'loop' is a Rust keyword
    pub movement: Option<String>,
    pub rewind: Option<String>,
}

#[derive(Debug)]
struct DreamMakerIconStateProperty {
    name: String,
    value: String,
}

pub fn parse_metadata(input: &str) -> Result<DreamMakerIconMetadata> {
    // parse the provided metadata
    let (input, dmi_metadata) = nomify_metadata(input)?;
    // if we didn't parse all of the provided input
    if !input.is_empty() {
        // you get to drink from the firehose...
        return Err(IncompleteParseError(String::from(input)));
    }
    // return the parse tree to the caller
    Ok(dmi_metadata)
}

fn nomify_metadata(input: &str) -> IResult<&str, DreamMakerIconMetadata> {
    let (input, _) = ws(tag("# BEGIN DMI"))(input)?;
    let (input, version) = parse_version(input)?;
    let (input, width) = parse_optional_width(input)?;
    let (input, height) = parse_optional_height(input)?;
    let (input, states) = parse_states(input)?;
    let (input, _) = ws(tag("# END DMI"))(input)?;

    Ok((
        input,
        DreamMakerIconMetadata {
            version,
            width,
            height,
            states,
        },
    ))
}

fn parse_version(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("version = ")(input)?;
    let (input, (major_version, _, minor_version)) = tuple((digit1, tag("."), digit1))(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, format!("{}.{}", major_version, minor_version)))
}

fn parse_optional_width(input: &str) -> IResult<&str, u32> {
    let (input, width) = alt((parse_width, success(32)))(input)?;
    Ok((input, width))
}

fn parse_optional_height(input: &str) -> IResult<&str, u32> {
    let (input, height) = alt((parse_height, success(32)))(input)?;
    Ok((input, height))
}

fn parse_width(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("\twidth = ")(input)?;
    let (input, width) = digit1(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, width.parse::<u32>().unwrap()))
}

fn parse_height(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("\theight = ")(input)?;
    let (input, height) = digit1(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, height.parse::<u32>().unwrap()))
}

fn parse_states(input: &str) -> IResult<&str, Vec<DreamMakerIconState>> {
    let (input, states) = many0(parse_state)(input)?;
    Ok((input, states))
}

fn parse_state(input: &str) -> IResult<&str, DreamMakerIconState> {
    let (input, name) = parse_state_name(input)?;

    let mut delay: Option<Vec<String>> = None;
    let mut dirs: Option<u32> = None;
    let mut frames: Option<u32> = None;
    let mut hotspot: Option<Vec<String>> = None;
    let mut _loop: Option<String> = None;
    let mut movement: Option<String> = None;
    let mut rewind: Option<String> = None;

    let (input, props) = parse_state_properties(input)?;

    for prop in props {
        match prop.name.as_str() {
            // delay = 8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8
            "delay" => {
                delay = Some(prop.value.split(",").map(|x| x.to_string()).collect());
            }
            // dirs = 4
            "dirs" => {
                dirs = Some(prop.value.parse::<u32>().unwrap());
            }
            // frames = 1
            "frames" => {
                frames = Some(prop.value.parse::<u32>().unwrap());
            }
            // hotspot = 8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8
            "hotspot" => {
                hotspot = Some(prop.value.split(",").map(|x| x.to_string()).collect());
            }
            // loop = 1
            "loop" => {
                _loop = Some(prop.value.clone());
            }
            // movement = 1
            "movement" => {
                movement = Some(prop.value.clone());
            }
            // rewind = 1
            "rewind" => {
                rewind = Some(prop.value.clone());
            }
            // this is an unknown property keyword
            _ => {
                return fail(input);
            }
        }
    }

    Ok((
        input,
        DreamMakerIconState {
            name,
            delay,
            dirs: dirs.unwrap(),
            frames: frames.unwrap(),
            hotspot,
            _loop,
            movement,
            rewind,
        },
    ))
}

fn parse_state_name(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("state = ")(input)?;
    let (input, name) = parse_quoted_string(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, name))
}

//------------------------------------------------------------------------------------------------------------------------
// See: https://users.rust-lang.org/t/solved-nom5-parse-a-string-containing-escaped-quotes-and-delimited-by-quotes/32818/2
//------------------------------------------------------------------------------------------------------------------------
fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    let qs = preceded(tag("\""), in_quotes);
    terminated(qs, tag("\""))(input)
}

fn in_quotes(input: &str) -> IResult<&str, String> {
    let mut ret = String::new();
    let mut skip_delimiter = false;
    for (i, ch) in input.char_indices() {
        if ch == '\\' && !skip_delimiter {
            skip_delimiter = true;
        } else if ch == '"' && !skip_delimiter {
            return Ok((&input[i..], ret));
        } else {
            ret.push(ch);
            skip_delimiter = false;
        }
    }
    Err(nom::Err::Incomplete(nom::Needed::Unknown))
}
//------------------------------------------------------------------------------------------------------------------------

fn parse_state_properties(input: &str) -> IResult<&str, Vec<DreamMakerIconStateProperty>> {
    let (input, props) = many0(parse_state_property)(input)?;
    Ok((input, props))
}

fn parse_state_property(input: &str) -> IResult<&str, DreamMakerIconStateProperty> {
    let (input, _) = tag("\t")(input)?;
    let (input, (name, _, value)) =
        tuple((parse_property_name, tag(" = "), parse_property_value))(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, DreamMakerIconStateProperty { name, value }))
}

fn parse_property_name(input: &str) -> IResult<&str, String> {
    let (input, name) = is_not(" ")(input)?;
    Ok((input, String::from(name)))
}

fn parse_property_value(input: &str) -> IResult<&str, String> {
    let (input, value) = is_not("\n")(input)?;
    Ok((input, String::from(value)))
}

//------------------------------------------------------------------------------------------------------------------------
// See: https://docs.rs/nom/latest/nom/recipes/index.html#whitespace
//------------------------------------------------------------------------------------------------------------------------
// fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
//     inner: F,
// ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
// where
//     F: Fn(&'a str) -> IResult<&'a str, O, E>,
// {
//     delimited(multispace0, inner, multispace0)
// }
//------------------------------------------------------------------------------------------------------------------------
fn ws<'a, F, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E> + 'a,
    E: ParseError<&'a str>,
{
    delimited(multispace0, inner, multispace0)
}
//------------------------------------------------------------------------------------------------------------------------
