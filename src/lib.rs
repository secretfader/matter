// Copyright 2018 Nicholas Young (and contributors).
// All rights reserved.
//
// Released under a 3-Clause BSD License. You should have received a
// copy with this software. Otherwise, visit https://opensource.org
// to acquire a copy.

//! A universal frontmatter parser and extractor.
//!
//! Provided with input data, Matter is able to separate frontmatter
//! from content. Common delimiters for supported formats are also
//! predefined.

use regex::{Captures, Regex};

lazy_static::lazy_static! {
    static ref DEFAULT_EXP: Regex =
        Regex::new(r"^[[:space:]]*\-\-\-\r?\n((?s).*?(?-s))\-\-\-\r?\n((?s).*(?-s))$").unwrap();
    #[cfg(feature = "toml")]
    static ref TOML_EXP: Regex =
        Regex::new(r"^[[:space:]]*\+\+\+\r?\n((?s).*?(?-s))\+\+\+\r?\n?((?s).*(?-s))$").unwrap();
}

/// Type alias for fallible extraction operations.
pub type ExtractResult<T = Option<(String, String)>> = std::result::Result<T, crate::Error>;

/// Errors that may occur during use of this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Length of input is zero")]
    InvalidLength,
}

/// Split a string (often resulting from reading in a file) into
/// frontmatter and content portions.
pub fn extract(input: &str) -> ExtractResult {
    if input.len() == 0 {
        return Err(Error::InvalidLength)
    }

    let mut captures: Option<Captures> = None;

    if DEFAULT_EXP.is_match(input) {
        captures = DEFAULT_EXP.captures(input);
    }

    if captures.is_none() && TOML_EXP.is_match(input) {
        captures = TOML_EXP.captures(input);
    }

    if let Some(cap) = captures {
        let res = (cap[1].trim().to_string(), cap[2].trim().to_string());
        return Ok(Some(res))
    }

    Ok(None)
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

        let (f, c) = match extract(contents).unwrap() {
            Some(res) => res,
            _ => panic!(),
        };

        assert_eq!(f, "title = \"TOML Frontmatter\"");
        assert_eq!(c, "This is some content.");
    }

    #[test]
    fn extract_basic_yaml() {
        let contents = r#"
        ---
        title: YAML Frontmatter
        ---

        This is some content.
        "#;

        let (f, c) = match extract(contents).unwrap() {
            Some(res) => res,
            _ => panic!(),
        };

        assert_eq!(f, "title: YAML Frontmatter");
        assert_eq!(c, "This is some content.");
    }

    #[test]
    fn extract_unquoted_yaml() {
        let contents = r#"
        ---
        title: Yaml Frontmatter --- Revenge of the Unquoted Strings
        ---

        This is some content.
        "#;

        let (f, c) = match extract(contents).unwrap() {
            Some(res) => res,
            _ => panic!(),
        };

        assert_eq!(f, "title: Yaml Frontmatter --- Revenge of the Unquoted Strings");
        assert_eq!(c, "This is some content.");
    }

    #[test]
    fn extract_multiline_yaml() {
        let contents = r#"
        ---
        text: |
            Nested multiline content, which may---contain loosely-formatted text.
        ---

        This is some content.
        "#;

        let (f, c) = match extract(contents).unwrap() {
            Some(res) => res,
            _ => panic!(),
        };

        let substr = r#"text: |
            Nested multiline content, which may---contain loosely-formatted text."#;
        assert_eq!(f, substr);
        assert_eq!(c, "This is some content.");
    }

    #[test]
    fn extract_nested_yaml() {
        let contents = r#"
        ---
        availability: public
        when:
          start: 1471/3/28 MTR 4::22
          duration: 0::30
        date: 2012-02-18
        title: Rutejìmo
        ---

        Text
        "#;

        let (f, c) = match extract(contents).unwrap() {
            Some(res) => res,
            _ => panic!(),
        };

        assert_ne!(f.len(), 0);
        assert_eq!(c, "Text");
    }
}
