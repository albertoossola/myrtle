use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
};
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{alphanumeric1, anychar, i32 as parse_i32, multispace0},
    number::complete::{float},
    combinator::{map, opt},
    error::ParseError,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use crate::{
    ast::{
        DeviceAST, EndpointAST, FlowAST, MachineAST, NodeAST, NodeParamAST, ProgramAST, SeqAST,
        StateAST,
    },
    seq::{ConstSeq, RepeatSeq, Seq},
    NodeData, NodeParam,
};

fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_int(i: &str) -> IResult<&str, NodeData> {
    return map(parse_i32, |n| NodeData::Int(n))(i);
}

fn parse_float(i: &str) -> IResult<&str, NodeData> {
    let (i, f) = map(float, |f| NodeData::Float(f))(i)?;
    let (i, _) = tag("f")(i)?;

    return Ok((i, f));
}

fn parse_char(i: &str) -> IResult<&str, NodeData> {
    return delimited(
        tag("'"),
        map(anychar, |c| NodeData::Char(c)),
        tag("'")
    )(i);
}

fn parse_bool(i: &str) -> IResult<&str, NodeData> {
    return map(tag("true").or(tag("false")), |b| {
        NodeData::Bool(b == "true")
    })(i);
}

fn parse_primitive(i: &str) -> IResult<&str, NodeData> {
    return parse_float
        .or(parse_int)
        .or(parse_char)
        .or(parse_bool)
        .parse(i);
}

fn parse_const_seq(i: &str) -> IResult<&str, SeqAST> {
    let (i, value) = parse_primitive(i)?;
    return Ok((i, SeqAST::Const(value)));
}

fn parse_repeat_seq_count(i: &str) -> IResult<&str, i32> {
    let (i, times) = ws(parse_i32)(i)?;
    let (i, _) = ws(tag("*"))(i)?;

    return Ok((i, times));
}

fn parse_repeat_seq(i: &str) -> IResult<&str, SeqAST> {
    let (i, times) = opt(parse_repeat_seq_count)
        .map(|o| o.unwrap_or(1))
        .parse(i)?;

    let (i, _) = ws(tag("["))(i)?;
    let (i, items) = separated_list0(ws(tag(",")), parse_seq)(i)?;
    let (i, _) = ws(tag("]"))(i)?;

    return Ok((i, SeqAST::Repeat(times, items)));
}

fn parse_seq(i: &str) -> IResult<&str, SeqAST> {
    return parse_repeat_seq.or(parse_const_seq).parse(i);
}

fn parse_string(i: &str) -> IResult<&str, NodeParamAST> {
    let (i, content) = ws(delimited(tag("\""), is_not("\""), tag("\"")))(i)?;
    Ok((
        i,
        NodeParamAST::String(content.to_string()),
    ))
}

fn parse_arg_value(i: &str) -> IResult<&str, NodeParamAST> {
    return parse_string
        .or(parse_primitive.map(|p| NodeParamAST::Base(p)))
        .or((parse_seq.map(|s| NodeParamAST::Seq(s))))
        .parse(i);
}

fn parse_arg(i: &str) -> IResult<&str, (String, NodeParamAST)> {
    let (i, (param, value)) =
        separated_pair(ws(alphanumeric1), ws(tag("=")), ws(parse_arg_value))(i)?;

    return Ok((i, (param.to_string(), value)));
}

fn parse_node(i: &str) -> IResult<&str, NodeAST> {
    let (i, kind) = ws(alphanumeric1)(i)?;
    let (i, _) = ws(tag("("))(i)?;
    let (i, args) = separated_list0(ws(tag(",")), parse_arg)(i)
        .map(|(i, args)| (i, BTreeMap::from_iter(args.into_iter())))?;

    let (i, _) = ws(tag(")"))(i)?;

    return Ok((
        i,
        NodeAST {
            kind: String::from(kind),
            args: args,
        },
    ));
}

fn parse_flow(i: &str) -> IResult<&str, FlowAST> {
    let (i, nodes) = separated_list1(ws(tag(">>")), parse_node)(i)?;
    let (i, _) = ws(tag(";"))(i)?;

    return Ok((i, FlowAST { nodes }));
}

fn parse_state(i: &str) -> IResult<&str, StateAST> {
    let (i, _) = ws(tag("state"))(i)?;
    let (i, name) = ws(alphanumeric1)(i)?;
    let (i, _) = ws(tag("{"))(i)?;
    let (i, flows) = many0(parse_flow)(i)?;
    let (i, _) = ws(tag("}"))(i)?;

    return Ok((
        i,
        StateAST {
            name: name.to_string(),
            flows,
        },
    ));
}

pub fn parse_machine(i: &str) -> IResult<&str, MachineAST> {
    let (i, _) = ws(tag("machine"))(i)?;
    let (i, name) = ws(alphanumeric1)(i)?;
    let (i, _) = ws(tag("{"))(i)?;
    let (i, states) = many0(parse_state)(i)?;
    let (i, _) = ws(tag("}"))(i)?;

    return Ok((
        i,
        MachineAST {
            name: String::from(name),
            states: states,
        },
    ));
}

pub fn parse_endpoint(i: &str) -> IResult<&str, (String, EndpointAST)> {
    let (i, varname) = ws(alphanumeric1)(i)?;
    let (i, _) = ws(tag("="))(i)?;
    let (i, kind) = ws(alphanumeric1)(i)?;
    let (i, _) = ws(tag("("))(i)?;
    let (i, args) = separated_list0(ws(tag(",")), parse_arg)(i)
        .map(|(i, args)| (i, BTreeMap::from_iter(args.into_iter())))?;

    let (i, _) = ws(tag(")"))(i)?;
    let (i, _) = ws(tag(";"))(i)?;

    return Ok((
        i,
        (
            String::from(varname),
            EndpointAST {
                kind: String::from(kind),
                args: args,
            },
        ),
    ));
}

pub fn parse_device(i: &str) -> IResult<&str, DeviceAST> {
    let (i, _) = ws(tag("device"))(i)?;
    let (i, _) = ws(tag("{"))(i)?;
    let (i, endpoints) = many0(parse_endpoint)(i)?;
    let (i, _) = ws(tag("}"))(i)?;

    return Ok((
        i,
        DeviceAST {
            endpoints: BTreeMap::from_iter(endpoints.into_iter()),
        },
    ));
}

pub fn parse_program(i: &str) -> IResult<&str, ProgramAST> {
    //Parse the device configuration
    let (i, device) = parse_device(i)?;

    //Parse the machine(s)
    let (i, machine) = parse_machine(i)?;

    Ok((i, ProgramAST { device, machine }))
}
