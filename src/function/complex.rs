use crate::function::*;
use crate::Value;

use crate::Context;
lazy_static::lazy_static! {
    pub static ref CTX_ALL: Context = {
        use crate::InsertFunction;
        let mut ctx = Context::new();
        ctx.insert_function("re".to_owned(), &re);
        ctx.insert_function("im".to_owned(), &im);
        ctx.insert_function("conj".to_owned(), &conj);
        ctx.insert_function("arg".to_owned(), &arg);
        ctx.insert_function("norm".to_owned(), &norm);
        ctx.insert_function("norm_sq".to_owned(), &norm_sq);
        ctx.insert_function("normalize".to_owned(), &normalize);
        ctx.insert_function("to_polar".to_owned(), &to_polar);
        ctx.insert_function("from_polar".to_owned(), &from_polar);
        ctx.insert_function("all_roots".to_owned(), &all_roots);
        ctx
    };
}

pub fn re(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::Complex(c) = args[0] {
        Ok(Value::Float(c.re))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn im(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::Complex(c) = args[0] {
        Ok(Value::Float(c.im))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn conj(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::Complex(c) = args[0] {
        Ok(Value::Complex(c.conj()))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn arg(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::Complex(c) = args[0] {
        Ok(Value::Float(c.arg()))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn norm(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::Complex(c) = args[0] {
        Ok(Value::Float(c.norm()))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn norm_sq(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::Complex(c) = args[0] {
        Ok(Value::Float(c.norm_sqr()))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn normalize(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::Complex(c) = args[0] {
        Ok(Value::Complex(c / c.norm()))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn to_polar(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::Complex(c) = args[0] {
        let polar = c.to_polar();
        Ok(Value::List(vec![
           Value::Float(polar.0),
           Value::Float(polar.1)
        ]))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn from_polar(args: Vec<Value>) -> Result {
    use crate::value::Complex;
    bound_args(args.len(), 1, 2)?;
    let (r, theta);
    if args.len() == 1 {
        if let Value::List(l) = &args[0] {
            if l.len() == 2 {
                r = l[0].clone();
                theta = l[1].clone();
            } else {
                return Err(EvalErrorKind::ListOutOfBounds(1).into())
            }
        } else {
            return Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
        }
    } else {
        r = args[0].clone();
        theta = args[1].clone();
    }
    match (to_float(r)?, to_float(theta)?) {
        (Value::Float(r), Value::Float(theta)) 
            => Ok(Value::Complex(Complex::from_polar(r, theta))),
        _ => unreachable!()
    }
}

// list of all roots
// all_roots(c, n) => all nth roots of c
pub fn all_roots(args: Vec<Value>) -> Result {
    use crate::value::Complex;
    use std::f64::consts::TAU;
    bound_args(args.len(), 2, 2)?;
    match (&args[0], &args[1]) {
        (Value::Complex(c), Value::Integer(n)) => {
            let n_recip = (*n as f64).recip();
            let (r, theta) = c.to_polar();
            let res_r = r.powf(n_recip);
            let res = (0..*n)
                .map(|k| (k as f64)*TAU/(*n as f64) + theta)
                .map(|t| Complex::new(
                        res_r*cos(t),
                        res_r*sin(t)))
                .map(|c| Value::Complex(c))
                .collect();
            Ok(Value::List(res))
        },
        (Value::Complex(_), x) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
        (x,_) => Err(EvalErrorKind::WrongArgType(x.clone()).into())
    }
}

fn cos(n: f64) -> f64 {
    let (cos, sin) = (n.cos(), n.sin());
    0.5*cos + 0.5*cos.signum()*(1. - sin*sin).sqrt()
}

fn sin(n: f64) -> f64 {
    let (cos, sin) = (n.cos(), n.sin());
    0.5*sin + 0.5*sin.signum()*(1. - cos*cos).sqrt()
}
