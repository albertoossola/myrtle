use chumsky::combinator::Ignored;
use chumsky::recursive::{Recursive, recursive};
use chumsky::text::TextParser;
use chumsky::{
    combinator::SeparatedBy,
    prelude::Simple,
    primitive::{filter, just},
    text::{keyword, whitespace},
    *,
};

use crate::runtime::*;

struct MyrtleParser;

pub fn parser() -> impl Parser<char, Node, Error = Simple<char>> {
    let identifier = filter(|c: &char| c.is_alphanumeric() || "-_".contains(*c))
        .repeated()
        .padded()
        .collect::<String>()
        .boxed();

    //TODO: Make this better
    let fq_identifier = filter(|c: &char| c.is_alphanumeric() || ":-_".contains(*c))
        .repeated()
        .padded()
        .collect::<String>()
        .validate(|name, span, emit| {
            if registry::make_node(name.as_str()).is_none() {
                emit(Simple::custom(
                    span, 
                    format!("Node not found: {}.", name)
                ));
            }

            name
        })
        .boxed();

    let literal = text::int::<_, Simple<char>>(10)
        .map(|s: String| NodeData::Int(s.parse().unwrap()))
        .boxed();

    let param_value = text::int(10)
        .map(|s: String| NodeParam::Int(s.parse().unwrap()))
        .boxed();

    let args = identifier
        .clone()
        .then_ignore(just("=").padded())
        .then(param_value)
        .boxed();

    let node = fq_identifier
        .clone()
        .then(
            args.separated_by(just(",").padded())
                .delimited_by(
                    just("(").padded(), 
                    just(")").padded()
                ),
        )
        .map(|(name, args)| {
            return registry::make_node(name.as_str())
                .map(|mut n| {
                    for (arg, value) in args.iter() {
                        n.set_param(arg, value.clone());
                    }
                    return n;
                })
                .unwrap()
        })
        .boxed();

    
    let flow = recursive(|flow|
        node.clone()
            .then((just(">>").padded().ignore_then(flow)).or_not())
            .map(|(mut a, b)| {
                match b {
                    Some(next) => a.set_next(Box::new(next)),
                    _ => {}
                };
                
                return a;
            })
        )
        .then_ignore(just(";").padded());
    

    let state = keyword("state")
        .padded()
        .ignore_then(identifier.clone())
        .then_ignore(just("{").padded())
        .then(flow.repeated())
        .then_ignore(just("}").padded())
        .boxed();

    let machine = keyword("machine")
        .ignore_then(identifier.clone())
        .then_ignore(just("{").padded())
        .then(state.repeated())
        .then_ignore(just("}").padded())
        .map(|(machine_name, states)| {
            let states_only = states
                .into_iter()
                .map(|(_, flows)| State::new(flows))
                .collect();

            Node::new(Box::new(Machine::new(states_only)))
        })
        .boxed();

    return machine;
}
