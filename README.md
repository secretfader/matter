# Matter

A universal frontmatter parser and extractor, written in Rust.

## Installation

By default, matter is installed with support for TOML, YAML, and JSON
frontmatter formats. However, you can override the defaults and install support
for one specific format like so:

`matter = { version = "0.1.0alpha1", features = ["toml"] }`

## Usage

Once installed, you'll notice that Matter exports only a handful of functions.
This is because it does most of the work for you. It also attempts to operate
with minimal overhead, by only allocating as necessary.

```rust
let input = std::fs::read_to_string("./path/to/content.md").unwrap();
let (matter, content) = matter::extract(&input);
```

See [the docs](https://docs.rs/matter) for more examples.

## CLI

Matter is also distributed as a command line application. Install (`cargo
install matter-cli`) and run to inspect any file's frontmatter:

`matter path/to/content.md`

## License

Copyright 2018 Nicholas Young, All rights reserved. Released under a [3-Clause
BSD License](LICENSE).
