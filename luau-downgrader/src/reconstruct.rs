use std::{backtrace, fmt::{format, Debug}};

use luau_ast_rs::ast::*;

fn compOpToStr(op: CompOpKind) -> &'static str {
    match op {
        CompOpKind::Add => "+",
        CompOpKind::Concat => "..",
        CompOpKind::Div => "/",
        CompOpKind::Mod => "%",
        CompOpKind::Mul => "*",
        CompOpKind::Pow => "^",
        CompOpKind::Sub => "-"
    }
}

fn binOpToStr(op: BinOpKind) -> &'static str {
    match op {
        BinOpKind::Add => "+",
        BinOpKind::And => " and ",
        BinOpKind::Concat => "..",
        BinOpKind::Div => "/",
        BinOpKind::Eq => "==",
        BinOpKind::Ge => ">=",
        BinOpKind::Gt => ">",
        BinOpKind::Le => "<=",
        BinOpKind::Lt => "<",
        BinOpKind::Mod => "%",
        BinOpKind::Mul => "*",
        BinOpKind::Ne => "~=",
        BinOpKind::Or => " or ",
        BinOpKind::Pow => "^",
        BinOpKind::Sub => "-"
    }
}

fn getVarName(var: Var) -> String {
    let name = match var { Var::Name(name) => Some(name), _ => None };
    return name.expect("Invalid varname struct");
}

fn getCallArgs(ca: CallArgs) -> Vec<Expr> {
    let name: Option<Vec<Expr>> = match ca { CallArgs::Exprs(name) => Some(name), _ => None };
    return name.expect("Invalid call args?");
}

fn makeFuncArgs(callArgs: CallArgs) -> String {
    let mut args = String::new();
    let exprs = getCallArgs(callArgs);
    for expr in exprs {
        args += &format!("{},", reconstructExpression(expr));
    }
    return args.get(0..args.len() - 1).unwrap_or("").to_owned();
}

fn reconstructExpression(expr: Expr) -> &String {
    match expr {
        Expr::BinOp(bop) => {
            let bo: BinOp = *bop;
            let lhs = reconstructExpression(bo.lhs);
            let rhs = reconstructExpression(bo.rhs);
            let op = binOpToStr(bo.op);
            return &format!("{}{}{}", lhs, op, rhs);
        }
        Expr::Bool(b) => if b { &"true".to_owned() } else { &"false".to_owned() },
        Expr::Call(c) => {
            return &format!("{}({})", reconstructExpression(c.func), makeFuncArgs(c.args));
        },
        Expr::Function(body) => {
            
        }
    }
}

// Reconstruct source from a statement.
// Also downgrades non-5.1 features to support 5.1
pub fn reconstructStmt(stmt: Stmt) {
    let mut statement: String = String::new();
    match stmt {
        Stmt::CompOp(co_stmt) => {
            let varn: &str = match co_stmt.lhs {
                Var::Name(ref name) => name,
                _ => "",
            };
            let val: &String = reconstructExpression(*co_stmt.rhs);
            statement = varn+"="+varn+compOpToStr(co_stmt.op)+val;
        }
        Stmt::Assign(a_stmt) => {

        }
        Stmt::Break(b_stmt) => {

        }
        Stmt::Call(cl_stmt) => {

        }
        Stmt::Continue(cn_stmt) => {

        }
        Stmt::Do(d_stmt ) => {

        }
        Stmt::For(f_stmt) => {

        }
        Stmt::ForIn(fi_stmt) => {

        }
        Stmt::FunctionDef(fd_stmt) => {

        }
        Stmt::If(i_stmt) => {

        }
        Stmt::Local(l_stmt) => {

        }
        Stmt::LocalFunctionDef(lf_stmt) => {

        }
        Stmt::Repeat(rp_stmt) => {

        }
        Stmt::Return(rt_stmt) => {

        }
        Stmt::TypeDef(td_stmt) => {
            // Pass, no types :rage:
        }
        Stmt::While(w_stmt) => {

        }
    }
}