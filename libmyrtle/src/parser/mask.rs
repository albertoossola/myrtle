use nom::combinator::opt;
use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::number::complete::{ i32 as parse_i32 };
use crate::ast::{MaskAST, SeqAST};
use crate::parser::nodedata::parse_nodedata;
use crate::parser::primitive::parse_int;
use crate::parser::utils::ws;

pub fn parse_const_mask(i: &str) -> IResult<&str, MaskAST> {
    let (i, value) = parse_nodedata(i)?;
    return Ok((i, MaskAST::Const(value)));
}

pub fn parse_chain_mask(i: &str) -> IResult<&str, MaskAST> {
    let (i, _) = ws(tag("["))(i)?;
    let (i, items) = separated_list0(ws(tag(",")), parse_mask)(i)?;
    let (i, _) = ws(tag("]"))(i)?;

    return Ok((i, MaskAST::Chain(items)));
}

pub fn parse_mask(i: &str) -> IResult<&str, MaskAST> {
    return parse_chain_mask.or(parse_const_mask).parse(i);
}