extern crate pest;

use std::time::Duration;

use pest::{Parser, iterators::Pair};

use crate::runtime::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyrtleParser;

pub fn parse(src : &str) -> Machine {
  match MyrtleParser::parse(Rule::machine, src) {
    Ok(mut pairs) => { 
      println!("Parsing.");

      let first_pair = pairs.next().unwrap();
    
      match first_pair.as_rule() {
        Rule::machine => parse_machine(first_pair),
        _ => { panic!("Root element is not a machine!"); }
      }
    },
    Err(_) => { panic!("Oh no"); }
  }
}

pub fn parse_machine(src : Pair<Rule>) -> Machine {
  match src.as_rule() {
    Rule::machine => {
      let mut statements = src.into_inner();

      let name_pair = statements.next().unwrap();
      let machine_name = parse_string(name_pair);

      println!("Parsing machine {}", machine_name);

      let mut states = vec![];

      for pair in statements {
        match pair.as_rule() {
          Rule::state => {
            states.push(parse_state(pair));
          },
          _ => {
            println!("Found other rule: {}", pair);
          }
        }
      }

      return Machine::new(states);
    },
    _ => { }
  };

  panic!("MACHINE AAAAAAH");
}

fn parse_state(src : Pair<Rule>) -> State {
  match src.as_rule() {
    Rule::state => {

      let mut statements = src.into_inner();

      //let states = vec![];

      let name_pair = statements.next().unwrap();
      let state_name = parse_string(name_pair);

      println!("Parsing state {}", state_name);

      let mut flows = vec![];

      for pair in statements {
        flows.push(parse_flow(pair));
      }

      return State::new(flows);
    },
    _ => { }
  };

  panic!("AAAAAAAAARGH");
}

fn parse_flow(src : Pair<Rule>) -> Node {
  let mut builder = FlowBuilder::new();

  match src.as_rule() {
    Rule::flow => {
      println!("Parsing flow...");

      let statements = src.into_inner();

      //let states = vec![];


      for pair in statements {
        match pair.as_rule() {
          Rule::node => {
            let mut parts = pair.into_inner();
            let mut node_name = parse_string(parts.next().unwrap());

            //TODO: auto name matching
            match node_name.as_str() {
              "print" => builder.append(Box::new(PrintNode {})),
              "timer" => builder.append(Box::new(TimerNode::new(Duration::from_millis(200)))),
              _ => { panic!("Unrecognized node \"{}\"", node_name) }
            };

            println!("Node {}", node_name);
          },
          _ => { println!("Found other token: {}", pair) }
        }
        //states.push(parse_state(pair.into_inner()));
      }

    },
    _ => { }
  };

  return *builder.build();
}

fn parse_args(src : Pair<Rule>) -> () {

}

fn parse_string(pair : Pair<Rule>) -> String {
  return pair.as_str().to_string();
}