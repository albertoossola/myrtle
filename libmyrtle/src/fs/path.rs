use alloc::boxed::Box;

use super::FsError;

#[derive(PartialEq)]
pub enum Path<'a> {
    Segment(&'a str, Box<Path<'a>>),
    End,
}

impl Path<'_> {
    pub fn new(path: &str) -> Result<Path, FsError> {
        if !path
            .chars()
            .all(|c| char::is_alphanumeric(c) || c == '/' || c == '_')
        {
            return Err(FsError::InvalidPath);
        }

        let trimmed = path.trim_matches('/');

        if trimmed.is_empty() {
            return Ok(Path::End);
        }

        let new_path = match trimmed.find("/") {
            Some(index) => {
                let subpath = Self::new(&trimmed[index + 1..])?;
                return Ok(Path::Segment(&trimmed[..index], Box::new(subpath)));
            }
            None => Path::Segment(trimmed, Box::new(Path::End)),
        };

        return Ok(new_path);
    }
}

#[cfg(test)]
mod tests {
    use crate::fs::path::Path;

    #[test]
    fn test_parse_empty() {
        let path_or_err = Path::new("");
        assert!(path_or_err.unwrap() == Path::End);
    }

    #[test]
    fn test_parse_slash() {
        let path_or_err = Path::new("/");
        assert!(path_or_err.unwrap() == Path::End);
    }

    #[test]
    fn test_parse_double_slash() {
        let path_or_err = Path::new("//");
        assert!(path_or_err.is_err());
        assert!(path_or_err.unwrap() == Path::End);
    }

    #[test]
    fn test_single_item() {
        let path_or_err = Path::new("foo");
        match path_or_err {
            Ok(Path::Segment("foo", _)) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn test_multiple_items() {
        let path = Path::new("foo/bar/xyz");

        if let Ok(Path::Segment("foo", _)) = path {
        } else {
            panic!();
        }
    }
}
