use super::*;

pub struct PrintBehaviour {
}

impl Behaviour for PrintBehaviour {
    fn step(
        &mut self,
        data: NodeData,
        vars: &mut VarStore,
    ) -> Option<NodeData> {
        println!("{}", data);
        Some(data)
    }

    fn reset(&mut self) { }

    fn is_working(&self) -> bool { false }
}

impl Parametric for PrintBehaviour {
    fn set_param(&mut self, param: &str, data : NodeParam) -> () {
        match param {
            _ => {}
        }
    }

    fn get_params(&self) -> &[&str] {
        &[]
    }
}

impl PrintBehaviour {
    pub fn new() -> PrintBehaviour {
        PrintBehaviour { }
    }
}