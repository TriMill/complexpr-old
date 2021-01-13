use complexpr::*;
use rustyline::{
    error::ReadlineError,
    completion::Completer,
    highlight::Highlighter,
    validate::Validator,
    hint::Hinter,
    Editor, Helper
};
use std::sync::{Mutex, Arc};
use std::borrow::Cow;

const GOOD: &str = "\x1b[92m";
const ERROR: &str = "\x1b[91m";
const PROMPT: &str = "\x1b[94m";
const RESET: &str = "\x1b[0m";

fn main() {
    let ctx = Arc::new(Mutex::new(ctx_full()));
    println!("Use {}exit(){} or press {}Ctrl+D{} to exit",
        PROMPT, RESET, PROMPT, RESET);
    let mut rl = Editor::<CHelper>::new();
    rl.set_helper(Some(CHelper{ctx: ctx.clone()}));
    // prevent lag later due to lazy_static
    eval("1", &mut ctx.lock().unwrap()).unwrap();
    loop {
        let line = rl.readline(">> ");
        match line { 
            Ok(line) => {
                let mut ctx = ctx.lock().unwrap();
                let line = line.as_str();
                rl.add_history_entry(line);
                let result = eval(&line, &mut ctx);
                match result {
                    Ok(Value::Void) => (),
                    Ok(value) => {
                        println!("{:?}", value);
                        ctx.insert("_".to_owned(), value);
                    },
                    Err(e) => println!("{}{}Error: {}{}", RESET, ERROR, RESET, e)
                }
            },
            Err(ReadlineError::Interrupted) => (),
            Err(ReadlineError::Eof) => {
                println!("EOF");
                break
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

struct CHelper {
    ctx: Arc<Mutex<Context>>
}

impl Helper for CHelper {}
impl Hinter for CHelper { type Hint = String; }
impl Validator for CHelper {}

impl Highlighter for CHelper {
    fn highlight_prompt<'b,'s:'b,'p:'b>(&'b self, prompt: &'b str, _: bool) 
    -> Cow<'b, str> {
        Cow::from(RESET.to_owned() + PROMPT + prompt + RESET)
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        let parenmatch = match_parens(line, pos);
        if let Ok(matchpos) = parenmatch {
            let first = matchpos.min(pos);
            let last = matchpos.max(pos);
            Cow::from([
                &line[..first],
                GOOD, "(", RESET,
                &line[(first+1)..last],
                GOOD, ")", RESET,
                &line[(last+1)..]
            ].join(""))
        } else if let Err(true) = parenmatch {
            let c = line.chars().nth(pos).unwrap();
            Cow::from([
                &line[..pos],
                ERROR, &c.to_string(), RESET,
                &line[(pos+1)..]
            ].join(""))
        } else {
            Cow::from(line)
        }
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        let ch = line.chars().nth(pos);
        ch == Some('(') || ch == Some(')')
    }
}

fn match_parens(line: &str, pos: usize) -> Result<usize, bool> {
    let c = line.chars().nth(pos);
    if c == Some('(') {
        let mut counter = 1;
        let mut matchpos = usize::MAX;
        for (i,c) in line[(pos+1)..].chars().enumerate() {
            if c == '(' {
                counter += 1;
            } else if c == ')' {
                counter -= 1;
            }
            if counter == 0 {
                matchpos = i+pos+1;
                break
            }
        }
        if matchpos == usize::MAX {
            Err(true)
        } else {
            Ok(matchpos)
        }
    } else if c == Some(')') {
        let mut counter = 1;
        let mut matchpos = usize::MAX;
        for (i,c) in line[..pos].chars().collect::<Vec<char>>().iter().enumerate().rev() {
            if *c == '(' {
                counter -= 1;
            } else if *c == ')' {
                counter += 1;
            }
            if counter == 0 {
                matchpos = i;
                break
            }
        }
        if matchpos == usize::MAX {
            Err(true)
        } else {
            Ok(matchpos)
        }
    } else {
        Err(false)
    }
}

impl Completer for CHelper {
    type Candidate = String;
    fn complete(&self, line: &str, pos: usize, _: &rustyline::Context<'_>)
    -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut res = String::new();
        for ch in line[..pos].chars().rev() {
            match ch {
                '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => res.push(ch),
                _ => break
            }
        }
        let res: String = res.chars().rev().collect();
        let mut keys = self.ctx.lock().unwrap().keys()
            .filter(|x| x.starts_with(&res))
            .cloned()
            .collect::<Vec<String>>();
        keys.sort();
        Ok((pos - res.len(), keys))
    }
}

