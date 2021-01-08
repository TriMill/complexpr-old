use crate::function::*;
use crate::Value;

use crate::Context;
lazy_static::lazy_static! {
    pub static ref CTX_ALL: Context = {
        use crate::InsertFunction;
        let mut ctx = Context::new();
        ctx.insert_function("add".to_owned(), &add);
        ctx.insert_function("sub".to_owned(), &sub);
        ctx.insert_function("mul".to_owned(), &mul);
        ctx.insert_function("div".to_owned(), &div);
        ctx.insert_function("frac".to_owned(), &frac);
        ctx.insert_function("mod".to_owned(), &modulo);
        ctx.insert_function("pow".to_owned(), &pow);
        ctx.insert_function("cmp".to_owned(), &cmp);
        ctx
    };
}

pub fn add(args: Vec<Value>) -> Result {
    if args.len() == 0 {
        Ok(Value::Integer(0))
    } else {
        let mut res = args[0].clone();
        for arg in &args[1..] {
            res = (res + arg.clone())?;
        }
        Ok(res)
    }
}

pub fn sub(args: Vec<Value>) -> Result {
    if args.len() == 0 {
        Ok(Value::Integer(0))
    } else {
        let mut res = args[0].clone();
        for arg in &args[1..] {
            res = (res - arg.clone())?;
        }
        Ok(res)
    }
}

pub fn mul(args: Vec<Value>) -> Result {
    if args.len() == 0 {
        Ok(Value::Integer(1))
    } else {
        let mut res = args[0].clone();
        for arg in &args[1..] {
            res = (res * arg.clone())?;
        }
        Ok(res)
    }
}

pub fn div(args: Vec<Value>) -> Result {
    if args.len() == 0 {
        Ok(Value::Integer(1))
    } else {
        let mut res = args[0].clone();
        for arg in &args[1..] {
            res = (res / arg.clone())?;
        }
        Ok(res)
    }
}

pub fn frac(args: Vec<Value>) -> Result {
    if args.len() == 0 {
        Ok(Value::Integer(1))
    } else {
        let mut res = args[0].clone();
        for arg in &args[1..] {
            res = res.frac(arg.clone())?;
        }
        Ok(res)
    }
}

pub fn modulo(args: Vec<Value>) -> Result {
    if args.len() == 0 {
        Ok(Value::Integer(1))
    } else {
        let mut res = args[0].clone();
        for arg in &args[1..] {
            res = (res % arg.clone())?;
        }
        Ok(res)
    }
}

pub fn pow(args: Vec<Value>) -> Result {
    if args.len() == 0 {
        Ok(Value::Integer(1))
    } else {
        let mut res = args[args.len()-1].clone();
        for arg in args[..(args.len()-1)].iter().rev() {
            res = arg.clone().pow(res)?;
        }
        Ok(res)
    }
}

pub fn cmp(args: Vec<Value>) -> Result {
    use std::cmp::Ordering;
    bound_args(args.len(), 2, 2)?;
    Ok(match args[0].partial_cmp(&args[1]) {
        Some(Ordering::Greater) => Value::Integer(1),
        Some(Ordering::Equal) => Value::Integer(0),
        Some(Ordering::Less) => Value::Integer(-1),
        None => Value::Void,
    })
}
