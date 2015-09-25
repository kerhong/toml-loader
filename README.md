# toml-loader
Load and parse toml files easily

[![Build Status](https://img.shields.io/travis/kerhong/toml-loader.svg)](https://travis-ci.org/kerhong/toml-loader)
[![Crates.io](https://img.shields.io/crates/v/toml-loader.svg)](https://crates.io/crates/toml-loader)
[![Coveralls.io](https://img.shields.io/coveralls/kerhong/toml-loader.svg)](https://coveralls.io/github/kerhong/toml-loader)

## License
MIT

## Documentation
[https://kerhong.github.io/toml-loader](https://kerhong.github.io/toml-loader)

## Examples
```rust
use toml_loader::Loader
use std::path::Path;

let toml = Loader::from_file(Path::new("some.toml")).unwrap();
```
