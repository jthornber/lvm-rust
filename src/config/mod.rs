// This is a straight translation of the C code.  I probably should
// use a parser generator or combinator library, but I need practise
// writing Rust.

use std::collections::HashMap;
use std::io::*;
use std::iter::SkipWhile;

// FIXME: what are the format flags for?

pub enum CfgValue {
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Box<CfgValue>>),
    Section(HashMap<String, Box<CfgValue>>)
}

enum Token {
    Int(i64),
    Float(f64),
    String(String),
    Eq,
    SectionBegin,
    SectionEnd,
    ArrayBegin,
    ArrayEnd,
    Identifier(String),
    Comma,
    Eof
}

fn is_space(c: &u8) -> bool {
    (*c as char) == ' '
}

fn eat_space<T: Iterator<Item=u8>>(input: &T) -> SkipWhile<u8> {
    (*input).skip_while(is_space)
}

#[cfg(test)]
mod test {
    use super::eat_space;
    
    fn assert_eq_iters<I1 : Iterator<Item=u8>,
                       I2 : Iterator<Item=u8>>(i1 : I1, i2 : I2) {
    }
    
    #[test]
    fn test_eat_space() {
        assert_eq_iters(eat_space(&"".bytes()), "".bytes())
    }
}

fn get_token<T: Iterator<Item=u8>>(input: &mut T,
                                   value_rather_than_ident: bool) -> Token {
    Token::Eof
}

pub fn parse<R: Read>(buf: R) -> Box<CfgValue> {
    let mut reader = BufReader::new(buf);

    for b in reader.bytes() {
        
    }
    
    Box::new(CfgValue::Int(0))
}

#[test]
fn test_parse() {
    let mut input = String::from("foo");
    parse(input.as_bytes());
}
