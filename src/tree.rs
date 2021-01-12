use crate::Value;
use crate::ops::*;
use crate::token::*;
use crate::function::{self, EvalError, EvalErrorKind, EvalTrace};

pub type Context = std::collections::HashMap<String, Value>;

#[derive(Clone, Debug)]
pub enum TreeError {
    NoLParen, NoRParen, TokenNoArgs(Token), AssignLeftInvalid, ColonLeftNotIdentifier(Node), NoOperator(Token)
}

impl std::fmt::Display for TreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoLParen => write!(f, "Unmatched closing parenthesis ')'"),
            Self::NoRParen => write!(f, "Unmatched opening parenthesis '('"),
            Self::TokenNoArgs(t) => write!(f, "Token {:?} is missing one or both arguments", t),
            Self::AssignLeftInvalid => write!(f, "Can only assign to an identifier"),
            Self::ColonLeftNotIdentifier(_) => write!(f, "Left-hand side of colon must be an identifier or a list of identifiers"),
            Self::NoOperator(t) => write!(f, "Token {:?} has no corresponding operator", t)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Node {
    BinaryOp(BinaryOp, Box<Node>, Box<Node>), 
    UnaryOp(UnaryOp, Box<Node>),
    Assign(String, Box<Node>),
    AssignOp(BinaryOp, String, Box<Node>),
    FunctionCall(Box<Node>, Vec<Node>),
    FunctionCreate(Vec<String>, Box<Node>),
    Value(Value), Identifier(String),
    List(Vec<Node>), Block(Vec<Node>)
}

impl Node {
    pub fn eval(&self, ctx: &mut Context) -> Result<Value, EvalError> {
        match self {
            Self::Assign(name, value) => {
                if name == "true" || name == "false" || name.chars().nth(0) == Some('$') {
                    return Err(EvalErrorKind::IdentifierReserved(name.to_owned()).into())
                }
                let result = value.eval(ctx)?;
                ctx.insert(name.to_owned(), result);
                Ok(Value::Void)
            },
            Self::AssignOp(op, name, value) => {
                if name == "true" || name == "false" || name.chars().nth(0) == Some('$') {
                    return Err(EvalErrorKind::IdentifierReserved(name.to_owned()).into())
                }
                let name = name.to_owned();
                if ctx.contains_key(&name) {
                    let prev = ctx[&name].clone();
                    let result = value.eval(ctx)?;
                    let result = op.eval(prev, result)?;
                    ctx.insert(name.to_owned(), result);
                    Ok(Value::Void)
                } else {
                    Err(EvalErrorKind::VariableUnset(name).into())
                }
            },
            Self::UnaryOp(op, rhs) => {
                let rhs = rhs.eval(ctx)?;
                op.eval(rhs)
            },
            Self::BinaryOp(op, lhs, rhs) => {
                let lhs = lhs.eval(ctx)?;
                let rhs = rhs.eval(ctx)?;
                op.eval(lhs, rhs)
            },
            Self::Value(v) => Ok(v.clone()),
            Self::Identifier(s) => if s.chars().nth(0) == Some('$') {
                match &s[..] {
                    "$ctx" => Ok(ctx_to_value(ctx)),
                    x => Err(EvalErrorKind::InvalidSpecialIdent(x.to_owned()).into())
                }
            } else if ctx.contains_key(s) {
                Ok(ctx[s].clone())
            } else {
                Err(EvalErrorKind::VariableUnset(s.to_owned()).into())
            },
            Self::List(v) => {
                let mut items = vec![];
                for i in v {
                    items.push(i.eval(ctx)?);
                }
                Ok(Value::List(items))
            },
            Self::Block(v) => {
                let mut last = Value::Void;
                for i in v {
                    last = i.eval(ctx)?;
                }
                Ok(last)
            },
            Self::FunctionCall(name, args) => {
                if let Node::Identifier(name) = *name.clone() {
                    if &name[0..1] == "$" {
                        let res = match &name[1..] {
                            "include" => include(args, ctx),
                            "catch" => catch(args, ctx),
                            "set" => set(args, ctx),
                            "unset" => unset(args, ctx),
                            "is_set" => is_set(args, ctx),
                            "get" => get(args, ctx),
                            x => return Err(EvalErrorKind::InvalidSpecialIdent(x.to_owned()).into())
                        };
                        return match res {
                            Ok(x) => Ok(x),
                            Err(EvalError{kind, trace: EvalTrace::None}) 
                                => Err(EvalError{kind: kind, trace: EvalTrace::Function(name.to_owned())}),
                            Err(e) => Err(e)
                        }
                    }
                } 
                let func = name.eval(ctx)?;
                let mut argvals = vec![];
                for arg in args {
                    argvals.push(arg.eval(ctx)?);
                }
                func.eval(argvals)
            },
            Self::FunctionCreate(args, inner) => {
                Ok(Value::Lambda{args: args.to_vec(), func: inner.clone(), ctx: Box::new(ctx.clone())})
            }
        }
    }
}

