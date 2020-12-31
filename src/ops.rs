use crate::Value;
use crate::function::FunctionError;

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
    pub fn eval(&self, lhs: Value, rhs: Value) -> Result<Value, FunctionError> {
        match self {
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
        }
    }
}

impl UnaryOp {
    pub fn eval(&self, val: Value) -> Result<Value, FunctionError> {
        match self {
            Self::Neg => -val
        }
    }
}
