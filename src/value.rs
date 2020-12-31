use num::rational;
use num::complex;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use std::cmp::Ordering;
use crate::function::{self, Function, FunctionError};
pub type Complex = complex::Complex<f64>;
pub type Ratio = rational::Ratio<i64>;
pub type List = Vec<Value>;

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Complex(Complex),
    Ratio(Ratio),
    Bool(bool),
    List(List),
    Function(Function),
    Lambda{args: Vec<String>, func: Box<crate::tree::Node>, ctx: Box<crate::Context>},
    Void
}

pub fn r2f64(r: &Ratio) -> f64 {
    (*r.numer() as f64)/(*r.denom() as f64)
}

fn sort<'a>(a: &'a Value, b: &'a Value) -> (&'a Value, &'a Value, bool) {
    use Value::*;
    if let Integer(_) = a {
        (a,b,false)
    } else if let Integer(_) = b {
        (b,a,true)
    } else if let Float(_) = a {
        (a,b,false)
    } else if let Float(_) = b {
        (b,a,true)
    } else if let Complex(_) = a {
        (a,b,false)
    } else if let Complex(_) = b {
        (b,a,true)
    } else if let Ratio(_) = a {
        (a,b,false)
    } else if let Ratio(_) = b {
        (b,a,true)
    } else if let Bool(_) = a {
        (a,b,false)
    } else if let Bool(_) = b {
        (b,a,true)
    } else if let List(_) = a {
        (a,b,false)
    } else if let List(_) = b {
        (b,a,true)
    } else if let Function(_) = a {
        (a,b,false)
    } else if let Function(_) = b {
        (b,a,true)
    } else if let Lambda{..} = a {
        (a,b,false)
    } else if let Lambda{..} = b {
        (b,a,true)
    } else {
        (a,b,false)
    }
}

impl Value {
    pub fn from_complex(re: f64, im: f64) -> Self {
        Self::Complex(Complex::new(re, im))
    }
    pub fn from_ratio(numer: i64, denom: i64) -> Self {
        Self::Ratio(Ratio::new_raw(numer, denom))
    }
    pub fn eval(&self, args: Vec<Self>) -> Result<Self, FunctionError> {
        match self {
            Self::Function(f) => f.0(args),
            Self::Lambda{args: argnames, func, ctx} => {
                function::bound_args(argnames.len(), args.len(), args.len())?;
                let mut ctx = ctx.clone();
                for i in 0..args.len() {
                    ctx.insert(argnames[i].to_string(), args[i].clone());
                }
                func.eval(&mut ctx)
            }
            Self::Bool(true) => function::func_true(args),
            Self::Bool(false) => function::func_false(args),
            _ => Err(FunctionError::WrongFunc(self.clone()))
        }
    }
}

impl PartialOrd<Value> for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Value::*;
        match (self, other) {
            (Integer(a), Integer(b)) => Some(a.cmp(&b)),
            (Float(a), Integer(b)) => a.partial_cmp(&(*b as f64)),
            (Integer(a), Float(b)) => (*a as f64).partial_cmp(&b),
            (Float(a), Float(b)) => a.partial_cmp(&b),
            (Ratio(a), Integer(b)) => Some(a.cmp(&num::rational::Ratio::from(*b))),
            (Ratio(a), Float(b)) => r2f64(a).partial_cmp(&b),
            (Integer(a), Ratio(b)) => Some(num::rational::Ratio::from(*a).cmp(&b)),
            (Float(a), Ratio(b)) => a.partial_cmp(&r2f64(b)),
            (Ratio(a), Ratio(b)) => Some(a.cmp(b)),
            (Bool(a), Bool(b)) => Some(a.cmp(b)),
            (_,_) => None,
        }
    }
}

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        use num::Zero;
        let sorted = sort(self, other);
        match (sorted.0, sorted.1) {
            (Integer(a), Integer(b)) => a == b,
            (Integer(a), Float(b)) => (*a as f64) == *b,
            (Float(a), Float(b)) => a == b,
            (Integer(a), Ratio(b)) => num::rational::Ratio::new(*a, 1) == *b,
            (Float(a), Ratio(b)) => *a == r2f64(b),
            (Ratio(a), Ratio(b)) => a == b,
            (Complex(a), Integer(b)) => a.re == (*b as f64) && a.im.is_zero(),
            (Complex(a), Float(b)) => a.re == *b && a.im.is_zero(),
            (Complex(a), Ratio(b)) => a.re == r2f64(b) && a.im.is_zero(),
            (Complex(a), Complex(b)) => a == b,
            (List(a), List(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            (Void, Void) => true,
            (_,_) => false,
        }
    }
}

