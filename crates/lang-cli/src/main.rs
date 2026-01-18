use std::io::{self, Write};

use lang::env::Env;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let mut input = String::new();
    let mut env = lang::env::Env::default();

    loop {
        write!(stdout, "-> ")?;
        stdout.flush()?;
        stdin.read_line(&mut input)?;

        match run(input.trim(), &mut env) {
            Ok(Some(val)) => {
                writeln!(stdout, "{}", val)?;
            }
            Ok(None) => {}
            Err(msg) => {
                writeln!(stderr, "{}", msg)?;
                stderr.flush()?;
            }
        }

        input.clear();
    }
}

fn run(input: &str, env: &mut Env) -> Result<Option<lang::val::Val>, String> {
    let parse = lang::parse(input).map_err(|msg| format!("Parse error: {}", msg))?;
    let evaluated = parse
        .eval(env)
        .map_err(|msg| format!("Evaluation error: {}", msg))?;
    if evaluated != lang::val::Val::Unit {
        return Ok(Some(evaluated));
    }
    Ok(None)
}
