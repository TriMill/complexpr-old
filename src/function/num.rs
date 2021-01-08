use crate::function::*;
use crate::Value;
use crate::value::Ratio;

use crate::Context;
lazy_static::lazy_static! {
    pub static ref CTX_ALL: Context = {
        use crate::InsertFunction;
        let mut ctx = Context::new();
        ctx.insert_function("min".to_owned(), &min);
        ctx.insert_function("max".to_owned(), &max);
        ctx.insert_function("abs".to_owned(), &abs);
        ctx.insert_function("sqrt".to_owned(), &sqrt);
        ctx.insert_function("root".to_owned(), &root);
        ctx.insert_function("exp".to_owned(), &exp);
        ctx.insert_function("log".to_owned(), &log);
        ctx.insert_function("ln".to_owned(), &ln);
        ctx.insert_function("fract".to_owned(), &fract);
        ctx.insert_function("floor".to_owned(), &floor);
        ctx.insert_function("deg2rad".to_owned(), &deg2rad);
        ctx.insert_function("rad2deg".to_owned(), &rad2deg);
        ctx.insert_function("factorial".to_owned(), &factorial);
        ctx.insert_function("is_int".to_owned(), &is_int);
        ctx.insert_function("is_float".to_owned(), &is_float);
        ctx.insert_function("is_ratio".to_owned(), &is_ratio);
        ctx.insert_function("is_complex".to_owned(), &is_complex);
        ctx.insert_function("is_bool".to_owned(), &is_bool);
        ctx.insert_function("is_list".to_owned(), &is_list);
        ctx.insert_function("is_callable".to_owned(), &is_callable);
        ctx.insert_function("is_infinite".to_owned(), &is_infinite);
        ctx.insert_function("is_nan".to_owned(), &is_nan);
        ctx.insert_function("is_normal".to_owned(), &is_normal);
        ctx.insert_function("to_ratio".to_owned(), &to_ratio);
        ctx.insert("pi".to_owned(), PI.clone());
        ctx.insert("e".to_owned(), E.clone());
        ctx.insert("inf".to_owned(), INF.clone());
        ctx.insert("neg_inf".to_owned(), NEG_INF.clone());
        ctx.insert("nan".to_owned(), NAN.clone());
        ctx
    };
}

pub const PI: Value = Value::Float(std::f64::consts::PI);
pub const E: Value = Value::Float(std::f64::consts::E);
pub const INF: Value = Value::Float(std::f64::INFINITY);
pub const NEG_INF: Value = Value::Float(std::f64::NEG_INFINITY);
pub const NAN: Value = Value::Float(std::f64::NAN);

pub fn min(args: Vec<Value>) -> Result {
    min_args(args.len(), 1)?;
    let mut lowest = &args[0];
    for arg in &args[1..] {
        if arg < &lowest {
            lowest = arg;
        }
    }
    Ok(lowest.clone())
}

pub fn max(args: Vec<Value>) -> Result {
    min_args(args.len(), 1)?;
    let mut highest = &args[0];
    for arg in &args[1..] {
        if arg > &highest {
            highest = arg;
        }
    }
    Ok(highest.clone())
}

pub fn abs(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match to_float(args[0].clone())? {
        Value::Float(n) => Ok(Value::Float(n.abs())),
        _ => unreachable!()
    }
}

pub fn sqrt(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(n) => Ok(Value::Float(n.sqrt())),
        Value::Complex(n) => Ok(Value::Complex(n.sqrt())),
        _ => unreachable!()
    }
}

pub fn root(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    let pow = args[1].clone().pow(Value::Float(-1.))?;
    args[0].clone().pow(pow)
}

pub fn exp(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(n) => Ok(Value::Float(n.exp())),
        Value::Complex(n) => Ok(Value::Complex(n.exp())),
        _ => unreachable!()
    }
}

