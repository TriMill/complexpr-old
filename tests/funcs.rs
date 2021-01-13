use complexpr::*;

#[test]
fn ops() {
    assert_eq!(eval_default("add(2, 3.5, 4.2)").unwrap(), Value::from(2.+3.5+4.2));
    assert_eq!(eval_default("sub(2, 3.5, 4.2)").unwrap(), Value::from(2.-3.5-4.2));
    assert_eq!(eval_default("mul(2, 3.5, 4.2)").unwrap(), Value::from(2.*3.5*4.2));
    assert_eq!(eval_default("div(2, 3.5, 4.2)").unwrap(), Value::from(2./3.5/4.2));
    assert_eq!(eval_default("pow(2, 3.5, 4.2)").unwrap(), Value::from(2f64.powf(3.5f64.powf(4.2))));
    assert_eq!(eval_default("mod(2, 3.5, 4.2)").unwrap(), Value::from(2.%3.5%4.2));
    assert_eq!(eval_default("frac(2, 3, 4)").unwrap(), Value::from_ratio(2, 3*4));
}

#[test]
fn trig() {
    assert_eq!(eval_default("sin(0.6)").unwrap(), Value::from(0.6f64.sin()));
    assert_eq!(eval_default("cos(0.6)").unwrap(), Value::from(0.6f64.cos()));
    assert_eq!(eval_default("tan(0.6)").unwrap(), Value::from(0.6f64.tan()));
    assert_eq!(eval_default("sinh(0.6)").unwrap(), Value::from(0.6f64.sinh()));
    assert_eq!(eval_default("cosh(0.6)").unwrap(), Value::from(0.6f64.cosh()));
    assert_eq!(eval_default("tanh(0.6)").unwrap(), Value::from(0.6f64.tanh()));
    assert_eq!(eval_default("asin(0.6)").unwrap(), Value::from(0.6f64.asin()));
    assert_eq!(eval_default("acos(0.6)").unwrap(), Value::from(0.6f64.acos()));
    assert_eq!(eval_default("atan(0.6)").unwrap(), Value::from(0.6f64.atan()));
    assert_eq!(eval_default("asinh(0.6)").unwrap(), Value::from(0.6f64.asinh()));
    assert_eq!(eval_default("acosh(1.6)").unwrap(), Value::from(1.6f64.acosh()));
    assert_eq!(eval_default("atanh(0.6)").unwrap(), Value::from(0.6f64.atanh()));
    assert_eq!(eval_default("atan2(0.6, 1.4)").unwrap(), Value::from(0.6f64.atan2(1.4)));
}

#[test]
fn num() {
    assert_eq!(eval_default("min(0.6, 1.4, -0.5)").unwrap(), Value::from(-0.5));
    assert_eq!(eval_default("max(0.6, 1.4, -0.5)").unwrap(), Value::from(1.4));
    assert_eq!(eval_default("abs(-0.6)").unwrap(), Value::from(0.6));
    assert_eq!(eval_default("sqrt(0.6)").unwrap(), Value::from(0.6f64.sqrt()));
    assert!(eval_default("sqrt(-1)").unwrap().as_float().unwrap().is_nan());
    assert_eq!(eval_default("sqrt(-1+0i)").unwrap(), Value::from_complex(0., 1.));
    assert_eq!(eval_default("root(0.6, 4)").unwrap(), Value::from(0.6f64.powf(0.25)));
    assert_eq!(eval_default("exp(0.6)").unwrap(), Value::from(0.6f64.exp()));
    assert_eq!(eval_default("log(0.6)").unwrap(), Value::from(0.6f64.ln()));
    assert_eq!(eval_default("log(0.6, 2)").unwrap(), Value::from(0.6f64.log2()));
    assert_eq!(eval_default("log(0.6, 5)").unwrap(), Value::from(0.6f64.log(5.)));
    assert_eq!(eval_default("fract(3.4)").unwrap(), Value::from(3.4f64.fract()));
    assert_eq!(eval_default("floor(3.4)").unwrap(), Value::from(3.4f64.floor()));
    assert_eq!(eval_default("fract(7//3)").unwrap(), Value::from_ratio(1, 3));
    assert_eq!(eval_default("floor(7//3)").unwrap(), Value::from_ratio(2, 1));
    assert_eq!(eval_default("deg2rad(90)").unwrap(), Value::from(std::f64::consts::PI*0.5));
    assert_eq!(eval_default("rad2deg(pi*0.5)").unwrap(), Value::from(90.));
    assert_eq!(eval_default("factorial(5)").unwrap(), Value::from(120));
    assert_eq!(eval_default("factorial(0)").unwrap(), Value::from(1));
}

#[test]
fn complex() {
    assert_eq!(eval_default("re(3-4i)").unwrap(), Value::from(3.));
    assert_eq!(eval_default("im(3-4i)").unwrap(), Value::from(-4.));
    assert_eq!(eval_default("conj(3-4i)").unwrap(), Value::from_complex(3., 4.));
    assert_eq!(eval_default("arg(1+i)").unwrap(), Value::from(std::f64::consts::FRAC_PI_4));
    assert_eq!(eval_default("norm(1+i)").unwrap(), Value::from(2f64.sqrt()));
    assert_eq!(eval_default("norm_sq(1+i)").unwrap(), Value::from(2f64));
    assert_eq!(eval_default("normalize(1+i)").unwrap(), Value::from_complex(1.0/2f64.sqrt(), 1.0/2f64.sqrt()));
}
