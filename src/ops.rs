use crate::Value;
use crate::function::{EvalError, EvalTrace};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod, Frac, Power,
    Greater, Less, GreaterEqual, LessEqual, Equal, NotEqual
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum UnaryOp {
    Neg
}

impl BinaryOp {
    pub fn eval(&self, lhs: Value, rhs: Value) -> Result<Value, EvalError> {
        let res = match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
            Self::Mod => lhs % rhs,
            Self::Power => lhs.pow(rhs),
            Self::Frac => lhs.frac(rhs),
            Self::Greater => Ok(Value::Bool(lhs > rhs)),
            Self::Less => Ok(Value::Bool(lhs < rhs)),
            Self::GreaterEqual => Ok(Value::Bool(lhs >= rhs)),
            Self::LessEqual => Ok(Value::Bool(lhs <= rhs)),
            Self::Equal => Ok(Value::Bool(lhs == rhs)),
            Self::NotEqual => Ok(Value::Bool(lhs != rhs))
        };
        match res {
            Ok(x) => Ok(x),
            Err(e) => Err(EvalError{kind: e.kind, trace: EvalTrace::Operator(self.to_string())})
        }
    }
}

impl UnaryOp {
    pub fn eval(&self, val: Value) -> Result<Value, EvalError> {
        let res = match self {
            Self::Neg => -val
        };
        match res {
            Ok(x) => Ok(x),
            Err(e) => Err(EvalError{kind: e.kind, trace: EvalTrace::Operator(self.to_string())})
        }
    }
}

impl ToString for UnaryOp {
    fn to_string(&self) -> String {
        match self {
            Self::Neg => "-",
        }.to_owned()
    }
}

impl ToString for BinaryOp {
    fn to_string(&self) -> String {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::Frac => "//",
            Self::Power => "^",
            Self::Greater => ">",
            Self::Less => "<",
            Self::GreaterEqual => ">=",
            Self::LessEqual => "<=",
            Self::Equal => "==",
            Self::NotEqual => "!="
        }.to_owned()
    }
}
