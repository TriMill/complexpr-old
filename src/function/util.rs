use crate::function::*;
use crate::Value;

use crate::Context;
lazy_static::lazy_static! {
    pub static ref CTX_ALL: Context = {
        use crate::InsertFunction;
        let mut ctx = Context::new();
        ctx.insert_function("eval".to_owned(), &eval);
        ctx.insert_function("map".to_owned(), &map);
        ctx.insert_function("fold".to_owned(), &fold);
        ctx.insert_function("rev".to_owned(), &rev);
        ctx.insert_function("filter".to_owned(), &filter);
        ctx.insert_function("sort".to_owned(), &sort);
        ctx.insert_function("index".to_owned(), &index);
        ctx.insert_function("slice".to_owned(), &slice);
        ctx.insert_function("apply".to_owned(), &apply);
        ctx.insert_function("len".to_owned(), &len);
        ctx.insert_function("chars".to_owned(), &chars);
        ctx.insert_function("range".to_owned(), &range);
        ctx.insert_function("first".to_owned(), &first);
        ctx.insert_function("repeat".to_owned(), &repeat);
        ctx.insert_function("enumerate".to_owned(), &enumerate);
        ctx.insert_function("zip".to_owned(), &zip);
        ctx.insert_function("iter".to_owned(), &iter);
        ctx.insert_function("enumiter".to_owned(), &enumiter);
        ctx.insert_function("seq".to_owned(), &seq);
        ctx.insert_function("or_else".to_owned(), &or_else);
        ctx.insert_function("and_then".to_owned(), &and_then);
        ctx.insert_function("loop".to_owned(), &fn_loop);
        ctx.insert_function("from_radix".to_owned(), &from_radix);
        ctx.insert_function("to_radix".to_owned(), &from_radix);
        ctx.insert_function("exact_eq".to_owned(), &exact_eq);
        ctx.insert_function("ord".to_owned(), &ord);
        ctx.insert_function("chr".to_owned(), &chr);
        ctx
    };
}