impl Add<Value> for Value {
    type Output = Result<Value, FunctionError>;
    fn add(self, rhs: Value) -> Self::Output {
        use Value::*;
        // Integer, Float, Complex, Ratio, Bool, List, Void
        match sort(&self, &rhs) {
            (Integer(a), Integer(b),_) => Ok(Integer(a+b)),
            (Float(a), Float(b),_) => Ok(Float(a+b)),
            (Integer(a), Float(b),_) => Ok(Float(*a as f64 + b)),
            (Integer(a), Complex(b),_) => Ok(Complex(*a as f64 + b)),
            (Float(a), Complex(b),_) => Ok(Complex(a + b)),
            (Complex(a), Complex(b),_) => Ok(Complex(a + b)),
            (Integer(a), Ratio(b),_) => Ok(Ratio(b + a)),
            (Float(a), Ratio(b),_) => Ok(Float(a + r2f64(b))),
            (Complex(a), Ratio(b),_) => Ok(Complex(a + r2f64(b))),
            (Ratio(a), Ratio(b),_) => Ok(Ratio(a + b)),
            (Bool(a), Bool(b),_) => Ok(Bool(a | b)),
            (List(a), List(b),_) => Ok(List(a.iter().chain(b.iter()).cloned().collect())),
            (_,_,_) => Err(FunctionError::WrongArgTypes(vec![self, rhs]))
        }
    }
}

impl Sub<Value> for Value {
    type Output = Result<Value, FunctionError>;
    fn sub(self, rhs: Value) -> Self::Output {
        use Value::*;
        // Integer, Float, Complex, Ratio, Bool, List, Void
        let sorted = sort(&self, &rhs);
        let (left, right) = (sorted.0, sorted.1);
        let mul = match sorted.2 {
            false => (1, 1.0),
            true => (-1, -1.0)
        };
        match (left, right) {
            (Integer(a), Integer(b)) => Ok(Integer(mul.0*(a - b))),
            (Float(a), Float(b)) => Ok(Float(mul.1*(a - b))),
            (Integer(a), Float(b)) => Ok(Float(mul.1*(*a as f64 - b))),
            (Integer(a), Complex(b)) => Ok(Complex(mul.1*(*a as f64 - b))),
            (Float(a), Complex(b)) => Ok(Complex(mul.1*(a - b))),
            (Complex(a), Complex(b)) => Ok(Complex(mul.1*(a - b))),
            (Integer(a), Ratio(b)) => Ok(Ratio((b - a)*(-mul.0))),
            (Float(a), Ratio(b)) => Ok(Float(mul.1*(a + r2f64(b)))),
            (Complex(a), Ratio(b)) => Ok(Complex(mul.1*(a + r2f64(b)))),
            (Ratio(a), Ratio(b)) => Ok(Ratio((a + b)*mul.0)),
            (_,_) => Err(FunctionError::WrongArgTypes(vec![self, rhs]))
        }
    }
}

impl Mul<Value> for Value {
    type Output = Result<Value, FunctionError>;
    fn mul(self, rhs: Value) -> Self::Output {
        use Value::*;
        // Integer, Float, Complex, Ratio, Bool, List, Void
        match sort(&self, &rhs) {
            (Integer(a), Integer(b),_) => Ok(Integer(a * b)),
            (Float(a), Float(b),_) => Ok(Float(a * b)),
            (Integer(a), Float(b),_) => Ok(Float((*a as f64) * b)),
            (Integer(a), Complex(b),_) => Ok(Complex((*a as f64) * b)),
            (Float(a), Complex(b),_) => Ok(Complex(a * b)),
            (Complex(a), Complex(b),_) => Ok(Complex(a * b)),
            (Integer(a), Ratio(b),_) => Ok(Ratio(b * a)),
            (Float(a), Ratio(b),_) => Ok(Float(a * r2f64(b))),
            (Complex(a), Ratio(b),_) => Ok(Complex(a * r2f64(b))),
            (Ratio(a), Ratio(b),_) => Ok(Ratio(a * b)),
            (Bool(a), Bool(b),_) => Ok(Bool(a & b)),
            (_,_,_) => Err(FunctionError::WrongArgTypes(vec![self, rhs])),
        }
    }
}

