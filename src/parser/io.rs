use crate::parser::dialect::Dialect;
use std::iter::Iterator;
use std::iter::Peekable;
use std::str::Chars;
use std::io::{self, prelude::*, BufReader};

type Location = (usize, usize);

const DEFAULT_DIALECT: &str = &"en";

pub struct GherkinLine {
    line_text: String,
    line_number: usize,
    indent_position: usize,
}

pub struct Token {
    pub line: GherkinLine,
    location: Location,
}

pub struct GherkinBuffer<R: Read> {
    pub line_number: usize,
    pub buf_reader: BufReader<R>,
}

pub struct TokenMatcher {
    dialect: Dialect,
    active_doc_string: bool
}

impl GherkinLine {
    pub fn new(line_text: &str, line_number: usize) -> GherkinLine {
        GherkinLine {
            line_text: String::from(line_text.trim_end()),
            line_number: line_number,
            indent_position: line_text.len() - line_text.trim_start().len(),
        }
    }

    pub fn starts_with(&self, pat: &str) -> bool {
        self.line_text.starts_with(pat)
    }

    pub fn is_empty(&self) -> bool {
        self.line_text.is_empty()
    }

    pub fn get_content(&self) -> &str {
        &self.line_text.trim_start()
    }

    pub fn get_content_indented(&self) -> &str {
        &self.line_text
    }

    pub fn get_content_after(&self, pat: &str) -> &str {
        self.line_text.trim_start_matches(pat)
    }

    // data table stuff
}

impl Token {
    pub fn new(line: GherkinLine, location: Location) -> Token {
        Token {
            line: line,
            location: location,
        }
    }

    pub fn get_value(&self) -> &str {
        self.line.get_content()
    }
}

impl<R: Read> GherkinBuffer<R> {
    pub fn new(reader: R) -> GherkinBuffer<R> {
        GherkinBuffer {
            line_number: 0,
            buf_reader: BufReader::new(reader),
        }
    }
    
    pub fn next_line(&mut self) -> Option<GherkinLine> {
        let mut line: String = String::with_capacity(64);
        match &self.buf_reader.read_line(&mut line) {
            Ok(_) => { Some(GherkinLine::new(&line, self.line_number)) },
            Err(e) => { None },
        }
    }
}

impl TokenMatcher {
    pub fn new(dialect: &str) -> TokenMatcher {
        TokenMatcher {
            dialect: Dialect::new(dialect),
            active_doc_string: false,
        }
    }

    pub fn match_tag_line(&self, token: Token) -> bool {
        token.line.starts_with("@")
    }

    pub fn match_feature_line(&self, token: Token) -> bool {
        let mut result = false;
        for keyword in &self.dialect.feature {
            result |= token.line.starts_with(&keyword)
        }
        result
    }

    pub fn match_rule_line(&self, token: Token) -> bool {
        token.line.starts_with("@")
    }

    pub fn next_token() -> Option<Token> {
        None
    }
}