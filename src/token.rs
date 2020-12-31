use lazy_static::lazy_static;
use regex::Regex;
use crate::ops;

lazy_static! {
    static ref NEXT_TOKEN: Regex 
        = Regex::new(r"^\d+(\.\d*)?i?|\.\d+i?|\(|\)|,|;|:|//|\^|<=?|>=?|!=|==|=|!|[+*-/%]=?|[a-zA-z][a-zA-Z0-9_]*").unwrap();
    static ref IS_NUMBER: Regex
        = Regex::new(r"^\d+(\.\d*)?i?|\.\d+i?|i$").unwrap();
    static ref IS_IDENT: Regex
        = Regex::new(r"^[a-zA-z][a-zA-Z0-9_]*$").unwrap();
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    BinaryOp(ops::BinaryOp),
    UnaryOp(ops::UnaryOp),
    Assign, AssignOp(ops::BinaryOp),
    LParen, RParen, Comma, Semicolon, Colon,
    Integer(i64), Float(f64), Imaginary(f64), True, False,
    Identifier(String),
    FunctionCall
}

impl Token {
    pub fn is_op(&self) -> bool {
        use Token::*;
        match self {
           UnaryOp(_) => true,
           BinaryOp(_) => true,
           Assign | AssignOp(_) => true,
           Comma => true,
           Semicolon => true,
           Colon => true,
           _ => false
        }
    }

    pub fn binding(&self) -> u32 {
        use Token::*;
        match self {
            Semicolon => 120,
            Comma => 110,
            Assign | AssignOp(_) => 100,
            BinaryOp(ops::BinaryOp::Greater)
                | BinaryOp(ops::BinaryOp::Less) 
                | BinaryOp(ops::BinaryOp::GreaterEqual) 
                | BinaryOp(ops::BinaryOp::LessEqual) 
                | BinaryOp(ops::BinaryOp::Equal) 
                | BinaryOp(ops::BinaryOp::NotEqual) => 80,
            BinaryOp(ops::BinaryOp::Add) 
                | BinaryOp(ops::BinaryOp::Sub) => 70,
            UnaryOp(ops::UnaryOp::Neg) => 60,
            BinaryOp(ops::BinaryOp::Mul) 
                | BinaryOp(ops::BinaryOp::Div) 
                | BinaryOp(ops::BinaryOp::Frac) => 50,
            BinaryOp(ops::BinaryOp::Power) => 40,
            Colon => 30,
            FunctionCall => 20,
            _ => 0
        }
    }

    pub fn right_assoc(&self) -> bool {
        use Token::*;
        match self {
           UnaryOp(_) => true,
           BinaryOp(ops::BinaryOp::Power) => true,
           Assign | AssignOp(_) => true,
           Comma => true,
           _ => false
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenizeError {
    Unexpected(usize, char), InvalidNumber(String), UnknownToken(String)
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut s = s.trim_end();
    let mut raw_tokens: Vec<&str> = vec![];
    let mut idx: usize = 0;
    while s.len() > 0 {
        let s_len_before = s.len();
        s = s.trim_start();
        idx += s_len_before - s.len();
        let next_match = NEXT_TOKEN.find(s);
        if next_match.is_none() {
            return Err(TokenizeError::Unexpected(idx, s.chars().next().unwrap_or('\0')));
        }
        let next_match = next_match.unwrap();
        let (start, end) = (next_match.start(), next_match.end());
        if start != 0 {
            return Err(TokenizeError::Unexpected(idx, s.chars().next().unwrap_or('\0')));
        }
        idx += end;
        raw_tokens.push(next_match.as_str());
        s = &s[end..];
    }
    let mut tokens: Vec<Token> = vec![];
    for raw in raw_tokens { 
        let token = match raw {
            "+" => Token::BinaryOp(ops::BinaryOp::Add),
            "-" => Token::BinaryOp(ops::BinaryOp::Sub),
            "*" => Token::BinaryOp(ops::BinaryOp::Mul),
            "/" => Token::BinaryOp(ops::BinaryOp::Div),
            "%" => Token::BinaryOp(ops::BinaryOp::Mod),
            "//" => Token::BinaryOp(ops::BinaryOp::Frac),
            "^" => Token::BinaryOp(ops::BinaryOp::Power),
            "==" => Token::BinaryOp(ops::BinaryOp::Equal),
            "!=" => Token::BinaryOp(ops::BinaryOp::NotEqual),
            ">" => Token::BinaryOp(ops::BinaryOp::Greater),
            "<" => Token::BinaryOp(ops::BinaryOp::Less),
            ">=" => Token::BinaryOp(ops::BinaryOp::GreaterEqual),
            "<=" => Token::BinaryOp(ops::BinaryOp::LessEqual),
            "=" => Token::Assign,
            "+=" => Token::AssignOp(ops::BinaryOp::Add),
            "-=" => Token::AssignOp(ops::BinaryOp::Sub),
            "*=" => Token::AssignOp(ops::BinaryOp::Mul),
            "/=" => Token::AssignOp(ops::BinaryOp::Div),
            "%=" => Token::AssignOp(ops::BinaryOp::Mod),
            "(" => Token::LParen,
            ")" => Token::RParen,
            "," => Token::Comma,
            ";" => Token::Semicolon,
            ":" => Token::Colon,
            "true" => Token::True,
            "false" => Token::False,
            token => if let Some(nstr) = IS_NUMBER.find_at(token, 0) {
                if nstr.start() == 0 {
                    let nstr = nstr.as_str();
                    if nstr == "i" {
                        Token::Imaginary(1.)
                    } else if let Ok(n) = nstr.parse::<i64>() {
                        Token::Integer(n)
                    } else if let Ok(n) = nstr.parse::<f64>() {
                        Token::Float(n)
                    } else if let Ok(n) = nstr[..nstr.len()-1].parse::<f64>() {
                        if &nstr[nstr.len()-1..] == "i" {
                            Token::Imaginary(n)
                        } else {
                            return Err(TokenizeError::InvalidNumber(nstr.to_owned()))
                        }
                    } else {
                        return Err(TokenizeError::InvalidNumber(nstr.to_owned()))
                    }
                } else if let Some(ident) = IS_IDENT.find(token) {
                    Token::Identifier(ident.as_str().to_owned())
                } else {
                    return Err(TokenizeError::UnknownToken(token.to_owned()))
                }
            } else if let Some(ident) = IS_IDENT.find(token) {
                Token::Identifier(ident.as_str().to_owned())
            } else {
                return Err(TokenizeError::UnknownToken(token.to_owned()))
            }
        };
        tokens.push(token);
    }
    Ok(tokens)
}
