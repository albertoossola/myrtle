use crate::{DataSource, NodeData};

use super::shellio::ShellIO;
use alloc::boxed::Box;

pub struct DataSourceShellIO {
    data_source: Box<dyn DataSource>
}

impl DataSourceShellIO {
    pub fn new(data_source : Box<dyn DataSource>) -> Self {
        Self {
            data_source
        }
    }
}

impl ShellIO for DataSourceShellIO {
    fn write(&mut self, c : u8) -> Result<(), super::ShellError> {
        while !self.data_source.can_push() { }

        self.data_source.push(NodeData::Int(c as i32));

        Ok(())
    }

    fn read(&mut self) -> Option<u8> {
        match self.data_source.poll() {
            crate::NodeData::Char(c) => Some(c as u8),
            crate::NodeData::Int(i) => Some(i as u8),
            crate::NodeData::Nil => None,
            _ => None
        }
    }
}