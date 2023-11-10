use alloc::{vec, vec::Vec};

use super::ShellError;

pub struct ShellPrompt<'a> {
    command: &'a str,
    args: Vec<&'a str>,
}

impl ShellPrompt<'_> {
    pub fn parse<'a>(prompt_text: &'a str) -> Result<ShellPrompt<'a>, ShellError> {
        let mut parts = prompt_text.split(' ');

        let command = parts.next().ok_or(ShellError::InvalidCommand)?;
        let args = parts.filter(|p| !p.is_empty()).collect();

        if command.is_empty() {
            return Err(ShellError::InvalidCommand);
        }

        return Ok(ShellPrompt { command, args });
    }

    pub fn get_command(&self) -> &str {
        return self.command;
    }

    pub fn get_args(&self) -> &[&str] {
        return &self.args;
    }
}

#[cfg(test)]
mod test {
    use super::ShellPrompt;

    #[test]
    pub fn test_empty() {
        assert!(ShellPrompt::parse("").is_err());
    }

    #[test]
    pub fn test_command_only() {
        let prompt = ShellPrompt::parse("foo").unwrap();
        assert_eq!(prompt.get_command(), "foo");
        assert!(prompt.get_args().is_empty());
    }

    #[test]
    pub fn test_args() {
        let prompt = ShellPrompt::parse("foo bar xyz abc").unwrap();
        assert_eq!("foo", prompt.get_command());
        assert_eq!(&["bar", "xyz", "abc"], prompt.get_args());
    }
}
