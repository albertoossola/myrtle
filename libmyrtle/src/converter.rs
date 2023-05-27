use alloc::{boxed::Box, collections::BTreeMap, string::String, vec, vec::Vec};

use crate::nodes::*;
use crate::seq::{ConstSeq, DelimitedSeq, RepeatSeq, Seq};
use crate::{ast::*, *};

pub fn make_program(
    adapter: &mut dyn HWAdapter,
    ast: &mut ProgramAST,
) -> Result<Machine, ErrorCode> {
    let mut machine = make_machine(&mut ast.machine)?;

    for (key, mut endpoint) in ast.device.endpoints.iter_mut() {
        let symbol = make_endpoint(adapter, &mut endpoint)?;

        machine.variables.insert(key.clone(), symbol);
    }

    return Ok(machine);
}

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

fn make_seq(ast: &SeqAST) -> Box<dyn Seq> {
    return match ast {
        SeqAST::Const(value) => Box::new(ConstSeq::new(*value)),
        SeqAST::Repeat(times, inner_asts) => {
            let inner_seqs = inner_asts.iter().map(|s| make_seq(s)).collect();
            Box::new(RepeatSeq::new(*times, inner_seqs))
        }
    };
}

fn make_param(ast: &NodeParamAST) -> NodeParam {
    match ast {
        NodeParamAST::Base(data) => NodeParam::Base(*data),
        NodeParamAST::String(str) => NodeParam::String(str.clone()),
        NodeParamAST::Seq(seq_ast) => NodeParam::Seq(make_seq(&seq_ast)),
    }
}

fn make_node(ast: &mut NodeAST) -> Result<Node, ErrorCode> {
    let mut behaviour: Box<dyn Behaviour> = match ast.kind.as_str() {
        "timer" => Box::new(TimerBehaviour::new(500)),
        "emit" => Box::new(EmitBehaviour::new(Box::new(RepeatSeq::new(0, vec![])))),
        "stream" => Box::new(StreamBehaviour::new(Box::new(RepeatSeq::new(0, vec![])))),
        "literal" => Box::new(LiteralBehaviour::new()),
        "setvar" => Box::new(SetVarBehaviour::new(String::from(""))),
        "delay" => Box::new(DelayBehaviour::new()),
        "debounce" => Box::new(DebounceBehaviour::new()),
        "watchvar" => Box::new(WatchVarBehaviour::new()),
        "ease" => Box::new(EaseBehaviour::new()),
        _ => Err(ErrorCode::UnknownNodeKind)?,
    };

    let mut args = ast
        .args
        .iter()
        .map(|(k, v)| (k.clone(), make_param(&v)))
        .collect();

    behaviour.init(&mut args)?;

    let node = Node {
        behaviour,
        in_buf: crate::NodeBuffer {
            data: crate::NodeData::Nil,
        },
        next: None,
    };

    return Ok(node);
}

fn make_endpoint(adapter: &mut dyn HWAdapter, ast: &mut EndpointAST) -> Result<Symbol, ErrorCode> {
    let mut args: BTreeMap<String, NodeParam> = ast
        .args
        .iter()
        .map(|(k, v)| (k.clone(), make_param(&v)))
        .collect();

    match ast.kind.as_str() {
        "out" => {
            let led_num = args.remove("pin").ok_or(ErrorCode::ArgumentRequired)?;

            match led_num {
                NodeParam::Base(crate::NodeData::Int(num)) => {
                    Ok(Symbol::new(adapter.set_push_pull_pin(num)))
                }
                _ => Err(ErrorCode::InvalidArgumentType),
            }
        }
        "in" => {
            let led_num = args.remove("pin").ok_or(ErrorCode::ArgumentRequired)?;

            match led_num {
                NodeParam::Base(crate::NodeData::Int(num)) => {
                    Ok(Symbol::new(adapter.set_input_pin(num)))
                }
                _ => Err(ErrorCode::InvalidArgumentType),
            }
        },
        "pwm" => {
            let channel = args.remove("channel").ok_or(ErrorCode::ArgumentRequired)?;

            match channel {
                NodeParam::Base(crate::NodeData::Int(num)) => {
                    Ok(Symbol::new(adapter.set_pwm_pin(num)))
                }
                _ => Err(ErrorCode::InvalidArgumentType),
            }
        },
        _ => Err(ErrorCode::UnknownNodeKind),
    }
}
