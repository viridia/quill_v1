/// The value of a style attribute, which can be either a constant or a variable.
#[derive(Debug, Clone, PartialEq)]
pub enum StyleExpr<T> {
    /// An expression that has already been cast to the correct type.
    Constant(T),
}

impl<T> StyleExpr<T>
where
    T: Copy,
{
    /// Return the value of this style expression, but only if it's a constant. Otherwise
    /// returns an error.
    pub fn get(&self) -> Result<T, StyleError> {
        match self {
            StyleExpr::Constant(val) => Ok(*val),
        }
    }
}

pub enum StyleError {}
