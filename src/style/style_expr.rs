#[derive(Debug, Clone)]
pub enum StyleExpr<T> {
    /// An expression that has already been cast to the correct type.
    Constant(T),
    /// A reference to a named variable "${varname}".
    Var(String),
}

impl<T> StyleExpr<T>
where
    T: Copy,
{
    // TODO: Add parameters for ancestor list so we can evaluate vars and classes.
    pub fn eval(&self) -> Result<T, StyleError> {
        match self {
            StyleExpr::Constant(val) => Ok(*val),
            StyleExpr::Var(_)  => {
                todo!("Implement style vars")
            }
            // StyleExpr::Expr(expr) => match expr.coerce() {
            //     Some(val) => Ok(val),
            //     None => todo!("Implement evaluation"),
            // },
        }
    }
}

pub enum StyleError {}
