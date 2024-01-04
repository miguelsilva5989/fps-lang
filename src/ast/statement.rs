use super::arithmetic::Expr;


#[derive(Debug)]
pub enum Statement {
    ArithmeticExpr(Expr),
    Print(Expr),
    Assign { id: String, expr: Expr },
}
