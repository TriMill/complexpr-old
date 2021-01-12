use crate::function::*;
use crate::Value;

use crate::Context;
lazy_static::lazy_static! {
    /// A `lazy_static` [`Context`] containing all the definitions from [`num`]
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
        ctx.insert_function("signum".to_owned(), &signum);
        ctx.insert_function("fract".to_owned(), &fract);
        ctx.insert_function("floor".to_owned(), &floor);
        ctx.insert_function("ceil".to_owned(), &ceil);
        ctx.insert_function("round".to_owned(), &round);
        ctx.insert_function("gcd".to_owned(), &gcd);
        ctx.insert_function("factors".to_owned(), &factors);
        ctx.insert_function("deg2rad".to_owned(), &deg2rad);
        ctx.insert_function("rad2deg".to_owned(), &rad2deg);
        ctx.insert_function("factorial".to_owned(), &factorial);
        ctx.insert_function("solve".to_owned(), &solve);
        ctx.insert_function("gamma".to_owned(), &gamma);
        ctx.insert_function("lambert_w".to_owned(), &lambert_w);
        ctx.insert("pi".to_owned(), PI.clone());
        ctx.insert("e".to_owned(), E.clone());
        ctx.insert("inf".to_owned(), INF.clone());
        ctx.insert("neg_inf".to_owned(), NEG_INF.clone());
        ctx.insert("nan".to_owned(), NAN.clone());
        ctx
    };
}

/// The [`Value`] form of [`std::f64::consts::PI`]
pub const PI: Value = Value::Float(std::f64::consts::PI);
/// The [`Value`] form of [`std::f64::consts::E`]
pub const E: Value = Value::Float(std::f64::consts::E);
/// The [`Value`] form of [`std::f64::INFINITY`]
pub const INF: Value = Value::Float(std::f64::INFINITY);
/// The [`Value`] form of [`std::f64::NEG_INFINITY`]
pub const NEG_INF: Value = Value::Float(std::f64::NEG_INFINITY);
/// The [`Value`] form of [`std::f64::NAN`]
pub const NAN: Value = Value::Float(std::f64::NAN);

/// Calculates the minimum of a series of arguments. This is done by comparing each argument
/// successively to the minimum, and updating the maximum if the argument is strictly smaller.
/// Requires at least one argument of any type, always returns one of the arguments.
pub fn min(args: Vec<Value>) -> Result {
    min_args(args.len(), 1)?;
    let mut lowest = &args[0];
    for arg in &args[1..] {
        if arg < &lowest || lowest.is_nan() {
            lowest = arg;
        }
    }
    Ok(lowest.clone())
}

/// Calculates the maximum of a series of arguments. This is done by comparing each argument
/// successively to the maximum, and updating the maximum if the argument is strictly greater.
/// Requires at least one argument of any type, always returns one of the arguments.
pub fn max(args: Vec<Value>) -> Result {
    min_args(args.len(), 1)?;
    let mut highest = &args[0];
    for arg in &args[1..] {
        if arg > &highest || highest.is_nan() {
            highest = arg;
        }
    }
    Ok(highest.clone())
}

/// Calculates the absolute value of a real number.
/// Requires exactly one real argument, returns a value of the same type.
pub fn abs(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    use ::num::Signed;
    match &args[0] {
        Value::Float(n) => Ok(Value::Float(n.abs())),
        Value::Integer(n) => Ok(Value::Integer(n.abs())),
        Value::Ratio(n) => Ok(Value::Ratio(n.abs())),
        _ => Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

/// Calculates the square root of a number. If the number is a float, the result is NaN if it would
/// be complex. Otherwise, the result is the principal root.
/// Requires exactly one numerical argument, returns either a [`Value::Float`] or a
/// [`Value::Complex`]
pub fn sqrt(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(n) => Ok(Value::Float(n.sqrt())),
        Value::Complex(n) => Ok(Value::Complex(n.sqrt())),
        _ => unreachable!()
    }
}

/// Calculates the nth root of a value. The first argument is the radicand, the second is the
/// index. The result is NaN if both arguments are floats and the result would be complex.
/// Otherwise, if both are floats, the result is the positive real-valued root. Otherwise, the result is the
/// principal root.
/// Requires exactly two numerical arguments, returns either a [`Value::Float`] or a
/// [`Value::Complex`]
pub fn root(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    let pow = args[1].clone().pow(Value::Float(-1.))?;
    args[0].clone().pow(pow)
}

/// Calculates [`E`] raised to the power of the argument.
/// Requires exactly one numerical argument, returns either a [`Value::Float`] or a
/// [`Value::Complex`]
pub fn exp(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(n) => Ok(Value::Float(n.exp())),
        Value::Complex(n) => Ok(Value::Complex(n.exp())),
        _ => unreachable!()
    }
}