pub fn log(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 2)?;
    if args.len() == 1 {
        match to_float_or_complex(args[0].clone())? {
            Value::Float(n) => Ok(Value::Float(n.ln())),
            Value::Complex(n) => Ok(Value::Complex(n.ln())),
            _ => unreachable!()
        }
    } else {
        let num = to_float_or_complex(args[0].clone())?;
        let base = to_float_or_complex(args[1].clone())?;
        match (num, base) {
            (Value::Float(n), Value::Float(b))
                => if b == 10. {
                    Ok(Value::Float(n.log10()))
                } else if b == 2. {
                    Ok(Value::Float(n.log2()))
                } else {
                    Ok(Value::Float(n.log(b)))
                },
            (Value::Complex(n), Value::Complex(b)) 
                => Ok(Value::Complex(n.ln()/b.ln())),
            (Value::Complex(n), Value::Float(b)) 
                => Ok(Value::Complex(n.log(b))),
            (Value::Float(n), Value::Complex(b)) 
                => Ok(Value::Complex(n.ln()/b.ln())),
            _ => unreachable!()
        }
    }
}

pub fn ln(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 2)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(n) => Ok(Value::Float(n.ln())),
        Value::Complex(n) => Ok(Value::Complex(n.ln())),
        _ => unreachable!()
    }
}

pub fn fract(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match args[0] {
        Value::Integer(_) => Ok(Value::Integer(0)),
        Value::Float(n) => Ok(Value::Float(n.fract())),
        Value::Ratio(n) => Ok(Value::Ratio(n.fract())),
        Value::Complex(n) => Ok(Value::from_complex(n.re.fract(), n.im.fract())),
        _ => Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn floor(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match args[0] {
        Value::Integer(n) => Ok(Value::Integer(n)),
        Value::Float(n) => Ok(Value::Float(n.floor())),
        Value::Ratio(n) => Ok(Value::Ratio(n.floor())),
        Value::Complex(n) => Ok(Value::from_complex(n.re.floor(), n.im.floor())),
        _ => Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

const DEG2RAD: f64 = std::f64::consts::PI/180.;
const RAD2DEG: f64 = 180./std::f64::consts::PI;
pub fn deg2rad(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match to_float(args[0].clone())? {
        Value::Float(n) => Ok(Value::Float(n*DEG2RAD)),
        _ => unreachable!()
    }
}

pub fn rad2deg(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match to_float(args[0].clone())? {
        Value::Float(n) => Ok(Value::Float(n*RAD2DEG)),
        _ => unreachable!()
    }
}

pub fn factorial(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match args[0] {
        Value::Integer(n) if n >= 0 => Ok(Value::Integer((1..(n+1)).fold(1, |a,b|a*b))),
        Value::Integer(n) => Err(EvalErrorKind::WrongArgValue(Value::Integer(n)).into()),
        _ => Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn is_float(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_float()))
}
pub fn is_int(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_int()))
}
pub fn is_complex(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_complex()))
}
pub fn is_ratio(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_ratio()))
}
pub fn is_bool(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_bool()))
}
pub fn is_list(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_list()))
}
pub fn is_str(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_str()))
}
pub fn is_callable(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_callable()))
}
pub fn is_void(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_void()))
}

pub fn is_infinite(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    return Ok(Value::Bool(match args[0] {
        Value::Float(f) => f.is_infinite(),
        Value::Complex(c) => c.is_infinite(),
        _ => false
    }))
}
pub fn is_nan(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    return Ok(Value::Bool(match args[0] {
        Value::Float(f) => f.is_nan(),
        Value::Complex(c) => c.is_nan(),
        _ => false
    }))
}
pub fn is_normal(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    return Ok(Value::Bool(match args[0] {
        Value::Float(f) => !f.is_nan() && !f.is_infinite(),
        Value::Complex(c) => !c.is_nan() && !c.is_infinite(),
        Value::Ratio(_) | Value::Integer(_) => true,
        _ => false
    }))
}

pub fn to_ratio(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::Ratio(_) = args[0] {
        Ok(args[0].clone())
    } else if let Value::Integer(i) = args[0] {
        Ok(Value::from_ratio(i, 1))
    } else if let Value::Float(f) = args[0] {
        match Ratio::approximate_float(f) {
            Some(x) => Ok(Value::Ratio(x)),
            None => Err(EvalErrorKind::WrongArgValue(args[0].clone()).into())
        }
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}
