use alloc::boxed::Box;
use nom::combinator::opt;
use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::number::complete::{ i32 as parse_i32 };
use crate::ast::SeqAST;
use crate::parser::nodedata::parse_nodedata;
use crate::parser::primitive::parse_int;
use crate::parser::utils::ws;

pub fn parse_byte_seq(i: &str) -> IResult<&str, SeqAST> {
    let (i, value) = tag("u8")(i)?;

    return Ok((i, SeqAST::Byte));
}

pub fn parse_const_seq(i: &str) -> IResult<&str, SeqAST> {
    let (i, value) = parse_nodedata(i)?;
    return Ok((i, SeqAST::Const(value)));
}

pub fn parse_repeat_seq(i: &str) -> IResult<&str, SeqAST> {
    let (i, times) = ws(parse_int)(i)?;
    let (i, _) = ws(tag("*"))(i)?;
    let (i, inner) = ws(parse_seq)(i)?;

    return Ok((i, SeqAST::Repeat(times, Box::new(inner))));
}

pub fn parse_chain_seq(i: &str) -> IResult<&str, SeqAST> {
    let (i, _) = ws(tag("["))(i)?;
    let (i, items) = separated_list0(ws(tag(",")), parse_seq)(i)?;
    let (i, _) = ws(tag("]"))(i)?;

    return Ok((i, SeqAST::Chain(items)));
}

pub fn parse_seq(i: &str) -> IResult<&str, SeqAST> {
    return parse_chain_seq
        .or(parse_repeat_seq)
        .or(parse_const_seq)
        .or(parse_byte_seq)
        .parse(i);
}