use crate::function::*;
use crate::Value;
use crate::value::Ratio;

use crate::Context;
lazy_static::lazy_static! {
    /// A `lazy_static` [`Context`] containing all the definitions from [`types`]
    pub static ref CTX_ALL: Context = {
        use crate::InsertFunction;
        let mut ctx = Context::new();
        ctx.insert_function("typeof".to_owned(), &fn_typeof);
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
        ctx.insert_function("to_int".to_owned(), &to_int);
        ctx.insert_function("to_ratio".to_owned(), &to_ratio);
        ctx.insert_function("to_str".to_owned(), &to_str);
        ctx.insert_function("to_repr".to_owned(), &to_repr);
        ctx
    };
}

/// Returns the type of a value as a [`Value::Str`], using [`Value::get_type`].
/// Requires exactly one argument of any type, always returns a [`Value::Str`].
pub fn fn_typeof(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Str(args[0].get_type()))
}
/// Checks if a value is a [`Value::Integer`].
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_float(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_float()))
}
/// Checks if a value is a [`Value::Integer`].
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_int(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_int()))
}
/// Checks if a value is a [`Value::Complex`].
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_complex(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_complex()))
}
/// Checks if a value is a [`Value::Ratio`].
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_ratio(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_ratio()))
}
/// Checks if a value is a [`Value::Bool`].
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_bool(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_bool()))
}
/// Checks if a value is a [`Value::List`].
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_list(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_list()))
}
/// Checks if a value is a [`Value::Str`].
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_str(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_str()))
}
/// Checks if a value is callable, ie. it is a [`Value::Function`], [`Value::Lambda`], or [`Value::Bool`]. 
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_callable(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_callable()))
}
/// Checks if a value is [`Value::Void`].
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_void(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_void()))
}

/// Checks if a value is infinite.
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_infinite(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_infinite()))
}
/// Checks if a value is `NaN`.
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_nan(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Bool(args[0].is_nan()))
}
/// Checks if a value is normal, ie. it is not `NaN` and not infinite.
/// Requires exactly one argument of any type, always returns a [`Value::Bool`].
pub fn is_normal(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    return Ok(Value::Bool(match args[0] {
        Value::Float(f) => !f.is_nan() && !f.is_infinite(),
        Value::Complex(c) => !c.is_nan() && !c.is_infinite(),
        Value::Ratio(_) | Value::Integer(_) => true,
        _ => false
    }))
}

/// Convert a value to an integer. Requires exactly one argument, either an `Integer`, a non-`NaN`
/// non-infinite `Float`, or a `Ratio`.
pub fn to_int(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match &args[0] {
        Value::Integer(n) => Ok(Value::Integer(*n)),
        Value::Float(f) if !f.is_nan() && !f.is_infinite()
            => Ok(Value::Integer(*f as i64)),
        Value::Float(_) => Err(EvalErrorKind::WrongArgValue(args[0].clone()).into()),
        Value::Ratio(n) => Ok(Value::Integer(*n.floor().numer())),
        _ => Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

/// Convert a value to a ratio. Requires exactly one argument, either a [`Value::Ratio`], [`Value::Float`], or
/// [`Value::Integer`], always returns a [`Value::Ratio`]. Throws an error if the input is a `NaN`
/// or infinite [`Value::Float`].
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

/// Convert a value to a string (using the [`ToString`] implementation on [`Value`]). 
/// Requires exactly one argument of any type, always returns a [`Value::Str`].
pub fn to_str(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Str(args[0].to_string()))
}

/// Convert a value to it's representation (using the [`Debug`] implementation on [`Value`]). 
/// Requires exactly one argument of any type, always returns a [`Value::Str`].
pub fn to_repr(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    Ok(Value::Str(format!("{:?}", args[0])))
}
