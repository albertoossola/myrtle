use super::*;
extern crate proc_macro;
use proc_macro::TokenStream;

pub trait Parametric {
  fn set_param(&mut self, param: &str, data : NodeParam) -> ();

  fn get_params(&self) -> &[&str];
}

pub trait Behaviour : Parametric {
  fn step(&mut self, data : NodeData, vars : &mut VarStore) -> Option<NodeData>;
  fn is_working(&self) -> bool;
  fn reset(&mut self) -> ();
}