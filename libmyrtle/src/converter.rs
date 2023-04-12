use alloc::{boxed::Box, collections::BTreeMap, string::String, vec, vec::Vec};

use crate::{
    ast::{FlowAST, MachineAST, NodeAST, StateAST},
    Behaviour, EmitBehaviour, ErrorCode, Machine, Node, SetVarBehaviour, State, TimerBehaviour,
};

pub fn make_machine(ast: &mut MachineAST) -> Result<Machine, ErrorCode> {
    let states_result: Result<Vec<(String, State)>, ErrorCode> = ast
        .states
        .iter_mut()
        .map(|s_ast| make_state(s_ast).map(|s| ((s_ast.name.clone(), s))))
        .collect();

    let states = states_result?;

    let machine = Machine {
        cur_state: String::from("entry"),
        variables: BTreeMap::new(),
        states: BTreeMap::from_iter(states),
    };

    return Ok(machine);
}

fn make_state(ast: &mut StateAST) -> Result<State, ErrorCode> {
    let flows_result: Result<Vec<Box<Node>>, ErrorCode> = ast
        .flows
        .iter_mut()
        .map(|f_tree| make_flow(f_tree))
        .collect();

    //TODO(?): keep a box<node> reference in state
    let flows = flows_result?.into_iter().map(|f| *f).collect();

    let state = State {
        vars: BTreeMap::new(),
        flows,
    };

    return Ok(state);
}

fn make_flow(ast: &mut FlowAST) -> Result<Box<Node>, ErrorCode> {
    let flow_res: Result<Vec<Node>, ErrorCode> =
        ast.nodes.iter_mut().map(|n_ast| make_node(n_ast)).collect();

    let flow = flow_res?;

    let n = flow.into_iter().rev().fold(None, |a, mut n| {
        n.next = a;
        return Some(Box::new(n));
    });

    return Ok(n.unwrap());
}

fn make_node(ast: &mut NodeAST) -> Result<Node, ErrorCode> {
    let mut behaviour: Box<dyn Behaviour> = match ast.kind.as_str() {
        "timer" => Box::new(TimerBehaviour::new(500)),
        "emit" => Box::new(EmitBehaviour::new(vec![])),
        "setvar" => Box::new(SetVarBehaviour::new(String::from(""))),
        _ => Err(ErrorCode::UnknownNodeKind)?,
    };

    behaviour.init(&mut ast.args)?;

    let node = Node {
        behaviour,
        in_buf: crate::NodeBuffer {
            data: crate::NodeData::Nil,
        },
        next: None,
    };

    return Ok(node);
}
