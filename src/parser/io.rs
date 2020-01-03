use std::iter::Iterator;
use std::iter::Peekable;
use std::str::Chars;
use std::io::{self, prelude::*, BufReader};

type Location = (usize, usize);

pub struct GherkinLine {
    line_text: String,
    line_number: usize,
    indent: usize,
}

pub struct Token {
    line: GherkinLine,
    location: Location,
}

pub struct TokenScanner<R: Read> {
    pub line_number: usize,
    pub buf_reader: BufReader<R>,
}

impl GherkinLine {
    pub fn new(line_text: &str, line_number: usize) -> GherkinLine {
        let full_size: usize = line_text.len();
        let trimmed_size: usize = line_text.trim_start().trim_end().len();
        GherkinLine {
            line_text: String::from(line_text.trim_start().trim_end()),
            line_number: line_number,
            indent: full_size - trimmed_size,
        }
    }

    pub fn starts_with(&self, pat: &str) -> bool {
        self.line_text.starts_with(pat)
    }

    pub fn is_empty(&self) -> bool {
        self.line_text.is_empty()
    }

    pub fn get_content(&self) -> &str {
        &self.line_text
    }

    pub fn get_content_after(&self, pat: &str) -> &str {
        self.line_text.trim_start_matches(pat)
    }
}

impl Token {
    pub fn new(line: GherkinLine, location: Location) -> Token {
        Token {
            line: line,
            location: location,
        }
    }

    pub fn get_token_value(&self) -> &str {
        self.line.get_content()
    }
}

impl<R: Read> TokenScanner<R> {
    pub fn new(reader: R) -> TokenScanner<R> {
        TokenScanner {
            line_number: 0,
            buf_reader: BufReader::new(reader),
        }
    }
    
    pub fn next_token(&self) -> Option<Token> {
        let mut line: String;
        match self.buf_reader.read_line(&mut line) {
            Ok(_) => { Some(
                Token {
                    line: GherkinLine::new(&line, self.line_number),
                    location: (self.line_number, 0),
                })
            },
            Err(e) => { None },
        }
    }
}