impl Div<Value> for Value {
    type Output = Result<Value, FunctionError>;
    fn div(self, rhs: Value) -> Self::Output {
        use Value::*;
        use num::Zero;
        match (self, rhs) {
            (Integer(a), Integer(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Integer(b)))
            } else {
                Ok(Float((a as f64)/(b as f64)))
            },
            (Float(a), Float(b)) => Ok(Float(a / b)),
            (Integer(a), Float(b)) => Ok(Float((a as f64)/b)),
            (Float(a), Integer(b)) => Ok(Float(a/(b as f64))),
            (Integer(a), Complex(b)) => Ok(Complex((a as f64)/b)),
            (Complex(a), Integer(b)) => Ok(Complex(a/(b as f64))),
            (Float(a), Complex(b)) => Ok(Complex(a/b)),
            (Complex(a), Float(b)) => Ok(Complex(a/b)),
            (Complex(a), Complex(b)) => Ok(Complex(a/b)),
            (Integer(a), Ratio(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Ratio(b)))
            } else {
                Ok(Ratio(num::rational::Ratio::<i64>::new(a,1)/b))
            },
            (Ratio(a), Integer(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Integer(b)))
            } else {
                Ok(Ratio(a/b))
            },
            (Float(a), Ratio(b)) => Ok(Float(a/r2f64(&b))),
            (Ratio(a), Float(b)) => Ok(Float(r2f64(&a)/b)),
            (Complex(a), Ratio(b)) => Ok(Complex(a/r2f64(&b))),
            (Ratio(a), Complex(b)) => Ok(Complex(r2f64(&a)/b)),
            (Ratio(a), Ratio(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Ratio(b)))
            } else {
                Ok(Ratio(a/b))
            },
            (a,b) => Err(FunctionError::WrongArgTypes(vec![a, b]))
        }
    }
}

impl Rem<Value> for Value {
    type Output = Result<Value, FunctionError>;
    fn rem(self, rhs: Value) -> Self::Output {
        use Value::*;
        use num::Zero;
        match (self, rhs) {
            (Integer(a), Integer(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Integer(b)))
            } else {
                Ok(Integer(a % b))
            },
            (Float(a), Float(b)) => Ok(Float(a % b)),
            (Integer(a), Float(b)) => Ok(Float((a as f64)%b)),
            (Float(a), Integer(b)) => Ok(Float(a%(b as f64))),
            (Integer(a), Ratio(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Ratio(b)))
            } else {
                Ok(Ratio(num::rational::Ratio::<i64>::new(a,1)%b))
            },
            (Ratio(a), Integer(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Integer(b)))
            } else {
                Ok(Ratio(a%b))
            },
            (Float(a), Ratio(b)) => Ok(Float(a%r2f64(&b))),
            (Ratio(a), Float(b)) => Ok(Float(r2f64(&a)%b)),
            (Ratio(a), Ratio(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Ratio(b)))
            } else {
                 Ok(Ratio(a%b))
            }
            (a,b) => Err(FunctionError::WrongArgTypes(vec![a, b]))
        }
    }
}

impl Neg for Value {
    type Output = Result<Value, FunctionError>;
    fn neg(self) -> Self::Output {
        use Value::*;
        match self {
            Integer(a) => Ok(Integer(-a)),
            Float(a) => Ok(Float(-a)),
            Ratio(a) => Ok(Ratio(-a)),
            Complex(a) => Ok(Complex(-a)),
            a => Err(FunctionError::WrongArgTypes(vec![a]))
        }
    }
}

impl Value {
    pub fn pow(self, rhs: Value) -> Result<Value, FunctionError> {
        use Value::*;
        use std::convert::TryInto;
        match (self, rhs) {
            (Integer(a), Integer(b)) => Ok(Float((a as f64).powf(b as f64))),
            (Float(a), Integer(b)) => Ok(Float(a.powf(b as f64))),
            (Integer(a), Float(b)) => Ok(Float((a as f64).powf(b))),
            (Float(a), Float(b)) => Ok(Float(a.powf(b))),
            (Ratio(a), Integer(b)) => match b.try_into() {
                Ok(bi) => Ok(Ratio(a.pow(bi))),
                Err(_) => Err(FunctionError::WrongArgValue(Integer(b)))
            },
            (Ratio(a), Float(b)) => Ok(Float(r2f64(&a).powf(b))),
            (Integer(a), Ratio(b)) => Ok(Float((a as f64).powf(r2f64(&b)))),
            (Float(a), Ratio(b)) => Ok(Float(a.powf(r2f64(&b)))),
            (Ratio(a), Ratio(b)) => Ok(Float(r2f64(&a).powf(r2f64(&b)))),
            (Complex(a), Integer(b)) => Ok(Complex(a.powf(b as f64))),
            (Complex(a), Float(b)) => Ok(Complex(a.powf(b))),
            (Complex(a), Ratio(b)) => Ok(Complex(a.powf(r2f64(&b)))),
            (Integer(a), Complex(b)) => Ok(Complex(num::Complex::from(a as f64).powc(b))),
            (Float(a), Complex(b)) => Ok(Complex(num::Complex::from(a).powc(b))),
            (Ratio(a), Complex(b)) => Ok(Complex(num::Complex::from(r2f64(&a)).powc(b))),
            (Complex(a), Complex(b)) => Ok(Complex(a.powc(b))),
            (Bool(a), Bool(b)) => Ok(Bool(a ^ b)),
            (a,b) => Err(FunctionError::WrongArgTypes(vec![a, b]))
        }
    }

