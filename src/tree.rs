use crate::Value;
use crate::ops::*;
use crate::token::*;
use crate::function::FunctionError;

pub type Context = std::collections::HashMap<String, Value>;

#[derive(Clone, Debug)]
pub enum TreeError {
    NoLParen, NoRParen, TokenNoArgs(Token), AssignLeftInvalid, ColonLeftNotIdentifier(Node), NoOperator(Token)
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
    pub fn eval(&self, ctx: &mut Context) -> Result<Value, FunctionError> {
        match self {
            Self::Assign(name, value) => {
                let result = value.eval(ctx)?;
                ctx.insert(name.to_owned(), result);
                Ok(Value::Void)
            },
            Self::AssignOp(op, name, value) => {
                let name = name.to_owned();
                if ctx.contains_key(&name) {
                    let prev = ctx[&name].clone();
                    let result = value.eval(ctx)?;
                    let result = op.eval(prev, result)?;
                    ctx.insert(name.to_owned(), result);
                    Ok(Value::Void)
                } else {
                    Err(FunctionError::VariableUnset(name))
                }
            },
            Self::UnaryOp(op, rhs) => {
                op.eval(rhs.eval(ctx)?)
            },
            Self::BinaryOp(op, lhs, rhs) => {
                let lhs = lhs.eval(ctx)?;
                let rhs = rhs.eval(ctx)?;
                op.eval(lhs, rhs)
            },
            Self::Value(v) => Ok(v.clone()),
            Self::Identifier(s) => if ctx.contains_key(s) {
                Ok(ctx[s].clone())
            } else {
                Err(FunctionError::VariableUnset(s.to_owned()))
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

    pub fn simplify(&mut self) -> Option<Value> {
        match self {
            Self::Assign(_,value) => {
                if let Some(v) = value.simplify() {
                    *value = Box::new(Node::Value(v));
                }
                None
            },
            Self::AssignOp(_,_,value) => {
                if let Some(v) = value.simplify() {
                    *value = Box::new(Node::Value(v));
                }
                None
            },
            Self::UnaryOp(op, rhs) => {
                if let Some(x) = rhs.simplify() {
                    match op.eval(x.clone()) {
                        Ok(v) => Some(v),
                        Err(_) => {
                            *rhs = Box::new(Node::Value(x));
                            None
                        }
                    }
                } else {
                    None
                }
            },
            Self::BinaryOp(op, lhs, rhs) => {
                let l = lhs.simplify();
                let r = rhs.simplify();
                match (l, r) {
                    (Some(x), None) => {
                        *lhs = Box::new(Node::Value(x));
                        None
                    },
                    (None, Some(x)) => {
                        *rhs = Box::new(Node::Value(x));
                        None
                    },
                    (Some(a), Some(b)) => match op.eval(a.clone(), b.clone()) {
                        Ok(x) => Some(x),
                        Err(_) => {
                            *lhs = Box::new(Node::Value(a));
                            *rhs = Box::new(Node::Value(b));
                            None
                        }
                    },
                    _ => None
                }
            },
            Self::Value(v) => Some(v.clone()),
            Self::Identifier(_) => None,
            Self::List(v) => {
                let mut items = vec![];
                for i in 0..(v.len()) {
                    let val = v[i].clone().simplify();
                    if let Some(val) = val {
                        v[i] = Node::Value(val.clone());
                        items.push(val)
                    }
                }
                if items.len() == v.len() {
                    Some(Value::List(items))
                } else {
                    None
                }
            },
            Self::Block(v) => {
                for i in 0..(v.len()) {
                    let val = v[i].clone().simplify();
                    if let Some(val) = val {
                        v[i] = Node::Value(val.clone());
                    }
                }
                None
            },
            Self::FunctionCall(_,args) => {
                for i in 0..args.len() {
                    args[i].simplify();
                }
                None
            },
            Self::FunctionCreate(_,inner) => {
                inner.simplify();
                None
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum ParenTreeNode {
    Token(Token), Node(Vec<ParenTreeNode>)
}

impl std::fmt::Debug for ParenTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Token(a) => write!(f, "{:?}", a)?,
            Self::Node(v) => write!(f, "[{}]", 
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

fn finish_tree(root: ParenTreeNode) -> Result<Node, TreeError> {
    if let ParenTreeNode::Token(a) = root {
        match a {
            Token::BinaryOp(_) | Token::UnaryOp(_) | Token::Assign | Token::AssignOp(_)
            | Token::Comma | Token::Semicolon | Token::FunctionCall | Token::Colon
                => Err(TreeError::TokenNoArgs(a)),
            Token::Integer(n) => Ok(Node::Value(Value::Integer(n))),
            Token::Float(n) => Ok(Node::Value(Value::Float(n))),
            Token::Imaginary(n) => Ok(Node::Value(Value::from_complex(0., n))),
            Token::True => Ok(Node::Value(Value::Bool(true))),
            Token::False => Ok(Node::Value(Value::Bool(false))),
            Token::Identifier(s) => Ok(Node::Identifier(s)),
            Token::LParen | Token::RParen => unreachable!()
        }
    } else if let ParenTreeNode::Node(nodes) = root {
        if nodes.len() == 0 {
            return Ok(Node::List(vec![]));
        } else if nodes.len() == 1 {
            return finish_tree(nodes[0].clone());
        }
        let (before, pivot, after) = next_split(nodes);
        if let ParenTreeNode::Token(token) = pivot {
            match token {
                Token::LParen | Token::RParen => unreachable!(),
                Token::BinaryOp(op)
                    => return Ok(Node::BinaryOp(op, 
                            Box::new(finish_tree(ParenTreeNode::Node(before))?), 
                            Box::new(finish_tree(ParenTreeNode::Node(after))?))),
                Token::UnaryOp(op)
                    => return Ok(Node::UnaryOp(op,
                            Box::new(finish_tree(ParenTreeNode::Node(after))?))),
                Token::Assign
                    => if before.len() == 1 {
                        if let ParenTreeNode::Token(Token::Identifier(name)) = &before[0] {
                            return Ok(Node::Assign(name.to_owned(),
                                Box::new(finish_tree(ParenTreeNode::Node(after))?)))
                        } else {
                            return Err(TreeError::AssignLeftInvalid)
                        }
                    } else {
                        return Err(TreeError::AssignLeftInvalid)
                    },
                Token::AssignOp(op)
                    => if before.len() == 1 {
                        if let ParenTreeNode::Token(Token::Identifier(name)) = &before[0] {
                            return Ok(Node::AssignOp(op, name.to_owned(),
                                Box::new(finish_tree(ParenTreeNode::Node(after))?)))
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
                        finish_tree(ParenTreeNode::Node(before))?
                    };
                    let args = finish_tree(ParenTreeNode::Node(after))?;
                    if let Node::List(args) = args {
                        return Ok(Node::FunctionCall(Box::new(name), args))
                    } else {
                        return Ok(Node::FunctionCall(Box::new(name), vec![args]))
                    }
                },
                Token::Comma => {
                    let mut items = vec![];
                    items.push(finish_tree(ParenTreeNode::Node(before))?);
                    if after.len() > 0 {
                        let mut cdr = after;
                        loop {
                            let split = next_split(cdr.clone());
                            let (car, pivot) = (split.0, split.1);
                            let next_cdr = split.2;
                            if pivot == ParenTreeNode::Token(Token::Comma) {
                                items.push(finish_tree(ParenTreeNode::Node(car))?);
                                if next_cdr.len() == 0 {
                                    break
                                } else {
                                    cdr = next_cdr;
                                }
                            } else {
                                items.push(finish_tree(ParenTreeNode::Node(cdr))?);
                                break
                            }
                        }
                    }
                    return Ok(Node::List(items))
                },
                Token::Semicolon => {
                    let before = finish_tree(ParenTreeNode::Node(before))?;
                    let after = finish_tree(ParenTreeNode::Node(after))?;
                    if let Node::Block(mut v) = after {
                        v.insert(0, before);
                        return Ok(Node::Block(v));
                    } else {
                        return Ok(Node::Block(vec![before, after]));
                    }
                },
                Token::Colon => {
                    let before = finish_tree(ParenTreeNode::Node(before))?;
                    let after = finish_tree(ParenTreeNode::Node(after))?;
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
            println!("{:?}", pivot);
            todo!()
        }
    } else {
        unreachable!()
    }
}

fn next_split(nodes: Vec<ParenTreeNode>) -> (Vec<ParenTreeNode>, ParenTreeNode, Vec<ParenTreeNode>) {
    let mut max_binding = 0;
    let mut idx = 0;
    for (i, node) in nodes.iter().enumerate() {
        if let ParenTreeNode::Token(token) = node {
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

fn parentree(tokens: Vec<Token>) -> Result<ParenTreeNode, TreeError> {
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
                    res.push(ParenTreeNode::Token(Token::FunctionCall));
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
                    res.push(ParenTreeNode::Token(Token::UnaryOp(UnaryOp::Neg)));
                } else {
                    res.push(ParenTreeNode::Token(token));
                }
            } else {
                res.push(ParenTreeNode::Token(token));
            }
            last_token_op = is_op;
        }
    }
    if depth > 0 {
        return Err(TreeError::NoRParen);
    }
    Ok(ParenTreeNode::Node(res))
}
