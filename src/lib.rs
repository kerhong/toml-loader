extern crate toml;
#[macro_use] extern crate quick_error;

use toml::{Parser, Value, ParserError};
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;

quick_error! {
    #[derive(Debug)]
    pub enum LoadError {
        Io(err: io::Error) {
            from()
        }
        Parse(err: Vec<ParserError>) {
            from()
        }
    }
}

/// Imlements helper functions for loading and parsing .toml files
pub struct Loader;

impl Loader {
    /// Create Toml::Value from file specified by path
    ///
    /// # Example
    /// ```no_run
    /// use toml_loader::Loader;
    /// use std::path::Path;
    ///
    /// let toml = Loader::from_file(Path::new("some.toml")).unwrap();
    /// ```
    pub fn from_file(path: &Path) -> Result<Value, LoadError> {
       let mut f = try!(File::open(path));
       let mut s = String::new();
       try!(f.read_to_string(&mut s));
       let mut parser = Parser::new(&s);
       let res = try!(parser.parse().map(Value::Table).ok_or(parser.errors));
       Ok(res)
    }
}

#[cfg(test)]
mod test {
    extern crate tempfile;

    use super::Loader;
    use self::tempfile::NamedTempFile;
    use std::io::Write;
    use std::path::Path;

    macro_rules! tmp_toml_file {
        ($content:expr) => {
            {
                let mut f = NamedTempFile::new().unwrap();
                f.write_all($content.as_bytes()).unwrap();
                f.flush().unwrap();
                f
            }
        }
    }

    #[test]
    fn loads_valid_toml_single() {
        let f = tmp_toml_file!("a = 1");
        Loader::from_file(f.path()).unwrap();
    }

    #[test]
    #[should_panic]
    fn fails_to_load_invalid_toml_single() {
        let f = tmp_toml_file!("a - 1");
        Loader::from_file(f.path()).unwrap();
    }

    #[test]
    #[should_panic]
    fn fails_to_load_non_existing_file() {
        // Path "?\0" is invalid on both windows and linux (windows can't have ? and unix can't have \0)
        Loader::from_file(Path::new("?\0")).unwrap();
    }
}
