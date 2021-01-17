pub mod ops;
pub mod trig;
pub mod num;
pub mod types;
pub mod util;
pub mod complex;
#[cfg(feature = "random")]
pub mod random;
pub mod io;
use crate::Value;
use std::sync::Arc;

#[derive(Clone, Debug)]
/// The type of a [`EvalError`], including its parameters.
pub enum EvalErrorKind {
    TooFewArgs{min: usize, count: usize}, TooManyArgs{max: usize, count: usize}, 
    AssignLeftNotIdent(Value), WrongFunc(Value),
    IdentifierReserved(String), InvalidSpecialIdent(String),
    VariableUnset(String),
    WrongArgType(Value), WrongOpArgTypes(Value, Value), WrongArgValue(Value), ListOutOfBounds(i64),
    IOError(Arc<std::io::Error>),
    Other(String)
}

impl From<EvalErrorKind> for EvalError {
    fn from(kind: EvalErrorKind) -> EvalError {
        Self { kind, trace: EvalTrace::None }
    }
}

#[derive(Clone, Debug)]
pub enum EvalTrace {
    Function(String), Operator(String), Manual, None
}

impl std::fmt::Display for EvalTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Function(s)
                => write!(f, "Function '{}': ", s),
            Self::Operator(a)
                => write!(f, "Operator '{}': ", a),
            Self::Manual
                => write!(f, "Induced manually: "),
            Self::None => Ok(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct EvalError {
    pub kind: EvalErrorKind,
    pub trace: EvalTrace
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.trace)?;
        match &self.kind {
            EvalErrorKind::TooFewArgs{min, count} 
                => write!(f, "Too few arguments (expected {}, found {})", min, count), 
            EvalErrorKind::TooManyArgs{max, count} 
                => write!(f, "Too many arguments (expected {}, found {})", max, count), 
            EvalErrorKind::AssignLeftNotIdent(val)
                => write!(f, "LHS of assignment must be an identifier, found '{}'", val.get_type()),
            EvalErrorKind::IdentifierReserved(name)
                => write!(f, "Identifier '{}' is reserved", name),
            EvalErrorKind::InvalidSpecialIdent(name)
                => write!(f, "Special identifier '{}' does not exist", name),
            EvalErrorKind::WrongFunc(val) 
                => write!(f, "'{}' is not a function", val),
            EvalErrorKind::VariableUnset(s)
                => write!(f, "Variable '{}' has not been initialized", s),
            EvalErrorKind::WrongArgType(a)
                => write!(f, "Argument '{}' is of the wrong type", a),
            EvalErrorKind::WrongOpArgTypes(a, b)
                => write!(f, "Operator arguments '{}' and '{}' are of the wrong types", a.get_type(), b.get_type()),
            EvalErrorKind::WrongArgValue(a)
                => write!(f, "Argument '{}' has an invalid value", a),
            EvalErrorKind::ListOutOfBounds(i)
                => write!(f, "List index {} out of bounds", i),
            EvalErrorKind::IOError(e)
                => write!(f, "IO Error: {:?}", e),
            EvalErrorKind::Other(s)
                => write!(f, "{}", s)
        }
    }
}

pub type Result = std::result::Result<Value, EvalError>;

pub type Fp = dyn Fn(Vec<Value>) -> Result + 'static + Sync + Send;

#[derive(Clone)]
pub struct Function (
    pub Arc<Fp>
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

pub fn min_args(count: usize, min: usize) -> std::result::Result<(), EvalError> {
    if count < min {
        Err(EvalErrorKind::TooFewArgs{min, count}.into())
    } else {
        Ok(())
    }
}

pub fn max_args(count: usize, max: usize) -> std::result::Result<(), EvalError> {
    if count > max {
        Err(EvalErrorKind::TooManyArgs{max, count}.into())
    } else {
        Ok(())
    }
}

pub fn bound_args(count: usize, min: usize, max: usize) -> std::result::Result<(), EvalError> {
    min_args(count, min)?;
    max_args(count, max)
}

pub fn to_float(val: Value) -> Result {
    use Value::*;
    match val {
        Integer(i) => Ok(Float(i as f64)),
        Float(i) => Ok(Float(i)),
        Ratio(i) => Ok(Float((*i.numer() as f64)/(*i.denom() as f64))),
        _ => Err(EvalErrorKind::WrongArgType(val).into())
    }
}

pub fn to_float_or_complex(val: Value) -> Result {
    use Value::*;
    match val {
        Integer(i) => Ok(Float(i as f64)),
        Float(i) => Ok(Float(i)),
        Ratio(i) => Ok(Float((*i.numer() as f64)/(*i.denom() as f64))),
        Complex(i) => Ok(Complex(i)),
        _ => Err(EvalErrorKind::WrongArgType(val).into())
    }
}
