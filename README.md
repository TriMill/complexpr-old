# complexpr

`complexpr` is an expression evaluator written in Rust. It inlcudes native support for ratios and complex numbers, list oeprations, a large library of functions, and syntax for defining new functions inside expressions.

## Types
All values are stored as variants of the `complexpr::value::Value` enum.
- `Integer(i64)` - a 64-bit signed integer
- `Float(f64)` - a 64-bit floating point number
- `Complex(num::complex::Complex<f64>)` - a complex number composed of two floats
- `Ratio(num::rational::Ratio<i64>)` - a ratio of two signed integers
- `Bool(bool)` - a boolean value
- `List(Vec<Value>)` - a list of values
- `Str(String)` - a string
- `Function(complexpr::function::Function)` - a Rust function callable from inside expressions
- `Lambda{..}` - a function created inside an expression

The `Value` enum implements the `+`, `-`, `*`, `/`, and `%` operators in Rust, as well as equality, comparison, and the functions `pow` and `frac`. It also implements other utility functions.

## Syntax
### Literals
The following literals are supported:
- integers - `\d+`
- floats - `\d+\.\d*|\.\d+` (at least one digit and exactly one `.`)
- imaginary numbers - `\d+(\.\d*)?i|\.\d+i` (integer or float followed by `i`)
- true and false - `true|false`
- strings - `"(?:[^"\\]|\\[\\"nrt0]|\\u\{[0-9a-fA-F]+\}|\\x[0-9a-fA-F]{2})*"` (see below)

#### String literals
String literals must begin and end with a double quote (`"`). The body of a string literal is a sequence of the following:
- any character besides `"` or `\`
- The escape sequences `\"` (double quote), `\\` (backslash), `\n` (newline), `\r` (carriage return), `\t` (tab), `\0` (null), `\e` (escape, hex code `0x1b`)
- The escape sequence `\x` followed by two hex digits (unicode character by hex value)
- The escape sequence `\u`, followed by a series of hex digits surrounded by curly braces (unicode character by hex value), example: `\u{1F41F}` (🐟)

### Operators
Binary operators:
- `+` - addition, string concatenation, list concatenation, boolean `or`
- `-` - subtraction
- `*` - multiplication, boolean `and`
- `/` - division
- `^` - exponentiation, boolean `xor`
- `//` - fraction
- `%` - modulo (remainder after division)
- `>`, `<`, `>=`, `<=` - greater than, less than, greater or equal, less or equal
- `==`, `!=` - equal, not equal

Assignment operators:
- `=` - assignment
- `+=`, `-=`, `*=`, `/=`, `%=` - compound assignments

Unary operators:
- `-` - negation, boolean `not`

Other:
- `,` - separate expressions in a list
- `;` - separate expressions in a block
- `:` - lambda expression

## It's not a bug, it's a feature!
Function calls where the only argument is a list "unpack" that list into arguments. For example: `foo(1, 2, 3)` is equivalent to `foo((1, 2, 3))`. This can be prevented by adding a comma at the end of the function arguments (`foo((1,2,3),)`

Lambda [expressions](expressions) cannot directly call themselves, even when assigned to a variable. For example, `foo = x:foo(x)` fails on call because `foo` is unset. It is usually better to use a builtin function for this (`loop`, `iter`, `seq`, etc). If needed, however, this can be circumvented by having the function take another function as an argument (`foo = (bar,x):bar(x)`) and then creating a second function that calls the first with itself (`real_foo = x:foo(foo, x)`)
