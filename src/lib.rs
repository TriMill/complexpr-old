pub mod value;
pub mod token;
pub mod tree;
pub mod ops;
pub mod function;
pub use value::Value;
pub use tree::Context;
pub use tree::Node;

#[derive(Clone, Debug)]
pub enum Error {
    Tokenize(token::TokenizeError),
    Tree(tree::TreeError),
    Eval(function::EvalError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Tokenize(e) => write!(f, "{}", e),
            Self::Tree(e) => write!(f, "{}", e),
            Self::Eval(e) => write!(f, "{}", e),
        }
    }
}

pub fn eval(expr: &str, ctx: &mut Context) -> Result<Value, Error> {
    let node = compile(expr)?;
    match node.eval(ctx) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::Eval(e))
    }
}

pub fn eval_str(expr: &str) -> Result<Value, Error> {
    let node = compile(expr)?;
    match node.eval(&mut ctx_default()) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::Eval(e))
    }
}

pub fn compile(expr: &str) -> Result<Node, Error> {
    match token::tokenize(expr) {
        Err(e) => Err(Error::Tokenize(e)),
        Ok(tokens) => {
            match tree::gen_tree(tokens) {
                Err(e) => Err(Error::Tree(e)),
                Ok(n) => {
                    Ok(n)
                }
            }
        }
    }
}

use lazy_static::lazy_static;
lazy_static! {
    static ref DEFAULT_CONTEXT: Context = {
        let mut ctx = Context::new();
        for (k, v) in function::ops::CTX_ALL.iter() {
            ctx.insert(k.to_owned(), v.clone());
        }
        for (k, v) in function::trig::CTX_ALL.iter() {
            ctx.insert(k.to_owned(), v.clone());
        }
        for (k, v) in function::num::CTX_ALL.iter() {
            ctx.insert(k.to_owned(), v.clone());
        }
        for (k, v) in function::util::CTX_ALL.iter() {
            ctx.insert(k.to_owned(), v.clone());
        }
        for (k, v) in function::complex::CTX_ALL.iter() {
            ctx.insert(k.to_owned(), v.clone());
        }
        ctx
    };
    static ref FULL_CONTEXT: Context = {
        let mut ctx = DEFAULT_CONTEXT.clone();
        for (k, v) in function::io::CTX_ALL.iter() {
            ctx.insert(k.to_owned(), v.clone());
        }
        ctx
    };
}

pub fn ctx_default() -> Context {
    DEFAULT_CONTEXT.clone()
}
pub fn ctx_full() -> Context {
    FULL_CONTEXT.clone()
}
pub fn ctx_empty() -> Context {
    Context::new()
}

pub trait InsertFunction {
    fn insert_function(&mut self, k: String, func: &'static function::Fp);
}

impl InsertFunction for Context {
    fn insert_function(&mut self, k: String, func: &'static function::Fp) {
        self.insert(k, Value::Function(function::Function(std::sync::Arc::new(func.clone()))));
    }
}
