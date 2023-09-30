pub mod channels;
pub mod command_sources;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

use crate::interface::channels::{Channel, MemoryBufferChannel};
use crate::interface::command_sources::CommandSource;

use self::command_sources::Command;

pub struct Interface {
    command_source: Box<dyn CommandSource>,
    channels: Vec<Box<dyn Channel>>,
    current_channel: usize,
}

impl Interface {
    pub fn poll(&mut self) -> Option<String> {
        let polled = self.command_source.poll();

        return match polled {
            Some(Command::SetChannel(channel)) => self.set_channel(channel),
            Some(Command::Write(buffer)) => self.append_to_current_channel(buffer),
            Some(Command::Run) => self.get_channel_string_and_rewind(),
            None => None,
        };
    }

    pub fn set_channel(&mut self, channel: u8) -> Option<String> {
        let new_channel = self.get_current_channel();
        new_channel.rewind();

        return None;
    }

    fn append_to_current_channel(&mut self, byte : u8) -> Option<String> {
        let channel = self.get_current_channel();

        if channel.has_room_for(&[byte]) {
            channel.write(&[byte]);
        }

        return None;
    }

    pub fn get_channel_string_and_rewind(&mut self) -> Option<String> {
        let current_channel = self.get_current_channel();
        let channel_content_or_none = current_channel.get_string_or_none();
        current_channel.rewind();

        return channel_content_or_none;
    }

    fn get_current_channel(&mut self) -> &mut dyn Channel {
        return self.channels[self.current_channel].as_mut();
    }

    fn is_valid_channel_index(&self, channel: u8) -> bool {
        return (channel as usize) < self.channels.len();
    }

    pub fn new(source: Box<dyn CommandSource>, channels: Vec<Box<dyn Channel>>) -> Interface {
        let mut channels: Vec<Box<dyn Channel>> = channels;

        return Interface {
            command_source: source,
            channels,
            current_channel: 0,
        };
    }
}