fn include(args: &Vec<Node>, ctx: &mut Context) -> function::Result {
    use {std::io::Read, std::sync::Arc};
    function::bound_args(args.len(), 1, 1)?;
    let a = args[0].eval(ctx)?;
    if let Value::Str(name) = a {
        let mut buf = String::new();
        let mut f = match std::fs::File::open(name) {
            Ok(x) => x,
            Err(e) => return Err(EvalErrorKind::IOError(Arc::new(e)).into())
        };
        if let Err(e) = f.read_to_string(&mut buf) {
            return Err(EvalErrorKind::IOError(Arc::new(e)).into())
        }
        match crate::eval(&buf, ctx) {
            Ok(x) => Ok(x),
            Err(crate::Error::Eval(e)) => Err(e),
            Err(x) => Err(EvalErrorKind::Other(format!("{}", x)).into())
        }
    } else {
        Err(EvalErrorKind::WrongArgType(a).into())
    }
}

fn catch(args: &Vec<Node>, ctx: &mut Context) -> function::Result {
    function::bound_args(args.len(), 1, 2)?;
    let alt = if args.len() == 1 {
        Value::Void
    } else {
        args[1].eval(ctx)?
    };
    match args[0].eval(ctx) {
        Ok(x) => Ok(x),
        Err(_) => Ok(alt)
    }
}

fn set(args: &Vec<Node>, ctx: &mut Context) -> function::Result {
    function::bound_args(args.len(), 2, 2)?;
    let a = args[0].eval(ctx)?;
    let val = args[1].eval(ctx)?;
    if let Value::Str(name) = a {
        ctx.insert(name, val);
        Ok(Value::Void)
    } else {
        Err(EvalErrorKind::WrongArgType(a).into())
    }
}

fn unset(args: &Vec<Node>, ctx: &mut Context) -> function::Result {
    function::bound_args(args.len(), 1, 1)?;
    let a = args[0].eval(ctx)?;
    if let Value::Str(name) = a {
        let val = ctx.get(&name).cloned().unwrap_or(Value::Void);
        ctx.remove(&name);
        Ok(val)
    } else {
        Err(EvalErrorKind::WrongArgType(a).into())
    }
}

fn is_set(args: &Vec<Node>, ctx: &mut Context) -> function::Result {
    function::bound_args(args.len(), 1, 1)?;
    let a = args[0].eval(ctx)?;
    if let Value::Str(name) = a {
        Ok(Value::Bool(ctx.contains_key(&name)))
    } else {
        Err(EvalErrorKind::WrongArgType(a).into())
    }
}

fn get(args: &Vec<Node>, ctx: &mut Context) -> function::Result {
    function::bound_args(args.len(), 1, 1)?;
    let a = args[0].eval(ctx)?;
    if let Value::Str(name) = a {
        match ctx.get(&name) {
            Some(x) => Ok(x.clone()),
            None => Err(EvalErrorKind::VariableUnset(name).into())
        }
    } else {
        Err(EvalErrorKind::WrongArgType(a).into())
    }
}

pub fn value_to_ctx(value: &Value) -> Option<Context> {
    if let Value::List(list) = value {
        let mut ctx = Context::new();
        for item in list {
            if let Value::List(l) = item {
                if l.len() != 2 {
                    return None
                }
                if let Value::Str(s) = &l[0] {
                    ctx.insert(s.to_owned(), l[1].clone());
                } else {
                    return None
                }
            } else {
                return None
            }
        }
        Some(ctx)
    } else {
        None
    }
}

