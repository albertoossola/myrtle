use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::mask::mask::Mask;
use crate::NodeData;

pub struct ChainMask {
    items : Vec<Box<dyn Mask>>,
    index : usize
}

impl Mask for ChainMask {
    fn is_done(&self) -> bool {
        return self.index >= self.items.len();
    }

    fn push(&mut self, data: NodeData) -> Option<NodeData> {
        if self.index >= self.items.len() {
            return None;
        }

        let current_item = &mut self.items[self.index];
        let item_push = current_item.push(data);

        if current_item.is_done() {
            self.index += 1;
        }

        return item_push;
    }

    fn reset(&mut self) -> () {
        self.index = 0;
        self.items.iter_mut().for_each(|i| i.reset());
    }
}