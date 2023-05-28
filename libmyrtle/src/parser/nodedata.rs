use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::number::complete::float;
use crate::NodeData;
use crate::parser::primitive::{parse_bool, parse_char, parse_float, parse_int};

pub fn parse_int_nodedata(i: &str) -> IResult<&str, NodeData> {
    parse_int.map(|i| NodeData::Int(i)).parse(i)
}

pub fn parse_float_nodedata(i: &str) -> IResult<&str, NodeData> {
    parse_float.map(|f| NodeData::Float(f)).parse(i)
}

pub fn parse_char_nodedata(i: &str) -> IResult<&str, NodeData> {
    parse_char.map(|c| NodeData::Char(c)).parse(i)
}

pub fn parse_bool_nodedata(i: &str) -> IResult<&str, NodeData> {
    parse_bool.map(|b| NodeData::Bool(b)).parse(i)
}


pub fn parse_nodedata(i: &str) -> IResult<&str, NodeData> {
    parse_float_nodedata
        .or(parse_int_nodedata)
        .or(parse_char_nodedata)
        .or(parse_bool_nodedata)
        .parse(i)
}