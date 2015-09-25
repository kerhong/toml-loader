extern crate toml;
#[macro_use] extern crate quick_error;

use toml::{Parser, Encoder, Value, ParserError};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io;
use std::io::Read;

quick_error! {
    /// Wraps all errors this library can produce
    #[derive(Debug)]
    pub enum LoadError {
        /// IO Error
        Io(err: io::Error) {
            from()
        }
        /// TOML parsing error
        Parse(err: Vec<ParserError>) {
            from()
        }
        /// Multiple files error
        MultiFile(err: &'static str)
    }
}

/// Imlements helper functions for loading and parsing .toml files
pub struct Loader;

impl Loader {
    /// Create Toml::Value from file specified by `path`
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

    /// Create Toml::Value from all files in `paths`.
    /// Will merge all configs into one.
    ///
    /// # Errors
    /// This function will return an error if any toml items are defined more than once.
    ///
    /// ## Except
    /// Adding items to existing table will insert if keys don't match.
    ///
    /// Adding items to array will append.
    ///
    /// # Example
    /// ```no_run
    /// use toml_loader::Loader;
    /// use std::path::Path;
    ///
    /// let mut paths = vec![];
    /// paths.push(Path::new("foo.toml").to_path_buf());
    /// paths.push(Path::new("bar.toml").to_path_buf());
    /// let toml = Loader::from_multiple_files(&paths).unwrap();
    /// ```
    pub fn from_multiple_files(paths: &Vec<PathBuf>) -> Result<Value, LoadError> {
        let empty = Value::Table(Encoder::new().toml);
        paths.iter().map(|p| { Loader::from_file(p) }).fold(Ok(empty), |a, i| {
            match a {
                Ok(_) => {
                    match i {
                        Ok(_) => {
                            merge_toml(a.as_ref().unwrap(), i.as_ref().unwrap())
                        },
                        _ => i
                    }
                },
                _ => a
            }
        })
    }
}

fn merge_values(a: &mut Value, b: &Value, pos: &str) -> Option<LoadError> {
    match b {
        &Value::Table(_) => {
        },
        &Value::Array(_) => {
        },
        _ => {
        }
    };
    None
}

fn merge_toml(a: &Value, b: &Value) -> Result<Value, LoadError> {
    let mut res = a.clone();
    merge_values(&mut res, b, "");
    Ok(res)
}

#[cfg(test)]
mod test {
    extern crate tempfile;
    extern crate tempdir;

    use super::Loader;
    use self::tempfile::NamedTempFile;
    use self::tempdir::TempDir;
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

    macro_rules! tmp_toml_file_indir {
        ($path:ident, $content:expr) => {
            {
                let mut f = NamedTempFile::new_in($path).unwrap();
                f.write_all($content.as_bytes()).unwrap();
                f.flush().unwrap();
                f
            }
        };
    }

    #[test]
    fn loads_valid_toml_merge() {
        let d = TempDir::new("toml-loader-test").unwrap();
        let path = d.path();
        let f1 = tmp_toml_file_indir!(path, "[test] a = 1");
        let f2 = tmp_toml_file_indir!(path, "[test] b = 2");
        let paths = vec![f1.path().to_path_buf(), f2.path().to_path_buf()];
        Loader::from_multiple_files(&paths).unwrap();
    }
}