/// Gives the logarithm of a number. The first parameter is the number, the second is the base
/// (defaulting to [`E`] if unspecified).
/// Requires one or two numerical arguments, returns either a [`Value::Float`] or a
/// [`Value::Complex`]
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

/// Gives the natural logarithm of a number. Equivalent to [`log`] with no second argument.
/// Requires exactly one numerical argument, returns either a [`Value::Float`] or a
/// [`Value::Complex`]
pub fn ln(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 2)?;
    match to_float_or_complex(args[0].clone())? {
        Value::Float(n) => Ok(Value::Float(n.ln())),
        Value::Complex(n) => Ok(Value::Complex(n.ln())),
        _ => unreachable!()
    }
}

/// Returns the sign of a real number. For integers and ratios this is either `1`, `-1`, or `0`.
/// For floats, only `1`, `-1`, and `NaN` are allowed (`+0` and `-0` are distinguished).
/// Requires exactly one numerical argument, returns a value of the same type.
pub fn signum(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    use ::num::Signed;
    match args[0] {
        Value::Integer(n) => Ok(Value::Integer(n.signum())),
        Value::Float(n) => Ok(Value::Float(n.signum())),
        Value::Ratio(n) => Ok(Value::Ratio(n.signum())),
        _ => Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

/// Gives the fractional part (part after the decimal point) of a number.
/// Requires exactly one numerical argument, returns a value of the same type.
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

/// Gives the floor (rounding down) of a number.
/// Requires exactly one numerical argument, returns a value of the same type.
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

/// Gives the ceil (rounding up) of a number.
/// Requires exactly one numerical argument, returns a value of the same type.
pub fn ceil(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match args[0] {
        Value::Integer(n) => Ok(Value::Integer(n)),
        Value::Float(n) => Ok(Value::Float(n.ceil())),
        Value::Ratio(n) => Ok(Value::Ratio(n.ceil())),
        Value::Complex(n) => Ok(Value::from_complex(n.re.ceil(), n.im.ceil())),
        _ => Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

/// Rounds a number to the nearest integer, or to a number of decimal places if a second argument
/// is provided.
/// Requires one numerical argument and optionally one integer argument, returns a value of the same type as the first argument.
pub fn round(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 2)?;
    let digits = match args.get(1) {
        Some(Value::Integer(n)) => *n,
        Some(x) => return Err(EvalErrorKind::WrongArgType(x.clone()).into()),
        None => 0
    };
    match args[0] {
        Value::Integer(n) => Ok(Value::Integer(n)),
        Value::Float(n) => {
            let m = 10f64.powf(digits as f64);
            Ok(Value::Float((m*n).round()/m))
        },
        Value::Ratio(n) => {
            let m = crate::value::Ratio::from(10).pow(digits as i32);
            Ok(Value::Ratio((n*m).round()/m))
        }
        Value::Complex(n) => {
            let m = 10f64.powf(digits as f64);
            Ok(Value::from_complex((m*n.re).round()/m, (m*n.im).round()/m))
        },
        _ => Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

/// Calculates the GCD of two integers.
/// Requires exactly two integer arguments, returns an integer.
pub fn gcd(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    match (&args[0], &args[1]) {
        (Value::Integer(u), Value::Integer(v)) => Ok(Value::Integer(gcd_inner(u.abs(), v.abs()))),
        (Value::Integer(_), x) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
        (x, _) => Err(EvalErrorKind::WrongArgType(x.clone()).into())
    }
}

fn gcd_inner(mut u: i64, mut v: i64) -> i64 {
    use std::cmp::min;
    use std::mem::swap;
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }
    let i = u.trailing_zeros();  u >>= i;
    let j = v.trailing_zeros();  v >>= j;
    let k = min(i, j);

    loop {
        if u > v {
            swap(&mut u, &mut v);
        }
        v -= u;
        if v == 0 {
            return u << k;
        }
        v >>= v.trailing_zeros();
    }
}

pub fn factors(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::Integer(n) = &args[0] {
        let mut n = n.abs();
        if n <= 1 {
            return Ok(Value::List(vec![]))
        }
        let fac2 = n.trailing_zeros();
        n >>= fac2;
        let mut res: Vec<i64> = std::iter::repeat(2).take(fac2 as usize).collect();
        let mut f = 3;
        while n > 1 {
            if n % f == 0 {
                res.push(f);
                n /= f;
            } else {
                f += 2;
            }
        }
        Ok(Value::List(res.into_iter().map(|x| Value::Integer(x)).collect()))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

const DEG2RAD: f64 = std::f64::consts::PI/180.;
const RAD2DEG: f64 = 180./std::f64::consts::PI;
/// Convert degrees to radians.
/// Requires exactly one nonnegative real argument, always returns a [`Value::Float`].
pub fn deg2rad(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match to_float(args[0].clone())? {
        Value::Float(n) => Ok(Value::Float(n*DEG2RAD)),
        _ => unreachable!()
    }
}

/// Convert radians to degrees.
/// Requires exactly one nonnegative real argument, always returns a [`Value::Float`].
pub fn rad2deg(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match to_float(args[0].clone())? {
        Value::Float(n) => Ok(Value::Float(n*RAD2DEG)),
        _ => unreachable!()
    }
}

/// Returns the factorial of a nonnegative integer.
/// Requires exactly one nonnegative [`Value::Integer`] argument, always returns a [`Value::Integer`].
pub fn factorial(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match args[0] {
        Value::Integer(n) if n >= 0 => Ok(Value::Integer((2..(n+1)).fold(1, |a,b|a*b))),
        Value::Integer(n) => Err(EvalErrorKind::WrongArgValue(Value::Integer(n)).into()),
        _ => Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}


pub fn gamma(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match to_float(args[0].clone())? {
        Value::Float(x) => Ok(Value::Float(gamma_inner(x))),
        _ => unreachable!()
    }
}

fn gamma_inner(x: f64) -> f64 {
    if x < 0. && x.fract() == 0. {
        std::f64::INFINITY
    } else if x < 0. {
        use std::f64::consts::PI;
        (-PI*x + PI)/(gamma_inner(-x + 2.)*(PI*x).sin())
    } else if x < 1. {
        gamma_approx(x + 1.)/x
    } else if x < 2. {
        gamma_approx(x)
    } else {
        let mut res = 1.;
        let mut x = x;
        while x > 2. {
            x -= 1.;
            res *= x;
        }
        res * gamma_approx(x)

    }
}

const TAYLOR_COEFFICIENTS: [f64; 29] = [
    -0.00000000000000000023,  0.00000000000000000141,  0.00000000000000000119,
    -0.00000000000000011813,  0.00000000000000122678, -0.00000000000000534812,
    -0.00000000000002058326,  0.00000000000051003703, -0.00000000000369680562,
     0.00000000000778226344,  0.00000000010434267117, -0.00000000118127457049,
     0.00000000500200764447,  0.00000000611609510448, -0.00000020563384169776,
     0.00000113302723198170, -0.00000125049348214267, -0.00002013485478078824,
     0.00012805028238811619, -0.00021524167411495097, -0.00116516759185906511,
     0.00721894324666309954, -0.00962197152787697356, -0.04219773455554433675,
     0.16653861138229148950, -0.04200263503409523553, -0.65587807152025388108,
     0.57721566490153286061,  1.00000000000000000000,
];
const INITIAL_SUM: f64 = 0.00000000000000000002;
fn gamma_approx(x: f64) -> f64 {
    TAYLOR_COEFFICIENTS.iter().fold(INITIAL_SUM, |sum, coefficient| {
        sum * (x - 1.0) + coefficient
    }).recip()
}

const LAMBERT_W_MIN: f64 = -1f64/std::f64::consts::E;

fn lambert_w(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match to_float(args[0].clone())? {
        Value::Float(x) if x >= LAMBERT_W_MIN => Ok(Value::Float(lambert_inner(x))),
        Value::Float(x) => Err(EvalErrorKind::WrongArgValue(Value::Float(x)).into()),
        _ => unreachable!()
    }
}

fn lambert_inner(a: f64) -> f64 {
    let mut x: f64 = 0.75*(a+1.).ln();
    for _ in 0..50 {
        let ex = x.exp();
        let next = x - (x*ex - a)/(ex*(1.+x));
        if next == x {
            break
        }
        x = next;
    }
    x
}

const EPSILON: Value = Value::Float(0.00000000023283064365386963);
const INV_EPSILON: Value = Value::Float(4294967296.);
/// solve(fn, guess)
pub fn solve(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    let func = &args[0];
    let mut res = args[1].clone();
    for _ in 0..100 {
        let fnres = func.eval(vec![res.clone()])?;
        if fnres == Value::Float(0.) {
            break
        }
        let deriv = ((func.eval(vec![(res.clone() + EPSILON)?])? - fnres.clone())? * INV_EPSILON)?;
        let next = (res.clone() - (fnres/deriv)?)?;
        if next == res {
            break
        }
        res = next;
    }
    Ok(res)
}

