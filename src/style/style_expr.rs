/// The value of a style attribute, which can be either a constant or a variable.
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
    /// Return the value of this style attribute in the current context. If the expression
    /// is a constant, then it merely returns it verbatim. If the expression is a style variable,
    /// however, it will search the containing scopes for a definition of that variable.
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
