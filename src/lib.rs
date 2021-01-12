//! Evaluate expressions, with support for complex numbers, fractions, many builtin functions,
//! lambda functions, and assignment.
pub mod value;
pub mod token;
pub mod tree;
pub mod ops;
pub mod function;
pub use value::Value;
pub use tree::Context;
pub use tree::Node;

#[derive(Clone, Debug)]
/// An error type encompassing errors from tokenization, tree creation, and evaluation.
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

/// Evaluate an expression with the given context. This may mutate the context.
pub fn eval(expr: &str, ctx: &mut Context) -> Result<Value, Error> {
    let node = compile(expr)?;
    match node.eval(ctx) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::Eval(e))
    }
}

/// Evaluate an expression with the default context. The context is cloned each time
/// this function is used, so if mutation of the context is not a concern prefer [`eval`].
pub fn eval_default(expr: &str) -> Result<Value, Error> {
    let node = compile(expr)?;
    match node.eval(&mut ctx_default()) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::Eval(e))
    }
}

/// Compile an expression to a [`Node`] so it can be evaluated later. This is considerably faster
/// than evaluating the same expression multiple times.
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
        for (k, v) in function::types::CTX_ALL.iter() {
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

/// Create a clone of the default context, including all modules except for [`function::io`].
pub fn ctx_default() -> Context {
    DEFAULT_CONTEXT.clone()
}
/// Create a clone of the full context, including all modules.
pub fn ctx_full() -> Context {
    FULL_CONTEXT.clone()
}
/// Create an empty context.
pub fn ctx_empty() -> Context {
    Context::new()
}

pub trait InsertFunction {
    /// Insert a function into this [`Context`]. Creates a new [`std::sync::Arc`] and [`function::Function`].
    fn insert_function(&mut self, k: String, func: &'static function::Fp);
}

impl InsertFunction for Context {
    fn insert_function(&mut self, k: String, func: &'static function::Fp) {
        self.insert(k, Value::Function(function::Function(std::sync::Arc::new(func.clone()))));
    }
}
