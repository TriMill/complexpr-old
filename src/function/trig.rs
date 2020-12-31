use crate::function::*;
use crate::Value;

use crate::Context;
lazy_static::lazy_static! {
    pub static ref CTX_ALL: Context = {
        use crate::InsertFunction;
        let mut ctx = Context::new();
        ctx.insert_function("sin".to_owned(), &sin);
        ctx.insert_function("cos".to_owned(), &cos);
        ctx.insert_function("tan".to_owned(), &tan);
        ctx.insert_function("sinh".to_owned(), &sinh);
        ctx.insert_function("cosh".to_owned(), &cosh);
        ctx.insert_function("tanh".to_owned(), &tanh);
        ctx.insert_function("asin".to_owned(), &asin);
        ctx.insert_function("acos".to_owned(), &acos);
        ctx.insert_function("atan".to_owned(), &atan);
        ctx.insert_function("asinh".to_owned(), &asinh);
        ctx.insert_function("acosh".to_owned(), &acosh);
        ctx.insert_function("atanh".to_owned(), &atanh);
        ctx.insert_function("atan2".to_owned(), &atan2);
        ctx
    };
}

pub fn sin(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.sin())),
        Value::Complex(i) => Ok(Value::Complex(i.sin())),
        _ => unreachable!()
    }
}

pub fn cos(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.cos())),
        Value::Complex(i) => Ok(Value::Complex(i.cos())),
        _ => unreachable!()
    }
}

pub fn tan(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.tan())),
        Value::Complex(i) => Ok(Value::Complex(i.tan())),
        _ => unreachable!()
    }
}

pub fn sinh(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.sinh())),
        Value::Complex(i) => Ok(Value::Complex(i.sinh())),
        _ => unreachable!()
    }
}

pub fn cosh(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.cosh())),
        Value::Complex(i) => Ok(Value::Complex(i.cosh())),
        _ => unreachable!()
    }
}

pub fn tanh(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.tanh())),
        Value::Complex(i) => Ok(Value::Complex(i.tanh())),
        _ => unreachable!()
    }
}

pub fn asin(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.asin())),
        Value::Complex(i) => Ok(Value::Complex(i.asin())),
        _ => unreachable!()
    }
}

pub fn acos(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.acos())),
        Value::Complex(i) => Ok(Value::Complex(i.acos())),
        _ => unreachable!()
    }
}

pub fn atan(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.atan())),
        Value::Complex(i) => Ok(Value::Complex(i.atan())),
        _ => unreachable!()
    }
}

pub fn asinh(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.asinh())),
        Value::Complex(i) => Ok(Value::Complex(i.asinh())),
        _ => unreachable!()
    }
}

pub fn acosh(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.acosh())),
        Value::Complex(i) => Ok(Value::Complex(i.acosh())),
        _ => unreachable!()
    }
}

pub fn atanh(args: Vec<Value>) -> Result {
    bound_args(args.len(),1,1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(i) => Ok(Value::Float(i.atanh())),
        Value::Complex(i) => Ok(Value::Complex(i.atanh())),
        _ => unreachable!()
    }
}

pub fn atan2(args: Vec<Value>) -> Result {
    bound_args(args.len(),2,2)?;
    match to_float(args[0].clone())? {
        Value::Float(y) => match to_float(args[1].clone())? {
            Value::Float(x) => Ok(Value::Float(f64::atan2(y, x))),
            _ => unreachable!()
        },
        _ => unreachable!()
    }
}
