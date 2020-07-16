# Matter

[![CI Status
Badge](https://gitlab.com/secretfader/matter/badges/master/pipeline.svg)](https://gitlab.com/secretfader/matter)
[![Crates.io Downloads Badge](https://img.shields.io/crates/d/matter)](https://crates.io/crates/matter)
[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Made by Fader](https://img.shields.io/badge/made_by-Fader-purple.svg)](https://www.secretfader.com)

A universal frontmatter parser and extractor, written in Rust. Supports common
delimiters for YAML, TOML, and JSON.

## Installation

`matter = { version = "0.1.0-alpha4" }`

## Usage

Once installed, you'll notice that Matter exports only a handful of functions.
This is because it does most of the work for you. It also attempts to operate
with minimal overhead, by only allocating as necessary.

```rust
let input = std::fs::read_to_string("./path/to/content.md").unwrap();
let (matter, content) = matter::matter(&input).unwrap();
```

See [the docs](https://docs.rs/matter) for more examples.

## License

Copyright 2018 Nicholas Young, All rights reserved. Released under
a [3-Clause BSD License](LICENSE).
