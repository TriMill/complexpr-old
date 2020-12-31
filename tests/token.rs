#[test]
fn test_tokenize() {
    use complexpr::token::{tokenize, Token};
    use complexpr::ops::{BinaryOp};
    let a = tokenize("0.5 + .23 + 12. + 5");
    assert_eq!(a.unwrap(), vec![
            Token::Float(0.5), Token::BinaryOp(BinaryOp::Add), Token::Float(0.23), Token::BinaryOp(BinaryOp::Add),
            Token::Float(12.), Token::BinaryOp(BinaryOp::Add), Token::Integer(5)
    ]);
}

#[test]
fn test_eval() {
    use complexpr::*;
    let mut ctx = complexpr::ctx_default();
    assert_eq!(eval("1+1", &mut ctx).unwrap(), Value::Integer(2));
    assert_eq!(eval("2*(3+4)", &mut ctx).unwrap(), Value::Integer(14));
    assert_eq!(eval("1//2+3//4", &mut ctx).unwrap(), Value::from_ratio(5, 4));
    assert_eq!(eval("x=1", &mut ctx).unwrap(), Value::Void);
    assert_eq!(eval("1==2", &mut ctx).unwrap(), Value::Bool(false));
    assert_eq!(eval("1==(1.5-0.5)", &mut ctx).unwrap(), Value::Bool(true));
    assert_eq!(eval("factorial(5)", &mut ctx).unwrap(), Value::Integer(120));
    eval("x = 1+2", &mut ctx).unwrap();
    assert_eq!(eval("sin(x)", &mut ctx).unwrap(), Value::Float(3f64.sin()));
    eval("y = add(x, x, 5)", &mut ctx).unwrap();
    assert_eq!(eval("y", &mut ctx).unwrap(), Value::Integer(11));
    assert_eq!(eval("(y < 2)(10, 20)", &mut ctx).unwrap(), Value::Integer(20));
    assert_eq!(eval("sqrt(2)", &mut ctx).unwrap(), Value::Float(2f64.sqrt()));
    assert_eq!(eval("var = 2; var += 5; var -= 3; var - 1", &mut ctx).unwrap(), Value::Integer(3));
    assert_eq!(eval("var = (var += 1; var*2); var", &mut ctx).unwrap(), Value::Integer(10));
    assert_eq!(eval("(var > 5)(add, sub)(1, 2)", &mut ctx).unwrap(), Value::Integer(3));
    assert_eq!(eval("(var < 5)(add, sub)(1, 2)", &mut ctx).unwrap(), Value::Integer(-1));
    eval("testfunc = (a, b):(max(a, b)-a)", &mut ctx).unwrap();
    assert_eq!(eval("testfunc(5, 3)", &mut ctx).unwrap(), Value::Integer(0));
    assert_eq!(eval("testfunc(3, 5)", &mut ctx).unwrap(), Value::Integer(2));
    eval("partial = (f, a):(b:f(a, b)); add_one = partial(add, 1)", &mut ctx).unwrap();
    assert_eq!(eval("add_one(5)", &mut ctx).unwrap(), Value::Integer(6));
}
