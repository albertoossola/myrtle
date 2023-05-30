use alloc::string::{String, ToString};
use nom::{IResult, Parser};
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::alphanumeric1;
use nom::combinator::map;
use nom::sequence::{delimited, separated_pair};
use crate::ast::NodeArgAST;
use crate::parser::nodedata::parse_nodedata;
use crate::parser::primitive::parse_string;
use crate::parser::seq::{parse_chain_seq};
use crate::parser::utils::ws;


pub fn parse_string_arg(i: &str) -> IResult<&str, NodeArgAST> {
    return parse_string.map(|s| NodeArgAST::String(s)).parse(i);
}

pub fn parse_arg_value(i: &str) -> IResult<&str, NodeArgAST> {
    return parse_string_arg
        .or(parse_nodedata.map(|p| NodeArgAST::Base(p)))
        .or(parse_chain_seq.map(|s| NodeArgAST::Seq(s)))
        .parse(i);
}

pub fn parse_arg(i: &str) -> IResult<&str, (String, NodeArgAST)> {
    let (i, (param, value)) =
        separated_pair(ws(alphanumeric1), ws(tag("=")), ws(parse_arg_value))(i)?;

    return Ok((i, (param.to_string(), value)));
}