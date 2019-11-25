// Copyright 2018 Nicholas Young (and contributors).
// All rights reserved.
//
// Released under a 3-Clause BSD License.
// You should have received a copy with this software.
// Otherwise, visit https://opensource.org to acquire
// a copy.

//! Matter
//!
//! A universal frontmatter parser and extractor,
//! written in Rust.
//!
//! Provided with frontmatter format and delimiters,
//! Matter is able to separate frontmatter from content,
//! and provide conventions for serializing Rust data
//! types into TOML, YAML, and JSON frontmatter formats.

use regex::{Captures, Regex};

lazy_static::lazy_static! {
    static ref DEFAULT_EXP: Regex =
        Regex::new(r"^[[:space:]]*\-\-\-\r?\n((?s).*?(?-s))\-\-\-\r?\n?((?s).*(?-s))$").unwrap();
    #[cfg(feature = "toml")]
    static ref TOML_EXP: Regex =
        Regex::new(r"^[[:space:]]*\+\+\+\r?\n((?s).*?(?-s))\+\+\+\r?\n?((?s).*(?-s))$").unwrap();
}

/// Split a string (often resulting from reading in a file)
/// into the frontmatter and content portions. Frontmatter
/// block formats are detected and parsed automatically.
pub fn extract(input: &str) -> Option<(String, String)> {
    let mut captures: Option<Captures> = None;

    if DEFAULT_EXP.is_match(input) {
        captures = DEFAULT_EXP.captures(input);
    }

    if captures.is_none() && TOML_EXP.is_match(input) {
        captures = TOML_EXP.captures(input);
    }

    match captures {
        Some(cap) => Some((cap[1].trim().to_string(), cap[2].trim().to_string())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::extract;

    #[test]
    fn extract_toml() {
        let contents = r#"
        +++
        title = "TOML Frontmatter"
        +++

        This is some content.
        "#;

        let (f, c) = match extract(contents) {
            Some((f, c)) => (f, c),
            _ => panic!(),
        };

        assert_eq!(f, "title = \"TOML Frontmatter\"");
        assert_eq!(c, "This is some content.");
    }

    #[test]
    fn extract_yaml() {
        let contents = r#"
        ---
        title: YAML Frontmatter
        ---

        This is some content.
        "#;

        let (f, c) = match extract(contents) {
            Some((f, c)) => (f, c),
            _ => panic!(),
        };

        assert_eq!(f, "title: YAML Frontmatter");
        assert_eq!(c, "This is some content.");
    }
}
