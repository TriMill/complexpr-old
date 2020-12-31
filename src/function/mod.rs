pub mod ops;
pub mod trig;
pub mod num;
pub mod util;
pub mod complex;
use crate::Value;

#[derive(Clone, Debug)]
pub enum FunctionError {
    TooFewArgs{min: usize, count: usize}, TooManyArgs{max: usize, count: usize}, 
    AssignLeftNotIdent(Value), WrongFunc(Value),
    VariableUnset(String),
    WrongArgTypes(Vec<Value>), WrongArgValue(Value), ListOutOfBounds(i64),
    Other(String)
}

pub type Result = std::result::Result<Value, FunctionError>;

pub type Fp = dyn Fn(Vec<Value>) -> Result + 'static + Sync + Send;

#[derive(Clone)]
pub struct Function (
    pub std::sync::Arc<Fp>
);

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<function>")?;
        Ok(())
    }
}

pub fn func_true(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    Ok(args[0].clone())
}

pub fn func_false(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    Ok(args[1].clone())
}

pub fn min_args(count: usize, min: usize) -> std::result::Result<(), FunctionError> {
    if count < min {
        Err(FunctionError::TooFewArgs{min, count})
    } else {
        Ok(())
    }
}

pub fn max_args(count: usize, max: usize) -> std::result::Result<(), FunctionError> {
    if count > max {
        Err(FunctionError::TooManyArgs{max, count})
    } else {
        Ok(())
    }
}

pub fn bound_args(count: usize, min: usize, max: usize) -> std::result::Result<(), FunctionError> {
    min_args(count, min)?;
    max_args(count, max)
}

pub fn to_float(val: Value) -> Result {
    use Value::*;
    match val {
        Integer(i) => Ok(Float(i as f64)),
        Float(i) => Ok(Float(i)),
        Ratio(i) => Ok(Float((*i.numer() as f64)/(*i.denom() as f64))),
        _ => Err(FunctionError::WrongArgTypes(vec![val]))
    }
}

pub fn to_float_or_complex(val: Value) -> Result {
    use Value::*;
    match val {
        Integer(i) => Ok(Float(i as f64)),
        Float(i) => Ok(Float(i)),
        Ratio(i) => Ok(Float((*i.numer() as f64)/(*i.denom() as f64))),
        Complex(i) => Ok(Complex(i)),
        _ => Err(FunctionError::WrongArgTypes(vec![val]))
    }
}
