use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, i32, multispace0},
    combinator::map,
    error::ParseError,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use crate::{
    nodedata, Behaviour, EmitBehaviour, Machine, Node, NodeBuffer, NodeData, SetVarBehaviour,
    State, TimerBehaviour,
};

fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_int(i: &str) -> IResult<&str, NodeData> {
    return map(i32, |n| NodeData::Int(n))(i);
}

fn parse_char(i: &str) -> IResult<&str, NodeData> {
    return map(anychar, |c| NodeData::Char('c'))(i);
}

fn parse_bool(i: &str) -> IResult<&str, NodeData> {
    return map(tag("true").or(tag("false")), |b| {
        NodeData::Bool(b == "true")
    })(i);
}

fn parse_node_data(i: &str) -> IResult<&str, NodeData> {
    return parse_int.or(parse_char).or(parse_bool).parse(i);
}

fn parse_param(i: &str) -> IResult<&str, (String, NodeData)> {
    let (i, (param, value)) =
        separated_pair(ws(alphanumeric1), ws(tag("=")), ws(parse_node_data))(i)?;

    return Ok((i, (param.to_string(), value)));
}

fn get_behaviour(name: &str) -> Option<Box<dyn Behaviour>> {
    match name {
        "timer" => Some(Box::new(TimerBehaviour {
            interval: 200,
            last_tick: 0,
        })),
        "setvar" => Some(Box::new(SetVarBehaviour::new("led".to_string()))),
        "emit" => Some(Box::new(EmitBehaviour::new(vec![
            NodeData::Int(0),
            NodeData::Int(1),
        ]))),
        _ => None,
    }
}

fn parse_node(i: &str) -> IResult<&str, Node> {
    let (i, kind) = ws(alphanumeric1)(i)?;
    let (i, _) = ws(tag("("))(i)?;
    //let (i, args) = separated_list0(ws(tag(",")), parse_param)(i)?;
    let (i, _) = ws(tag(")"))(i)?;

    //TODO: Handle errors
    let behaviour = get_behaviour(kind).unwrap();

    return Ok((
        i,
        Node {
            in_buf: NodeBuffer {
                data: NodeData::Nil,
            },
            behaviour,
            next: None,
        },
    ));
}

fn parse_state(i: &str) -> IResult<&str, (String, State)> {
    let (i, _) = ws(tag("state"))(i)?;
    let (i, name) = ws(alphanumeric1)(i)?;
    let (i, _) = ws(tag("{"))(i)?;
    let (i, mut flows) = separated_list1(ws(tag(">>")), parse_node)(i)?;
    let (i, _) = ws(tag("}"))(i)?;

    //link the flows together
    let boxed: Vec<Box<Node>> = flows.into_iter().map(|n| Box::new(n)).collect();

    let flow = boxed
        .into_iter()
        .rev()
        .fold(None, |acc, mut x| {
            x.next = acc;
            return Some(x);
        })
        .unwrap();

    return Ok((
        i,
        (
            name.to_string(),
            State {
                flows: vec![*flow],
                vars: BTreeMap::new(),
            },
        ),
    ));
}

pub fn parse_machine(i: &str) -> IResult<&str, Machine> {
    let (i, _) = ws(tag("machine"))(i)?;
    let (i, name) = ws(alphanumeric1)(i)?;
    let (i, _) = ws(tag("{"))(i)?;
    let (i, states) = many0(parse_state)(i)?;
    let (i, _) = ws(tag("}"))(i)?;

    return Ok((
        i,
        Machine {
            states: BTreeMap::from_iter(states),
            variables: BTreeMap::new(),
            cur_state: String::from("entry"),
        },
    ));
}
