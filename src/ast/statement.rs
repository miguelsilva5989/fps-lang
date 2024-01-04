use super::arithmetic::Expr;

pub enum Statement {
    Arithmetic(Expr),
    Print(Expr),
    Assign { id: String, expr: Expr },
}
