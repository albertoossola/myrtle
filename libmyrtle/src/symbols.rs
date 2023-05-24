use core::ops::IndexMut;

use alloc::{boxed::Box, vec, vec::Vec};

use crate::nodedata::NodeData;

/* Data Source */
pub trait DataSource {
    fn poll(&mut self) -> NodeData;
    fn can_push(&self) -> bool;
    fn push(&mut self, data: NodeData) -> ();

    fn can_open(&self) -> bool;
    fn open(&mut self) -> ();
    fn close(&mut self) -> ();
}

/* Variables in memory */

pub struct MemoryDataSource {
    value: NodeData,
}

impl MemoryDataSource {
    pub fn new() -> MemoryDataSource {
        MemoryDataSource {
            value: NodeData::Nil,
        }
    }
}

impl DataSource for MemoryDataSource {
    fn poll(&mut self) -> NodeData {
        let to_return = self.value;
        self.value = NodeData::Nil;
        return to_return;
    }

    fn can_push(&self) -> bool {
        true
    }

    fn push(&mut self, data: NodeData) -> () {
        self.value = data
    }

    fn open(&mut self) -> () {}

    fn close(&mut self) -> () {}

    fn can_open(&self) -> bool {
        true
    }
}

/* Symbols */

struct SymbolListenerStatus {
    active: bool,
    up_to_date: bool,
}

pub struct Symbol {
    pub source: Box<dyn DataSource>,
    listeners: Vec<SymbolListenerStatus>,
    polled_data: NodeData,
    open: bool,
}

impl Symbol {
    pub fn new(source: Box<dyn DataSource>) -> Symbol {
        Symbol {
            source: source,
            listeners: vec![],
            polled_data: NodeData::Nil,
            open: false,
        }
    }

    pub fn poll(&mut self, listener: i32) -> NodeData {
        if self
            .listeners
            .iter()
            .filter(|l| l.active)
            .all(|l| l.up_to_date)
        {
            let data_from_source = self.source.poll();

            if let NodeData::Nil = data_from_source {
                return NodeData::Nil;
            } else {
                self.polled_data = data_from_source;
                self.listeners.iter_mut().for_each(|l| {
                    l.up_to_date = false;
                })
            }
        }

        let listener_data = self.listeners.index_mut(listener as usize);

        if !listener_data.up_to_date {
            listener_data.up_to_date = true;
            return self.polled_data;
        }

        return NodeData::Nil;
    }

    pub fn register_listener(&mut self) -> i32 {
        let new_listener = SymbolListenerStatus {
            active: false,
            up_to_date: true,
        };

        self.listeners.push(new_listener);

        return (self.listeners.len() - 1) as i32;
    }

    pub fn activate_listener(&mut self, listener: i32) -> () {
        let listener_data = self.listeners.index_mut(listener as usize);

        listener_data.active = true;
        listener_data.up_to_date = true;
    }

    pub fn suspend_listener(&mut self, listener: i32) -> () {
        let listener_data = self.listeners.index_mut(listener as usize);
        listener_data.active = false;
    }

    pub fn can_push(&mut self) -> bool {
        if !self.is_open() {
            return false;
        }

        let all_listeners_up_to_date = self
            .listeners
            .iter()
            .filter(|l| l.active)
            .all(|l| l.up_to_date);

        self.source.can_push() && all_listeners_up_to_date
    }

    pub fn push(&mut self, data: NodeData) -> () {
        self.source.push(data);
    }

    pub fn open(&mut self) -> () {
        self.open = true;
    }

    pub fn can_open(&self) -> bool {
        self.source.can_open() && !self.open
    }

    pub fn close(&mut self) -> () {
        self.open = false;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}
