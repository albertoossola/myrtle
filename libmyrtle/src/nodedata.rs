#[derive(Clone, Copy, Debug)]
pub enum NodeData {
    Int(i32),
    Bool(bool),
    Char(char),
    Nil
}