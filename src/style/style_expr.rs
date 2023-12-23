use bevy::{render::color::Color, ui};

use super::tokens::{StyleToken, TokenLookup, TokenValue};

/// The value of a style attribute, which can be either a constant or a variable.
#[derive(Debug, Clone, PartialEq)]
pub enum StyleExpr<T> {
    /// An expression that has already been cast to the correct type.
    Constant(T),
    /// A reference to a named style token "${token}".
    Token(StyleToken),
}

impl<T> StyleExpr<T>
where
    T: Copy,
{
    // /// Return the value of this style expression in the current context. If the expression
    // /// is a constant, then it merely returns it verbatim. If the expression is a style token,
    // /// however, it will search the containing scopes for a definition of that token.
    // pub fn eval(&self, lookup: &TokenLookup) -> Result<T, StyleError> {
    //     match self {
    //         StyleExpr::Constant(val) => Ok(*val),
    //         StyleExpr::Token(tok) => {
    //             match  lookup.find::<T>(tok) {
    //                 Some(val) => val.coerce().ok_or(err!()),
    //                 None => todo!(),
    //             }
    //         }
    //         // StyleExpr::Expr(expr) => match expr.coerce() {
    //         //     Some(val) => Ok(val),
    //         //     None => todo!("Implement evaluation"),
    //         // },
    //     }
    // }

    /// Return the value of this style expression, but only if it's a constant. Otherwise
    /// returns an error.
    pub fn get(&self) -> Result<T, StyleError> {
        match self {
            StyleExpr::Constant(val) => Ok(*val),
            StyleExpr::Token(_) => {
                panic!("Style tokens not implemented for this type.")
            }
        }
    }
}

pub trait StyleExprEval<T> {
    fn eval(&self, lookup: &TokenLookup) -> Result<T, StyleError>;
}

impl StyleExprEval<Option<Color>> for StyleExpr<Option<Color>> {
    fn eval(&self, lookup: &TokenLookup) -> Result<Option<Color>, StyleError> {
        match self {
            StyleExpr::Constant(val) => Ok(*val),
            StyleExpr::Token(tok) => {
                match  lookup.find(tok) {
                    Some(TokenValue::Color(val)) => Ok(val),
                    Some(_) => todo!(),
                    None => todo!(),
                }
            }
            // StyleExpr::Expr(expr) => match expr.coerce() {
            //     Some(val) => Ok(val),
            //     None => todo!("Implement evaluation"),
            // },
        }
    }
}

impl StyleExprEval<ui::Val> for StyleExpr<ui::Val> {
    fn eval(&self, lookup: &TokenLookup) -> Result<ui::Val, StyleError> {
        match self {
            StyleExpr::Constant(val) => Ok(*val),
            StyleExpr::Token(tok) => {
                match  lookup.find(tok) {
                    Some(TokenValue::Length(val)) => Ok(val),
                    Some(_) => todo!(),
                    None => todo!(),
                }
            }
            // StyleExpr::Expr(expr) => match expr.coerce() {
            //     Some(val) => Ok(val),
            //     None => todo!("Implement evaluation"),
            // },
        }
    }
}

pub enum StyleError {}