pub fn ctx_to_value(ctx: &Context) -> Value {
    let mut l = vec![];
    for (k, v) in ctx {
        l.push(Value::List(vec![Value::Str(k.to_owned()), v.clone()]))
    }
    Value::List(l)
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
enum GroupNode {
    Token(Token), Node(Vec<GroupNode>), List(Vec<GroupNode>)
}

impl std::fmt::Debug for GroupNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Token(a) => write!(f, "{:?}", a)?,
            Self::Node(v) => write!(f, "({})]", 
                    v.iter()
                    .map(|x| format!("{:?}", x))
                    .collect::<Vec<String>>()
                    .join(", "))?,
            Self::List(v) => write!(f, "[{}]", 
                    v.iter()
                    .map(|x| format!("{:?}", x))
                    .collect::<Vec<String>>()
                    .join(", "))?
        }
        Ok(())
    }
}

pub fn gen_tree(tokens: Vec<Token>) -> Result<Node, TreeError> {
    let parentree = parentree(tokens)?;
    finish_tree(parentree)
}

fn finish_tree(root: GroupNode) -> Result<Node, TreeError> {
    if let GroupNode::Token(a) = root {
        match a {
            Token::BinaryOp(_) | Token::UnaryOp(_) | Token::Assign | Token::AssignOp(_)
            | Token::Comma | Token::FunctionCall | Token::Colon
                => Err(TreeError::TokenNoArgs(a)),
            Token::Integer(n) => Ok(Node::Value(Value::Integer(n))),
            Token::Float(n) => Ok(Node::Value(Value::Float(n))),
            Token::Imaginary(n) => Ok(Node::Value(Value::from_complex(0., n))),
            Token::True => Ok(Node::Value(Value::Bool(true))),
            Token::False => Ok(Node::Value(Value::Bool(false))),
            Token::Identifier(s) => Ok(Node::Identifier(s)),
            Token::Str(s) => Ok(Node::Value(Value::Str(s))),
            Token::Semicolon => Ok(Node::Value(Value::Void)),
            Token::LParen | Token::RParen => unreachable!()
        }
    } else if let GroupNode::Node(nodes) = root {
        if nodes.len() == 0 {
            return Ok(Node::List(vec![]));
        } else if nodes.len() == 1 {
            return finish_tree(nodes[0].clone());
        }
        let (before, pivot, after) = next_split(nodes);
        if let GroupNode::Token(token) = pivot {
            match token {
                Token::LParen | Token::RParen => unreachable!(),
                Token::BinaryOp(op) => if before.len() == 0 || after.len() == 0 {
                    return Err(TreeError::TokenNoArgs(Token::BinaryOp(op)))
                } else {
                    return Ok(Node::BinaryOp(op, 
                        Box::new(finish_tree(GroupNode::Node(before))?), 
                        Box::new(finish_tree(GroupNode::Node(after))?)))
                },
                Token::UnaryOp(op) => if after.len() == 0 {
                    return Err(TreeError::TokenNoArgs(Token::UnaryOp(op)))
                } else {
                    return Ok(Node::UnaryOp(op,
                            Box::new(finish_tree(GroupNode::Node(after))?)))
                },
                Token::Assign => if before.len() == 1 {
                    if let GroupNode::Token(Token::Identifier(name)) = &before[0] {
                        return Ok(Node::Assign(name.to_owned(),
                            Box::new(finish_tree(GroupNode::Node(after))?)))
                    } else {
                        return Err(TreeError::AssignLeftInvalid)
                    }
                } else {
                    return Err(TreeError::AssignLeftInvalid)
                },
                Token::AssignOp(op) => if before.len() == 1 {
                    if let GroupNode::Token(Token::Identifier(name)) = &before[0] {
                        return Ok(Node::AssignOp(op, name.to_owned(),
                            Box::new(finish_tree(GroupNode::Node(after))?)))
                    } else {
                        return Err(TreeError::AssignLeftInvalid)
                    }
                } else {
                    return Err(TreeError::AssignLeftInvalid)
                },
                Token::FunctionCall => {
                    let name = if before.len() == 1 {
                        finish_tree(before[0].clone())?
                    } else {
                        finish_tree(GroupNode::Node(before))?
                    };
                    let args = finish_tree(GroupNode::Node(after))?;
                    if let Node::List(args) = args {
                        return Ok(Node::FunctionCall(Box::new(name), args))
                    } else {
                        return Ok(Node::FunctionCall(Box::new(name), vec![args]))
                    }
                },
                Token::Comma => {
                    let mut items = vec![];
                    if before.len() == 0 {
                        return Err(TreeError::TokenNoArgs(Token::Comma))
                    }
                    items.push(finish_tree(GroupNode::Node(before))?);
                    if after.len() > 0 {
                        let mut cdr = after;
                        loop {
                            let split = next_split(cdr.clone());
                            let (car, pivot) = (split.0, split.1);
                            let next_cdr = split.2;
                            if pivot == GroupNode::Token(Token::Comma) {
                                items.push(finish_tree(GroupNode::Node(car))?);
                                if next_cdr.len() == 0 {
                                    break
                                } else {
                                    cdr = next_cdr;
                                }
                            } else {
                                items.push(finish_tree(GroupNode::Node(cdr))?);
                                break
                            }
                        }
                    }
                    return Ok(Node::List(items))
                },
                Token::Semicolon => {
                    if before.len() == 0 {
                        return finish_tree(GroupNode::Node(after))
                    } else if after.len() == 0 {
                        return Ok(Node::Block(vec![
                            finish_tree(GroupNode::Node(before))?,
                            Node::Value(Value::Void)
                        ]))
                    }
                    let before = finish_tree(GroupNode::Node(before))?;
                    let after = finish_tree(GroupNode::Node(after))?;
                    if let Node::Block(mut v) = after {
                        v.insert(0, before);
                        return Ok(Node::Block(v));
                    } else {
                        return Ok(Node::Block(vec![before, after]));
                    }
                },
                Token::Colon => {
                    let before = finish_tree(GroupNode::Node(before))?;
                    let after = finish_tree(GroupNode::Node(after))?;
                    let params = match before {
                        Node::Identifier(s) => vec![s],
                        Node::List(l) => {
                            let mut r = vec![];
                            for i in l {
                                if let Node::Identifier(s) = i {
                                    r.push(s);
                                } else {
                                    return Err(TreeError::ColonLeftNotIdentifier(i))
                                }
                            }
                            r
                        },
                        _ => return Err(TreeError::ColonLeftNotIdentifier(before))
                    };
                    return Ok(Node::FunctionCreate(params, Box::new(after)))

                },
                x => {
                    return Err(TreeError::NoOperator(x))
                }
            }
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

fn next_split(nodes: Vec<GroupNode>) -> (Vec<GroupNode>, GroupNode, Vec<GroupNode>) {
    let mut max_binding = 0;
    let mut idx = 0;
    for (i, node) in nodes.iter().enumerate() {
        if let GroupNode::Token(token) = node {
            let binding = token.binding();
            let right = token.right_assoc();
            if binding > max_binding || (binding == max_binding && !right) {
                max_binding = binding;
                idx = i;
            }
        }
    }
    let before = &nodes[..idx];
    let pivot = &nodes[idx];
    let after = &nodes[(idx+1)..];
    (before.to_vec(), pivot.clone(), after.to_vec())
}

fn parentree(tokens: Vec<Token>) -> Result<GroupNode, TreeError> {
    let mut depth = 0;
    let mut subtree = vec![];
    let mut res = vec![];
    let mut last_token_op = true;
    for token in tokens {
        if token == Token::LParen {
            if depth > 0 {
                subtree.push(token);
            }
            depth += 1;
        } else if token == Token::RParen {
            depth -= 1;
            if depth < 0 {
                return Err(TreeError::NoLParen);
            } else if depth == 0 {
                if !last_token_op {
                    res.push(GroupNode::Token(Token::FunctionCall));
                }
                res.push(parentree(subtree)?);
                subtree = vec![];
                last_token_op = false;
            } else {
                subtree.push(token);
            }
        } else if depth > 0 {
            subtree.push(token);
        } else {
            let is_op = token.is_op();
            if last_token_op {
                if let Token::BinaryOp(BinaryOp::Sub) = token {
                    res.push(GroupNode::Token(Token::UnaryOp(UnaryOp::Neg)));
                } else {
                    res.push(GroupNode::Token(token));
                }
            } else {
                res.push(GroupNode::Token(token));
            }
            last_token_op = is_op;
        }
    }
    if depth > 0 {
        return Err(TreeError::NoRParen);
    }
    Ok(GroupNode::Node(res))
}
