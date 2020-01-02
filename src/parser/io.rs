use std::iter::Iterator;
use std::iter::Peekable;
use std::str::Chars;

const CARRIAGE_RETURN: u8 = '\n' as u8;

pub struct InputStream {
    index: usize,
    length: usize,
    data: Vec<u8>,
    position: (usize, usize),
}

impl InputStream {
    pub fn from_string(input: &String) -> InputStream {
        InputStream { 
            index: 0,
            length: input.len(),
            data: input.as_bytes().to_vec(),
            position: (1,1), 
        }
    }

    pub fn get_position(&self) -> (usize, usize) { self.position }

    pub fn next(&mut self) -> Option<Box<u8>> {
        
        match self.peek() {
            Some(b) => {
                match *b {
                    CARRIAGE_RETURN => {
                        self.position.0 += 1;
                        self.position.1 = 1;
                    },
                    _ => { self.position.1 += 1; },
                }
                Some(b)
            },
            None => { None },
        }
    }
    
    pub fn peek(&mut self) -> Option<Box<u8>> {
        if self.index > self.length {
            return Some(Box::new(self.data[self.index]));
        } else {
            return None;
        }
    }

    pub fn is_eof(&mut self) -> bool {
        self.peek().is_none()
    }
}
