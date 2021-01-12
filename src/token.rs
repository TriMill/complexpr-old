use lazy_static::lazy_static;
use regex::Regex;
use crate::ops;

lazy_static! {
    static ref NEXT_TOKEN: Regex 
        = Regex::new(r###"-?^\d+(\.\d*)?i?|-?\.\d+i?|\(|\)|,|;|:|//|\^|<=?|>=?|!=|==|=|[+*-/%]=?|\$?[a-zA-Z_][a-zA-Z0-9_]*|"(?:[^"\\]|\\[\\"nrte0]|\\u\{[0-9a-fA-F]{1,8}\}|\\x[0-9a-fA-F]{2})*""###).unwrap();
    static ref IS_NUMBER: Regex
        = Regex::new(r"-?^\d+(\.\d*)?i?|-?\.\d+i?|-?i$").unwrap();
    static ref IS_IDENT: Regex
        = Regex::new(r"^\$?[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    static ref IS_STR: Regex
        = Regex::new(r###""(?:[^"\\]|\\[\\"nrte0]|\\u\{[0-9a-fA-F]{1,8}\}|\\x[0-9a-fA-F]{2})*""###).unwrap();
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    BinaryOp(ops::BinaryOp),
    UnaryOp(ops::UnaryOp),
    Assign, AssignOp(ops::BinaryOp),
    LParen, RParen,
    Comma, Semicolon, Colon,
    Integer(i64), Float(f64), Imaginary(f64), True, False,
    Identifier(String), Str(String),
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
            BinaryOp(ops::BinaryOp::Mul) 
                | BinaryOp(ops::BinaryOp::Div) 
                | BinaryOp(ops::BinaryOp::Frac) => 60,
            BinaryOp(ops::BinaryOp::Power) => 40,
            UnaryOp(ops::UnaryOp::Neg) => 35,
            Colon => 30,
            FunctionCall => 20,
            _ => 0
        }
    }

    pub fn right_assoc(&self) -> bool {
        use Token::*;
        match self {
           UnaryOp(_) => false,
           BinaryOp(ops::BinaryOp::Power) => true,
           Assign | AssignOp(_) => true,
           Comma => true,
           _ => false
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenizeError {
    Unexpected(usize, char), InvalidNumber(String), UnknownToken(String), InvalidCodepoint(u32)
}

impl std::fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unexpected(n, c)
                => write!(f, "Unexpected character {} at position {}", c, n),
            Self::InvalidNumber(s)
                => write!(f, "Numerical literal {} could not be parsed as a number", s),
            Self::UnknownToken(s)
                => write!(f, "Token beginning with {} extracted but unable to be identified", s),
            Self::InvalidCodepoint(n)
                => write!(f, "{:#x} is not a valid Unicode codepoint", n)
        }
    }
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
                    let last_len = nstr.chars().last().unwrap().to_string().len();
                    if nstr == "i" {
                        Token::Imaginary(1.)
                    } else if let Ok(n) = nstr.parse::<i64>() {
                        Token::Integer(n)
                    } else if let Ok(n) = nstr.parse::<f64>() {
                        Token::Float(n)
                    } else if let Ok(n) = nstr[..nstr.len()-last_len].parse::<f64>() {
                        if &nstr[nstr.len()-last_len..] == "i" {
                            Token::Imaginary(n)
                        } else {
                            return Err(TokenizeError::InvalidNumber(nstr.to_owned()))
                        }
                    } else {
                        return Err(TokenizeError::InvalidNumber(nstr.to_owned()))
                    }
                } else if let Some(ident) = IS_IDENT.find(token) {
                    Token::Identifier(ident.as_str().to_owned())
                } else if let Some(s) = IS_STR.find(token) {
                    Token::Str(parse_str(&s.as_str())?)
                } else {
                    return Err(TokenizeError::UnknownToken(token.to_owned()))
                }
            } else if let Some(ident) = IS_IDENT.find(token) {
                Token::Identifier(ident.as_str().to_owned())
            } else if let Some(s) = IS_STR.find(token) {
                Token::Str(parse_str(&s.as_str())?)
            } else {
                return Err(TokenizeError::UnknownToken(token.to_owned()))
            }
        };
        tokens.push(token);
    }
    Ok(tokens)
}

fn parse_str(raw_str: &str) -> Result<String, TokenizeError> {
    // unreachable!(): regex already checked that this is impossible
    let raw_str = &raw_str[1..(raw_str.len()-1)];
    let mut chars = raw_str.chars().peekable();
    let mut res = String::new();
    while let Some(c) = chars.next() {
        match c {
            '"' => unreachable!(),
            '\\' => {
                if let Some(esc) = chars.next() {
                    match esc {
                        'n' => res.push('\n'),
                        'r' => res.push('\r'),
                        't' => res.push('\t'),
                        'e' => res.push('\x1b'),
                        '0' => res.push('\0'),
                        '"' => res.push('"'),
                        '\\' => res.push('\\'),
                        'x' => if let (Some(h), Some(l)) = (chars.next(), chars.next()) {
                            let mut s = String::new();
                            s.push(h);
                            s.push(l);
                            let n = u32::from_str_radix(&s, 16).unwrap();
                            if let Some(c) = std::char::from_u32(n) {
                                res.push(c)
                            } else {
                                return Err(TokenizeError::InvalidCodepoint(n))
                            }
                        } else {
                            unreachable!()
                        },
                        'u' => {
                            chars.next(); //discard '{'
                            let mut s = String::new();
                            while let Some(c) = chars.next() {
                                if c == '}' {
                                    break
                                }
                                s.push(c);
                            }
                            let n = u32::from_str_radix(&s, 16).unwrap();
                            if let Some(c) = std::char::from_u32(n) {
                                res.push(c)
                            } else {
                                return Err(TokenizeError::InvalidCodepoint(n))
                            }
                        }
                        _ => unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
            c => res.push(c)
        }
    }
    Ok(res)
}
