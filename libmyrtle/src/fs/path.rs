use alloc::string::String;
use alloc::vec::Vec;
use core::slice::Split;

pub struct Path { }

impl Path {
    pub fn get_head_tail(path : &str) -> Option<(&str, &str)> {
        let trimmed_path = path.trim_matches('/');

        if trimmed_path.len() == 0 {
            return None;
        }

        let index_of_slash = trimmed_path.find('/');

        return match index_of_slash {
            Some(position) => Self::split_head_and_remainder(trimmed_path, position),
            None => Some((trimmed_path, ""))
        }
    }

    fn split_head_and_remainder(path : &str, position : usize) -> Option<(&str, &str)> {
        let head = &path[0..position];
        let remainder = &path[position + 1..];

        return Some((head, remainder));
    }
}

#[cfg(test)]
mod tests {
    use crate::fs::path::Path;

    #[test]
    fn test_empty_path() {
        let split_or_none = Path::get_head_tail("");
        assert!(split_or_none.is_none());
    }

    #[test]
    fn test_only_slashes() {
        let split_or_none = Path::get_head_tail("///");
        assert!(split_or_none.is_none());
    }

    #[test]
    fn test_no_slash() {
        let split_or_none = Path::get_head_tail("foobar");
        assert_eq!(split_or_none, Some(("foobar", "")));
    }

    #[test]
    fn test_leading_slash() {
        let split_or_none = Path::get_head_tail("/foobar");
        assert_eq!(split_or_none, Some(("foobar", "")));
    }

    #[test]
    fn test_multipart() {
        let split_or_none = Path::get_head_tail("/foo/bar/xyz/");
        assert_eq!(split_or_none, Some(("foo", "bar/xyz")));
    }
}