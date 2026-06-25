use crate::constants::{
    COMMAND_BROADCAST, COMMAND_CONNECT_NBR, COMMAND_EJECT, COMMAND_FORK, COMMAND_FORWARD,
    COMMAND_INCANTATION, COMMAND_INVENTORY, COMMAND_LEFT, COMMAND_LOOK, COMMAND_RIGHT, COMMAND_SET,
    COMMAND_TAKE, DEFAULT_COMMAND_TIME_UNITS, FORK_TIME_UNITS, INCANTATION_TIME_UNITS,
    INVENTORY_COMMAND_TIME_UNITS, ZERO_COMMAND_TIME_UNITS,
};
use crate::world::map::Resource;
use std::collections::VecDeque;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Forward,
    Right,
    Left,
    Look,
    Inventory,
    Broadcast(String),
    ConnectNbr,
    Fork,
    Eject,
    Take(Resource),
    Set(Resource),
    Incantation,
    Unknown,
}

impl Command {
    pub fn parse(line: &str) -> Self {
        match line {
            COMMAND_FORWARD => Self::Forward,
            COMMAND_RIGHT => Self::Right,
            COMMAND_LEFT => Self::Left,
            COMMAND_LOOK => Self::Look,
            COMMAND_INVENTORY => Self::Inventory,
            COMMAND_CONNECT_NBR => Self::ConnectNbr,
            COMMAND_FORK => Self::Fork,
            COMMAND_EJECT => Self::Eject,
            COMMAND_INCANTATION => Self::Incantation,
            _ => Self::parse_command_with_argument(line),
        }
    }

    fn parse_command_with_argument(line: &str) -> Self {
        if let Some(message) = line.strip_prefix(&format!("{COMMAND_BROADCAST} ")) {
            return if message.is_empty() {
                Self::Unknown
            } else {
                Self::Broadcast(message.to_string())
            };
        }
        if let Some(resource) = parse_single_resource_argument(line, COMMAND_TAKE) {
            return Self::Take(resource);
        }
        if let Some(resource) = parse_single_resource_argument(line, COMMAND_SET) {
            return Self::Set(resource);
        }
        Self::Unknown
    }

    pub const fn time_units(&self) -> u64 {
        match self {
            Self::Inventory => INVENTORY_COMMAND_TIME_UNITS,
            Self::ConnectNbr | Self::Unknown => ZERO_COMMAND_TIME_UNITS,
            Self::Fork => FORK_TIME_UNITS,
            Self::Incantation => INCANTATION_TIME_UNITS,
            _ => DEFAULT_COMMAND_TIME_UNITS,
        }
    }
}

fn parse_single_resource_argument(line: &str, command: &str) -> Option<Resource> {
    let mut parts = line.split_whitespace();
    if parts.next()? != command {
        return None;
    }
    let resource = Resource::from_protocol_name(parts.next()?)?;
    parts.next().is_none().then_some(resource)
}

#[derive(Debug)]
pub struct ActiveCommand {
    pub command: Command,
    pub finishes_at: Instant,
    pub incantation_id: Option<usize>,
}

impl ActiveCommand {
    pub fn new(command: Command, finishes_at: Instant) -> Self {
        Self {
            command,
            finishes_at,
            incantation_id: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct CommandQueue {
    commands: VecDeque<Command>,
}

impl CommandQueue {
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn push_back(&mut self, command: Command) {
        self.commands.push_back(command);
    }

    pub fn pop_front(&mut self) -> Option<Command> {
        self.commands.pop_front()
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use crate::world::map::Resource;

    #[test]
    fn parses_typed_commands() {
        assert_eq!(Command::parse("Take food"), Command::Take(Resource::Food));
        assert_eq!(
            Command::parse("Broadcast hello team"),
            Command::Broadcast("hello team".to_string())
        );
    }

    #[test]
    fn rejects_extra_arguments() {
        assert_eq!(Command::parse("Forward now"), Command::Unknown);
        assert_eq!(Command::parse("Set food now"), Command::Unknown);
    }
}
