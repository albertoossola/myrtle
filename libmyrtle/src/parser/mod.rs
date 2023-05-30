mod args;
mod primitive;
mod utils;
mod seq;
mod nodedata;
mod mask;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::IResult;
use nom::multi::{many0, separated_list0, separated_list1};
use crate::ast::{DeviceAST, EndpointAST, FlowAST, MachineAST, NodeAST, ProgramAST, StateAST};
use crate::parser::args::parse_arg;
use crate::parser::utils::ws;

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

fn parse_endpoint(i: &str) -> IResult<&str, (String, EndpointAST)> {
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
