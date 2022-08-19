use std::collections::HashMap;

use crate::{
    ast::{Expr, ExprKind, Ident, Program},
    util::{Error, PResult},
};

pub const MAX_ARG_COUNT: usize = 'Z' as usize - 'A' as usize + 1;

pub type Env = HashMap<String, bool>;

pub fn eval_program(program: &Program) -> PResult<bool> {
    let env = args_to_env(&program.args);
    eval_expr(&program.expr, &env)
}

pub fn args_to_env(args: &[bool]) -> Env {
    assert!(args.len() <= MAX_ARG_COUNT);
    ('A'..)
        .map(String::from)
        .zip(args.iter().copied())
        .collect()
}

fn eval_expr(expr: &Expr, env: &Env) -> PResult<bool> {
    match &expr.kind {
        ExprKind::Var(ident) => eval_ident(ident, env),
        ExprKind::App(ident, args) => eval_app(ident, args, env),
    }
}

fn eval_ident(Ident { ident, span }: &Ident, env: &Env) -> PResult<bool> {
    match env.get(ident) {
        Some(bool) => Ok(*bool),
        None => Err(Error::new(format!("`{ident}` is not defined"), *span)),
    }
}

fn eval_app(Ident { ident, span }: &Ident, args: &[Expr], env: &Env) -> PResult<bool> {
    match ident.as_str() {
        "not" => {
            if args.len() == 1 {
                Ok(!eval_expr(&args[0], env)?)
            } else {
                Err(Error::new(
                    format!("`{ident}` requires single argument"),
                    *span,
                ))
            }
        }
        "and" => args
            .iter()
            .try_fold(true, |a, expr| Ok(a && eval_expr(expr, env)?)),
        "or" => args
            .iter()
            .try_fold(false, |a, expr| Ok(a || eval_expr(expr, env)?)),
        _ => Err(Error::new(
            format!("cannot call undefined function `{ident}`"),
            *span,
        )),
    }
}