pub fn eval(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 2)?;
    if let Value::Str(src) = &args[0] {
        let mut ctx = match args.len() {
            1 => Context::new(),
            2 => match crate::tree::value_to_ctx(&args[1]) {
                Some(ctx) => ctx,
                None => return Err(EvalErrorKind::WrongArgValue(args[1].clone()).into())
            },
            _ => unreachable!()
        };
        match crate::eval(src, &mut ctx) {
            Ok(v) => Ok(v),
            Err(e) => Err(EvalErrorKind::Other(format!("Inside eval: {}", e)).into())
        }
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn map(args: Vec<Value>) -> Result {
    min_args(args.len(), 1)?;
    let first = &args[0];
    let fns = &args[1..];
    if let Value::List(v) = first {
        let mut list = v.to_vec();
        for f in fns {
            list = {
                let mut newlist = vec![];
                for v in list {
                    newlist.push(f.eval(vec![v])?);
                }
                newlist
            };
        }
        Ok(Value::List(list))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn fold(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 3)?;
    if args.len() == 3 {
        let (mut res, list, func) = (args[0].clone(), &args[1], &args[2]);
        if let Value::List(l) = list {
            for v in l {
                res = func.eval(vec![res, v.clone()])?;
            }
            Ok(res)
        } else {
            Err(EvalErrorKind::WrongArgType(args[1].clone()).into())
        }
    } else {
        let (list, func) = (args[0].clone(), &args[1]);
        if let Value::List(l) = list {
            if l.len() == 0 {
                return Ok(Value::Void)
            }
            let mut res = l[0].clone();
            for v in &l[1..] {
                res = func.eval(vec![res, v.clone()])?;
            }
            Ok(res)
        } else {
            Err(EvalErrorKind::WrongArgType(args[1].clone()).into())
        }
    }
}

pub fn rev(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match &args[0] {
        Value::List(l) => Ok(Value::List(l.iter().rev().cloned().collect())),
        Value::Str(s) => Ok(Value::Str(s.chars().rev().collect())),
        _ => Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn filter(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    let func = &args[1];
    match &args[0] {
        Value::List(l) => {
            let mut res = vec![];
            for v in l {
                if func.eval(vec![v.clone()])? == Value::Bool(true) {
                    res.push(v);
                }
            }
            let res = res.iter().map(|&x| x.clone()).collect();
            Ok(Value::List(res))
        },
        Value::Str(s) => {
            let mut res = String::new();
            for c in s.chars() {
                if func.eval(vec![Value::Str(c.to_string())])? == Value::Bool(true) {
                    res.push(c);
                }
            }
            Ok(Value::Str(res))
        },
        _ => return Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn sort(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match &args[0] {
        Value::List(l) => {
            let l: Vec<&Value> = sort_inner(l.iter().by_ref().collect());
            Ok(Value::List(l.iter().map(|&x| x.clone()).collect()))
        },
        Value::Str(s) => {
            let mut s2: Vec<char> = s.chars().collect();
            s2.sort();
            Ok(Value::Str(s2.into_iter().collect()))
        },
        _ => return Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn sort_inner(list: Vec<&Value>) -> Vec<&Value> {
    if list.len() <= 1 {
        return list
    }
    let pivot: &Value = list[0];
    let mut before: Vec<&Value> = vec![];
    let mut after: Vec<&Value> = vec![];
    for v in list.into_iter().skip(1) {
        if v >= pivot {
            after.push(v);
        } else {
            before.push(v);
        }
    }
    let mut res = sort_inner(before);
    let after = sort_inner(after);
    res.push(pivot);
    for v in after {
        res.push(v);
    }
    res
}

pub fn index(args: Vec<Value>) -> Result {
    use std::convert::TryInto;
    bound_args(args.len(), 2, 2)?;
    let list = &args[0];
    let idx = &args[1];
    match (list, idx) {
        (Value::List(l), Value::Integer(i)) => {
            if *i >= 0 && *i < l.len().try_into().unwrap_or(-1) {
                Ok(l[*i as usize].clone())
            } else {
                Err(EvalErrorKind::ListOutOfBounds(*i).into())
            }
        },
        (Value::Str(s), Value::Integer(i)) => {
            if *i >= 0 && *i < s.len().try_into().unwrap_or(-1) {
                Ok(Value::Str(s.chars().nth(*i as usize).unwrap().to_string()))
            } else {
                Err(EvalErrorKind::ListOutOfBounds(*i).into())
            }
        },
        (Value::List(_), x) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
        (Value::Str(_), x) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
        (x,_) => Err(EvalErrorKind::WrongArgType(x.clone()).into())
    }
}

pub fn slice(args: Vec<Value>) -> Result {
    bound_args(args.len(), 3, 3)?;
    let list = &args[0];
    let idx1 = &args[1];
    let idx2 = &args[2];
    match (list, idx1, idx2) {
        (Value::List(l), Value::Integer(a), Value::Integer(b)) => {
            let len = l.len() as i64;
            let mut a = *a;
            let mut b = *b;
            if a < 0 { a = len + a; }
            if b < 0 { b = len + b; }
            if a < 0 || a >= len {
                return Err(EvalErrorKind::WrongArgValue(idx1.clone()).into())
            }
            if b < 0 || b >= len {
                return Err(EvalErrorKind::WrongArgValue(idx2.clone()).into())
            }
            let a = a as usize;
            let b = b as usize;
            if a == b {
                Ok(Value::List(vec![]))
            } else if a < b {
                Ok(Value::List(l[a..b].to_vec()))
            } else {
                Ok(Value::List(l[b..a].iter().rev().cloned().collect::<Vec<Value>>()))
            }
        },
        (Value::Str(s), Value::Integer(a), Value::Integer(b)) => {
            let len = s.len() as i64;
            let mut a = *a;
            let mut b = *b;
            if a < 0 { a = len + a; }
            if b < 0 { b = len + b; }
            if a < 0 || a > len {
                return Err(EvalErrorKind::WrongArgValue(idx1.clone()).into())
            }
            if b < 0 || b > len {
                return Err(EvalErrorKind::WrongArgValue(idx2.clone()).into())
            }
            let a = a as usize;
            let b = b as usize;
            if a == b {
                Ok(Value::Str(String::new()))
            } else if a < b {
                Ok(Value::Str(s[a..b].to_owned()))
            } else {
                Ok(Value::Str(s[b..a].chars().rev().collect::<String>()))
            }
        },
        (_, Value::Integer(_), x) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
        (Value::List(_), x, _) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
        (Value::Str(_), x, _) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
        (x,_,_) => Err(EvalErrorKind::WrongArgType(x.clone()).into())
    }
}

pub fn apply(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    let func = &args[0];
    let list = &args[1];
    match list {
        Value::List(l) => func.eval(l.to_vec()),
        _ => Err(EvalErrorKind::WrongArgType(args[1].clone()).into())
    }
}

pub fn len(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::List(l) = &args[0] {
        Ok(Value::Integer(l.len() as i64))
    } else if let Value::Str(s) = &args[0] {
        Ok(Value::Integer(s.len() as i64))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn chars(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    match &args[0] {
        Value::Str(s) => Ok(Value::List(s.chars().map(|x| Value::Str(x.to_string())).collect())),
        _ => Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

/// Return a list of values within a range
/// range(max) => (0, 1, 2, ..., max)
/// range(min, max) => (min, min+1, min+2, ..., max)
/// range(min, max, step) => (min, min+step, min+2*step, ...) until >= max
pub fn range(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 3)?;
    if args.len() == 1 {
        // 0..max
        if let Value::Integer(n) = args[0] {
            Ok(Value::List((0..n).map(|x| Value::Integer(x)).collect()))
        } else {
            Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
        }
    } else if args.len() == 2 {
        // min..max
        match (&args[0], &args[1]) {
            (Value::Integer(min), Value::Integer(max)) =>
                Ok(Value::List((*min..*max).map(|x| Value::Integer(x)).collect())),
            (Value::Integer(_), x) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
            (x, _) => Err(EvalErrorKind::WrongArgType(x.clone()).into())
        }
    } else {
        match (&args[0], &args[1], &args[2]) {
            (Value::Integer(min), Value::Integer(max), Value::Integer(step)) 
                if *step > 0 => 
                Ok(Value::List(
                        (*min..*max).step_by(*step as usize)
                        .map(|x| Value::Integer(x))
                        .collect())),
            (Value::Integer(_), Value::Integer(_), x) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
            (Value::Integer(_), x, _) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
            (x, _, _) => Err(EvalErrorKind::WrongArgType(x.clone()).into())
        }
    }
}

pub fn first(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    match (&args[0], &args[1]) {
        (Value::List(l), Value::Integer(n)) if *n >= 0 => {
            if l.len() <= *n as usize {
                Ok(Value::List(l.clone()))
            } else {
                Ok(Value::List(l[..(*n as usize)].to_vec()))
            }
        },
        (Value::Str(s), Value::Integer(n)) if *n >= 0 => {
            if s.len() <= *n as usize {
                Ok(Value::Str(s.to_owned()))
            } else {
                Ok(Value::Str(s[..(*n as usize)].to_owned()))
            }
        },
        (Value::List(_), x) | (Value::Str(_), x) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
        (x, _) => Err(EvalErrorKind::WrongArgType(x.clone()).into())
    }
}

pub fn repeat(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    match (&args[0], &args[1]) {
        (Value::List(l), Value::Integer(n)) if *n >= 0 => {
            Ok(Value::List(l.iter().cloned().cycle().take(l.len()*(*n as usize)).collect()))
        },
        (Value::Str(s), Value::Integer(n)) if *n >= 0 => {
            Ok(Value::Str(s.chars().cycle().take(s.len()*(*n as usize)).collect()))
        },
        (Value::List(_), x) | (Value::Str(_), x) => Err(EvalErrorKind::WrongArgType(x.clone()).into()),
        (x, _) => Err(EvalErrorKind::WrongArgType(x.clone()).into())
    }
}

pub fn zip(args: Vec<Value>) -> Result {
    min_args(args.len(), 1)?;
    let mut min_len = usize::MAX;
    let mut lists = vec![];
    for arg in args {
        if let Value::List(l) = arg {
            if l.len() < min_len {
                min_len = l.len();
            }
            lists.push(l);
        } else {
            return Err(EvalErrorKind::WrongArgType(arg.clone()).into())
        }
    }
    let mut res = vec![];
    for i in 0..min_len {
        res.push(Value::List(lists.iter().map(|l| l[i].clone()).collect()));
    }
    Ok(Value::List(res))
}

pub fn enumerate(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::List(l) = &args[0] {
        Ok(Value::List(
                l.iter().cloned()
                .enumerate()
                .map(|(i, v)| Value::List(vec![Value::Integer(i as i64), v]))
                .collect()
                ))
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn iter(args: Vec<Value>) -> Result {
    bound_args(args.len(), 3, 3)?;
    let func = &args[0];
    let init = &args[1];
    if let Value::Integer(n) = args[2] {
        let mut val = init.clone();
        for _ in 0..n {
            val = func.eval(vec![val])?;
        }
        Ok(val)
    } else {
        Err(EvalErrorKind::WrongArgType(args[2].clone()).into())
    }
}

pub fn enumiter(args: Vec<Value>) -> Result {
    bound_args(args.len(), 3, 3)?;
    let func = &args[0];
    let init = &args[1];
    if let Value::Integer(n) = args[2] {
        let mut val = init.clone();
        for i in 0..n {
            val = func.eval(vec![Value::Integer(i), val])?;
        }
        Ok(val)
    } else {
        Err(EvalErrorKind::WrongArgType(args[2].clone()).into())
    }
}

pub fn seq(args: Vec<Value>) -> Result {
    bound_args(args.len(), 3, 3)?;
    let func = &args[0];
    let init = &args[1];
    if let Value::Integer(n) = args[2] {
        let mut val = init.clone();
        let mut res = vec![val.clone()];
        for _ in 0..(n-1) {
            val = func.eval(vec![val])?;
            res.push(val.clone())
        }
        Ok(Value::List(res))
    } else {
        Err(EvalErrorKind::WrongArgType(args[2].clone()).into())
    }
}

pub fn or_else(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    match &args[0] {
        Value::Void => Ok(args[1].clone()),
        x => Ok(x.clone())
    }
}

pub fn and_then(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    match &args[0] {
        Value::Void => Ok(Value::Void),
        x => args[1].eval(vec![x.clone()])
    }
}

pub fn fn_loop(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    let mut res = Value::Void;
    loop {
        let a = args[0].eval(vec![])?;
        if a.is_void() {
            break
        } else {
            res = a
        }
    }
    Ok(res)
}

pub fn from_radix(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    if let Value::Integer(r) = args[1] {
        if r < 2 || r > 36 {
            return Err(EvalErrorKind::WrongArgValue(args[1].clone()).into())
        }
        if let Value::Str(s) = &args[0] {
            match i64::from_str_radix(s, r as u32) {
                Ok(n) => Ok(Value::Integer(n)),
                Err(_) => Err(EvalErrorKind::WrongArgValue(args[0].clone()).into())
            }
        } else {
            Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
        }
    } else {
        Err(EvalErrorKind::WrongArgType(args[1].clone()).into())
    }
}

pub fn to_radix(args: Vec<Value>) -> Result {
    bound_args(args.len(), 2, 2)?;
    if let Value::Integer(r) = args[1] {
        if r < 2 || r > 36 {
            return Err(EvalErrorKind::WrongArgValue(args[1].clone()).into())
        }
        if let Value::Integer(s) = &args[0] {
            Ok(Value::Str(format_radix(*s, r as u32)))
        } else {
            Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
        }
    } else {
        Err(EvalErrorKind::WrongArgType(args[1].clone()).into())
    }
}

fn format_radix(mut x: i64, radix: u32) -> String {
    assert!(radix >= 2 && radix <= 36);
    let i64radix = radix as i64;
    let mut result = vec![];
    let is_neg = x < 0;
    x = x.abs();

    loop {
        let m = x % i64radix;
        x = x / i64radix;
        result.push(std::char::from_digit(m as u32, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    if is_neg {
        result.push('-')
    }
    result.into_iter().rev().collect()
}

pub fn exact_eq(args: Vec<Value>) -> Result {
    use std::mem::discriminant;
    bound_args(args.len(), 2, 2)?;
    Ok(Value::Bool(args[0] == args[1] && discriminant(&args[0]) == discriminant(&args[1])))
}

pub fn ord(args: Vec<Value>) -> Result {
    bound_args(args.len(), 1, 1)?;
    if let Value::Str(s) = &args[0] {
        if s.chars().count() != 1 { 
            Err(EvalErrorKind::WrongArgValue(args[0].clone()).into())
        } else {
            Ok(Value::Integer(s.chars().next().unwrap() as i64))
        }
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}

pub fn chr(args: Vec<Value>) -> Result {
    use std::convert::TryInto;
    bound_args(args.len(), 1, 1)?;
    if let Value::Integer(n) = args[0] {
        if let Ok(n) = n.try_into() {
            if let Some(c) = std::char::from_u32(n) {
                Ok(Value::Str(String::from(c)))
            } else {
                Err(EvalErrorKind::WrongArgValue(args[0].clone()).into())
            }
        } else {
            Err(EvalErrorKind::WrongArgValue(args[0].clone()).into())
        }
    } else {
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}
