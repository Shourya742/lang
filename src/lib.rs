use crate::{env::Env, val::Val};

pub mod binding_def;
pub mod env;
pub mod expr;
pub mod statements;
pub mod utils;
pub mod val;


pub struct Parse(statements::Stmt);

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = statements::Stmt::new(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err("input was not consumed fully by parser".to_string())
    }
}

impl Parse {
    pub fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}