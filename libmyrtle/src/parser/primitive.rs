use alloc::string::{String, ToString};
use core::cmp::min;
use nom::{IResult, Parser};
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{anychar, digit1};
use nom::combinator::{map, opt};
use nom::number::complete::{float, i32 as parse_i32};
use nom::number::Endianness;
use nom::sequence::delimited;
use crate::NodeData;
use crate::parser::utils::ws;

pub fn parse_int(i: &str) -> IResult<&str, i32> {
    let (i, minus) = opt(tag("-"))(i)?;
    let (i, digits) = digit1(i)?;

    let int : i32 = str::parse(digits).unwrap_or_default();

    let signed = match minus {
        Some(_) => int * -1,
        None => int
    };

    return Ok((i, signed));
}

pub fn parse_float(i: &str) -> IResult<&str, f32> {
    let (i, f) = float(i)?;
    let (i, _) = tag("f")(i)?;

    return Ok((i, f));
}

pub fn parse_char(i: &str) -> IResult<&str, char> {
    return delimited(
        tag("'"),
        anychar,
        tag("'")
    )(i);
}

pub fn parse_bool(i: &str) -> IResult<&str, bool> {
    return map(tag("true").or(tag("false")), |b| b == "true")(i);
}

pub fn parse_string(i: &str) -> IResult<&str, String> {
    let (i, content) = ws(delimited(tag("\""), is_not("\""), tag("\"")))(i)?;
    Ok((i, content.to_string()))
}