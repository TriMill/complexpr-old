//! Can be disabled with the `random` feature flag

use crate::function::*;
use crate::Value;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::Context;
lazy_static::lazy_static! {
    /// A `lazy_static` [`Context`] containing all the definitions from [`rand`]
    pub static ref CTX_ALL: Context = {
        use crate::InsertFunction;
        let mut ctx = Context::new();
        ctx.insert_function("random".to_owned(), &random);
        ctx.insert_function("random_range".to_owned(), &random_range);
        ctx.insert_function("random_choose".to_owned(), &random_choose);
        ctx.insert_function("shuffle".to_owned(), &shuffle);
        ctx
    };
}

pub fn random(args: Vec<Value>) -> Result {
    max_args(args.len(), 0)?;
    Ok(Value::Float(rand::random()))
}

pub fn random_range(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 2)?;
    if args.len() == 1 {
        if let Value::Integer(max) = &args[0] {
            if max <= &0 {
                return Ok(Value::Void)
            }
            Ok(Value::Integer(rand::thread_rng().gen_range(0..*max)))
        } else {
            Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
        }
    } else {
        if let Value::Integer(min) = &args[0] {
            if let Value::Integer(max) = &args[1] {
                if min >= max {
                    return Ok(Value::Void)
                }
                Ok(Value::Integer(rand::thread_rng().gen_range(*min..*max)))
            } else {
                Err(EvalErrorKind::WrongArgType(args[1].clone()).into())
            }
        } else {
            Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
        }
    }
}

pub fn random_choose(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::List(l) = &args[0] {
        if l.len() == 0 {
            return Ok(Value::Void)
        }
        let idx = rand::thread_rng().gen_range(0..l.len());
        Ok(l[idx].clone())
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn shuffle(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::List(l) = &args[0] {
        let mut l = l.clone();
        l.shuffle(&mut rand::thread_rng());
        Ok(Value::List(l))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}
