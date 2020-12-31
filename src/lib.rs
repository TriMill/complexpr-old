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
    Eval(function::FunctionError),
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
        Ok(tokens) => match tree::gen_tree(tokens) {
            Err(e) => Err(Error::Tree(e)),
            Ok(n) => {
                let mut n = Node::Block(vec![n]);
                n.simplify();
                Ok(n)
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
}

pub fn ctx_default() -> Context {
    DEFAULT_CONTEXT.clone()
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
