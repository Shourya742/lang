use crate::{
    binding_def::BindingDef,
    env::Env,
    expr::Expr,
    func_def::{self, FuncDef},
    val::Val,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    BindingDef(BindingDef),
    FuncDef(FuncDef),
    Expr(Expr),
}

impl Stmt {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| Expr::new(s).map(|(s, expr)| (s, Self::Expr(expr))))
            .or_else(|_| FuncDef::new(s).map(|(s, func_def)| (s, Self::FuncDef(func_def))))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        match self {
            Self::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Val::Unit)
            }
            Self::Expr(expr) => expr.eval(env),
            Self::FuncDef(func_def) => {
                func_def.eval(env)?;
                Ok(Val::Unit)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        expr::{Number, Op, binding_usage::BindingUsage},
        func_def::FuncDef,
    };

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Stmt::new("let a = 10"),
            Ok((
                "",
                Stmt::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Number(Number(10)),
                }),
            )),
        );
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Stmt::new("1+1"),
            Ok((
                "",
                Stmt::Expr(Expr::Operation {
                    lhs: Box::new(Expr::Number(Number(1))),
                    rhs: Box::new(Expr::Number(Number(1))),
                    op: Op::Add,
                }),
            )),
        );
    }

    #[test]
    fn eval_binding_def() {
        assert_eq!(
            Stmt::BindingDef(BindingDef {
                name: "whatever".to_string(),
                val: Expr::Number(Number(-10)),
            })
            .eval(&mut Env::default()),
            Ok(Val::Unit),
        );
    }

    #[test]
    fn eval_expr() {
        assert_eq!(
            Stmt::Expr(Expr::Number(Number(5))).eval(&mut Env::default()),
            Ok(Val::Number(5)),
        );
    }

    #[test]
    fn parse_func_def() {
        assert_eq!(
            Stmt::new("fn identity x => x"),
            Ok((
                "",
                Stmt::FuncDef(FuncDef {
                    name: "identity".to_string(),
                    params: vec!["x".to_string()],
                    body: Box::new(Stmt::Expr(Expr::BindingUsage(BindingUsage {
                        name: "x".to_string(),
                    }))),
                }),
            )),
        );
    }

    #[test]
    fn eval_func_def() {
        assert_eq!(
            Stmt::FuncDef(FuncDef {
                name: "always_return_one".to_string(),
                params: Vec::new(),
                body: Box::new(Stmt::Expr(Expr::Number(Number(1)))),
            })
            .eval(&mut Env::default()),
            Ok(Val::Unit),
        );
    }
}