    pub fn frac(self, rhs: Value) -> Result<Value, FunctionError> {
        use Value::*;
        use num::Zero;
        match (self, rhs) {
            (Integer(a), Integer(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Integer(b)))
            } else {
                Ok(Ratio(num::rational::Ratio::new(a, b)))
            },
            (Integer(a), Ratio(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Ratio(b)))
            } else {
                Ok(Ratio(num::rational::Ratio::<i64>::new(a,1)/b))
            },
            (Ratio(a), Integer(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Integer(b)))
            } else {
                Ok(Ratio(a/b))
            },
            (Ratio(a), Ratio(b)) => if b.is_zero() {
                Err(FunctionError::WrongArgValue(Ratio(b)))
            } else {
                Ok(Ratio(a/b))
            },
            (a,b) => Err(FunctionError::WrongArgTypes(vec![a, b]))
        }
    }
}

// is_T, as_T functions
impl Value {
    pub fn is_int(&self) -> bool {
        match self { Self::Integer(_) => true, _ => false }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self { Self::Integer(n) => Some(*n), _ => None }
    }
    pub fn is_float(&self) -> bool {
        match self { Self::Float(_) => true, _ => false }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self { Self::Float(n) => Some(*n), _ => None }
    }
    pub fn is_complex(&self) -> bool {
        match self { Self::Complex(_) => true, _ => false }
    }
    pub fn as_complex(&self) -> Option<Complex> {
        match self { Self::Complex(n) => Some(*n), _ => None }
    }
    pub fn is_ratio(&self) -> bool {
        match self { Self::Ratio(_) => true, _ => false }
    }
    pub fn as_ratio(&self) -> Option<Ratio> {
        match self { Self::Ratio(n) => Some(*n), _ => None }
    }
    pub fn is_bool(&self) -> bool {
        match self { Self::Bool(_) => true, _ => false }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self { Self::Bool(n) => Some(*n), _ => None }
    }
    pub fn is_list(&self) -> bool {
        match self { Self::List(_) => true, _ => false }
    }
    pub fn as_list(&self) -> Option<&List> {
        match self { Self::List(n) => Some(&n), _ => None }
    }
    pub fn is_void(&self) -> bool {
        match self { Self::Void => true, _ => false }
    }
    pub fn as_void(&self) -> Option<()> {
        match self { Self::Void => Some(()), _ => None }
    }
    pub fn is_callable(&self) -> bool {
        match self { 
            Self::Lambda{..} => true, 
            Self::Function(_) => true, 
            Self::Bool(_) => true, 
            _ => false
        }
    }
}



impl From<i64> for Value {
    fn from(n: i64) -> Self { Self::Integer(n) }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self { Self::Float(n) }
}

impl From<Complex> for Value {
    fn from(n: Complex) -> Self { Self::Complex(n) }
}

impl From<Ratio> for Value {
    fn from(n: Ratio) -> Self { Self::Ratio(n) }
}

impl From<Vec<Value>> for Value {
    fn from(n: Vec<Value>) -> Self { Self::List(n) }
}

impl From<bool> for Value {
    fn from(n: bool) -> Self { Self::Bool(n) }
}

impl From<()> for Value {
    fn from(_: ()) -> Self { Self::Void }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::Integer(n) => n.to_string(),
            Self::Float(n) => n.to_string(),
            Self::Complex(n) => format!("{}+{}i", n.re, n.im),
            Self::Ratio(n) => format!("{}//{}", n.numer(), n.denom()),
            Self::Bool(n) => n.to_string(),
            Self::List(n) => format!("({})", n.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")),
            Self::Function(f) => format!("{:?}", f),
            Self::Lambda{args,..} => format!("<function of {} args>", args.len()),
            Self::Void => "<void>".to_owned()
        }
    }
}
