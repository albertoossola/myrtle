use alloc::{boxed::Box, collections::BTreeMap, string::String, vec, vec::Vec};

use crate::nodes::*;
use crate::seq::{ByteSeq, ChainSeq, ConstSeq, RepeatSeq, Seq};
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
        states: BTreeMap::from_iter(states)
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
        SeqAST::Chain(inner) => {
            Box::new(ChainSeq::new(inner.iter().map(|i| make_seq(i)).collect()))
        }
        SeqAST::Repeat(times, inner) => {
            let inner_seq = make_seq(inner);
            Box::new(RepeatSeq::new(*times, inner_seq))
        }
        SeqAST::Byte => Box::new(ByteSeq::new()),
    };
}

fn make_param(ast: &NodeArgAST) -> NodeArg {
    match ast {
        NodeArgAST::Base(data) => NodeArg::Base(*data),
        NodeArgAST::String(str) => NodeArg::String(str.clone()),
        NodeArgAST::Seq(seq_ast) => NodeArg::Seq(make_seq(&seq_ast)),
    }
}

fn make_node(ast: &mut NodeAST) -> Result<Node, ErrorCode> {
    let mut behaviour: Box<dyn Behaviour> = match ast.kind.as_str() {
        "once" => Box::new(OnceBehaviour::new()),
        "timer" => Box::new(TimerBehaviour::new(500)),
        "emit" => Box::new(EmitBehaviour::new(Box::new(ChainSeq::new(vec![])))),
        "stream" => Box::new(StreamBehaviour::new(Box::new(ChainSeq::new(vec![])))),
        "literal" => Box::new(LiteralBehaviour::new()),
        "setvar" => Box::new(SetVarBehaviour::new(String::from(""))),
        "setstate" => Box::new(SetStateBehaviour::new(String::from("entry"))),
        "delay" => Box::new(DelayBehaviour::new()),
        "debounce" => Box::new(DebounceBehaviour::new()),
        "watchvar" => Box::new(WatchVarBehaviour::new()),
        "ease" => Box::new(EaseBehaviour::new()),
        "equals" => Box::new(EqualsBehaviour::new()),
        "mask" => Box::new(MaskBehaviour::new()),
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
    let mut args: BTreeMap<String, NodeArg> = ast
        .args
        .iter()
        .map(|(k, v)| (k.clone(), make_param(&v)))
        .collect();

    match ast.kind.as_str() {
        "out" => {
            let led_num = args.remove("pin").ok_or(ErrorCode::ArgumentRequired)?;

            match led_num {
                NodeArg::Base(crate::NodeData::Int(num)) => {
                    Ok(Symbol::new(adapter.set_push_pull_pin(num)))
                }
                _ => Err(ErrorCode::InvalidArgumentType),
            }
        }
        "in" => {
            let led_num = args.remove("pin").ok_or(ErrorCode::ArgumentRequired)?;

            match led_num {
                NodeArg::Base(crate::NodeData::Int(num)) => {
                    Ok(Symbol::new(adapter.set_input_pin(num)))
                }
                _ => Err(ErrorCode::InvalidArgumentType),
            }
        }
        "pwm" => {
            let channel = args.remove("channel").ok_or(ErrorCode::ArgumentRequired)?;

            match channel {
                NodeArg::Base(crate::NodeData::Int(num)) => {
                    Ok(Symbol::new(adapter.set_pwm_pin(num)))
                }
                _ => Err(ErrorCode::InvalidArgumentType),
            }
        }
        "uart" => {
            let baud = args.remove("baud").ok_or(ErrorCode::ArgumentRequired)?;

            match baud {
                NodeArg::Base(crate::NodeData::Int(baud_i32)) => {
                    Ok(Symbol::new(adapter.set_uart(14, 15, baud_i32)))
                }
                _ => Err(ErrorCode::InvalidArgumentType),
            }
        },
        "i2c" => {
            let sda_pin = args.remove("sda").ok_or(ErrorCode::ArgumentRequired)?;
            let scl_pin = args.remove("scl").ok_or(ErrorCode::ArgumentRequired)?;

            match (sda_pin, scl_pin) {
                (NodeArg::Base(NodeData::Int(sda)), NodeArg::Base(NodeData::Int(scl))) => {
                    Ok(Symbol::new(adapter.set_i2c(sda, scl)))
                },
                _ => Err(ErrorCode::InvalidArgumentType)
            }
        },
        _ => Err(ErrorCode::UnknownNodeKind),
    }
}
