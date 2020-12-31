use crate::function::*;
use crate::Value;

use crate::Context;
lazy_static::lazy_static! {
    pub static ref CTX_ALL: Context = {
        use crate::InsertFunction;
        let mut ctx = Context::new();
        ctx.insert_function("map".to_owned(), &map);
        ctx.insert_function("index".to_owned(), &map);
        ctx.insert_function("apply".to_owned(), &map);
        ctx
    };
}

pub fn map(args: Vec<Value>) -> Result {
    min_args(args.len(), 1)?;
    let first = &args[0];
    let fns = &args[1..];
    if let Value::List(v) = first {
        let mut list = v.to_vec();
        for f in fns {
            list = {
                let mut newlist = vec![];
                for v in list {
                    newlist.push(f.eval(vec![v])?);
                }
                newlist
            };
        }
        Ok(Value::List(list))
    } else {
        Err(FunctionError::WrongArgTypes(args))
    }
}

pub fn index(args: Vec<Value>) -> Result {
    use std::convert::TryInto;
    bound_args(args.len(), 2, 2)?;
    let list = &args[0];
    let idx = &args[1];
    match (list, idx) {
        (Value::List(l), Value::Integer(i)) => {
            if *i >= 0 && *i < l.len().try_into().unwrap_or(-1) {
                Ok(l[*i as usize].clone())
            } else {
                Err(FunctionError::ListOutOfBounds(*i))
            }
        }
        _ => Err(FunctionError::WrongArgTypes(args))
    }
}


pub fn apply(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    let func = &args[0];
    let list = &args[1];
    match list {
        Value::List(l) => func.eval(l.to_vec()),
        _ => Err(FunctionError::WrongArgTypes(args))
    }
}
