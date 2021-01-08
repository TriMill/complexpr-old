use crate::function::*;
use crate::Value;
use std::sync::Arc;

use crate::Context;
lazy_static::lazy_static! {
    pub static ref CTX_ALL: Context = {
        use crate::InsertFunction;
        let mut ctx = Context::new();
        ctx.insert_function("print".to_owned(), &f_print);
        ctx.insert_function("println".to_owned(), &f_println);
        ctx.insert_function("readln".to_owned(), &readln);
        ctx.insert_function("exit".to_owned(), &exit);
        ctx.insert_function("read_file".to_owned(), &read_file);
        ctx
    };
}

pub fn f_println(args: Vec<Value>) -> Result {
    for a in args {
        print!("{}", a);
    }
    println!();
    Ok(Value::Void)
}

pub fn f_print(args: Vec<Value>) -> Result {
    use std::io::Write;
    for a in args {
        print!("{}", a);
    }
    match std::io::stdout().flush() {
        Ok(_) => Ok(Value::Void),
        Err(e) => Err(EvalErrorKind::IOError(Arc::new(e)).into())
    }
}

pub fn readln(args: Vec<Value>) -> Result {
    bound_args(args.len(), 0, 0)?;
    match readln_inner() {
        Ok(Some(s)) => Ok(Value::Str(s)),
        Ok(None) => Ok(Value::Void),
        Err(e) => Err(EvalErrorKind::IOError(Arc::new(e)).into())
    }
}

fn readln_inner() -> std::io::Result<Option<String>> {
    use std::io::BufRead;
    let mut buf = String::new();
    match std::io::stdin().lock().read_line(&mut buf)? {
        0 => Ok(None),
        _ => Ok(Some(buf))
    }
}

pub fn exit(args: Vec<Value>) -> Result {
    bound_args(args.len(), 0, 1)?;
    if args.len() == 1 {
        if let Value::Integer(n) = args[0] {
            if n >= (i32::MIN as i64) && n <= (i32::MAX as i64) {
                std::process::exit(n as i32);
            } else {
                return Err(EvalErrorKind::WrongArgValue(Value::Integer(n)).into())
            }
        } else {
            return Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
        }
    } else {
        std::process::exit(0);
    }
}

pub fn read_file(args: Vec<Value>) -> Result {
    use std::io::Read;
    bound_args(args.len(), 1, 1)?;
    if let Value::Str(s) = args[0].clone() {
        match std::fs::File::open(s) {
            Ok(mut f) => {
                let mut buf = String::new();
                match f.read_to_string(&mut buf) {
                    Ok(_) => Ok(Value::Str(buf)),
                    Err(e) => Err(EvalErrorKind::IOError(Arc::new(e)).into())
                }
            },
            Err(e) => Err(EvalErrorKind::IOError(Arc::new(e)).into())
        } 
    } else { 
        Err(EvalErrorKind::WrongArgType(args[0].clone()).into())
    }
}
