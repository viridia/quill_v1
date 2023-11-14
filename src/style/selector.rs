use std::fmt;

use bevy::utils::HashSet;
use winnow::{
    ascii::space0,
    combinator::{alt, opt, preceded, repeat, separated},
    stream::AsChar,
    token::{one_of, take_while},
    PResult, Parser,
};

/// Represents a predicate which can be used to conditionally style a node.
/// Selectors support a subset of CSS grammar:
///
/// * Current element (`&`)
/// * Classname matching
/// * Parent element (`>`) pattern
/// * Multiple patterns can be specified by commas.
///
/// Examples:
/// ```css
///   &
///   &.name
///   .state > &
///   .state > * > &.name
/// ```
///
/// Selectors must target the "current element": this means that the "`&`" selector is
/// required, and it can only appear on the last term of the selector expression. This means
/// that parent elements cannot implicitly style their children; child elements must have styles
/// explicitly specified (although those styles can be conditional on the state of their parents).
#[derive(Debug, PartialEq, Clone)]
pub enum Selector {
    /// If we reach this state, it means the match was successful
    Accept,

    /// Match an element with a specific class name.
    Class(String, Box<Selector>),

    /// Reference to the current element.
    Current(Box<Selector>),

    /// Reference to the parent of this element.
    Parent(Box<Selector>),

    /// List of alternate choices.
    Either(Vec<Box<Selector>>),
}

fn parent<'s>(input: &mut &'s str) -> PResult<()> {
    (space0, '>', space0).void().parse_next(input)
}

fn class_name<'s>(input: &mut &'s str) -> PResult<&'s str> {
    preceded(
        '.',
        (
            one_of(AsChar::is_alpha),
            take_while(0.., (AsChar::is_alphanum, '-', '_')),
        ),
    )
    .recognize()
    .parse_next(input)
}

fn simple_selector<'s>(input: &mut &'s str) -> PResult<(Option<char>, Vec<&'s str>)> {
    (opt(alt(('*', '&'))), repeat(0.., class_name)).parse_next(input)
}

fn combo_selector<'s, 'a>(input: &mut &'s str) -> PResult<Box<Selector>> {
    let mut sel = Box::new(Selector::Accept);
    let (prefix, classes) = simple_selector.parse_next(input)?;
    for cls in classes {
        sel = Box::new(Selector::Class(cls[1..].into(), sel));
    }
    if let Some(ch) = prefix {
        if ch == '&' {
            sel = Box::new(Selector::Current(sel));
        }
    }
    Ok(sel)
}

impl<'a> Selector {
    pub fn parser<'s>(input: &mut &'s str) -> PResult<Box<Selector>> {
        Self::either.parse_next(input)
    }

    fn either<'s>(input: &mut &'s str) -> PResult<Box<Selector>> {
        separated(1.., Self::desc_selector, (space0, ',', space0))
            .map(|mut items: Vec<Box<Selector>>| {
                if items.len() == 1 {
                    items.pop().unwrap()
                } else {
                    Box::new(Selector::Either(items))
                }
            })
            .parse_next(input)
    }

    fn desc_selector<'s>(input: &mut &'s str) -> PResult<Box<Selector>> {
        let mut sel = combo_selector.parse_next(input)?;
        while parent.parse_next(input).is_ok() {
            sel = Box::new(Selector::Parent(sel));
            let (prefix, classes) = simple_selector.parse_next(input)?;
            for cls in classes {
                sel = Box::new(Selector::Class(cls[1..].into(), sel));
            }
            if let Some(ch) = prefix {
                if ch == '&' {
                    sel = Box::new(Selector::Current(sel));
                }
            }
        }

        Ok(sel)
    }

    /// Returns a number indicating how many levels up the entity ancestor hierarchy we might
    /// have to search to look for classes.
    pub(crate) fn depth(&self) -> usize {
        match self {
            Selector::Accept => 1,
            Selector::Class(_, next) => next.depth(),
            Selector::Current(next) => next.depth(),
            Selector::Parent(next) => next.depth() + 1,
            Selector::Either(opts) => opts.iter().map(|next| next.depth()).max().unwrap_or(0),
        }
    }

    /// Given an array of ElementClasses components representing the ancestor chain, match the
    /// selector expression with the classes.
    pub(crate) fn match_classes(&self, classes: &[HashSet<String>]) -> bool {
        match self {
            Selector::Accept => true,
            Selector::Class(cls, next) => {
                classes.first().map(|f| f.contains(cls)).unwrap_or(false)
                    && next.match_classes(classes)
            }
            Selector::Current(next) => next.match_classes(classes),
            Selector::Parent(next) => next.match_classes(&classes[1..]),
            Selector::Either(opts) => opts.iter().any(|next| next.match_classes(classes)),
        }
    }
}

impl std::str::FromStr for Selector {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Selector::parser
            .parse(input.trim())
            .map(|a| *a)
            .map_err(|e| e.to_string())
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Selector::Accept => Ok(()),
            Selector::Current(prev) => {
                // Because 'current' comes first, reverse order
                let mut str = String::with_capacity(64);
                let mut p = prev.as_ref();
                while let Selector::Class(name, desc) = p {
                    str.insert_str(0, name);
                    str.insert_str(0, ".");
                    p = desc.as_ref()
                }
                str.insert_str(0, "&");
                write!(f, "{}{}", p, str)
            }

            Selector::Class(name, prev) => write!(f, "{}.{}", prev, name),
            Selector::Parent(prev) => match prev.as_ref() {
                Selector::Parent(_) => write!(f, "{}* > ", prev),
                _ => write!(f, "{} > ", prev),
            },
            Selector::Either(items) => {
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    item.fmt(f)?
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_current() {
        assert_eq!(
            "&".parse::<Selector>().unwrap(),
            Selector::Current(Box::new(Selector::Accept))
        );
    }

    #[test]
    fn test_serialize() {
        assert_eq!(
            Selector::Current(Box::new(Selector::Accept)).to_string(),
            "&",
        );
        assert_eq!(
            Selector::Class("x".into(), Box::new(Selector::Accept)).to_string(),
            ".x",
        );
        assert_eq!(
            ".foo > &.bar".parse::<Selector>().unwrap().to_string(),
            ".foo > &.bar",
        );
        assert_eq!(
            ".foo > .bar.baz".parse::<Selector>().unwrap().to_string(),
            ".foo > .bar.baz",
        );
        assert_eq!(
            ".foo > * > .bar".parse::<Selector>().unwrap().to_string(),
            ".foo > * > .bar",
        );
        assert_eq!(
            ".foo > &.bar.baz".parse::<Selector>().unwrap().to_string(),
            ".foo > &.bar.baz",
        );
        assert_eq!(
            ".a.b.c > .d.e.f > &.g.h.i"
                .parse::<Selector>()
                .unwrap()
                .to_string(),
            ".a.b.c > .d.e.f > &.g.h.i",
        );
        assert_eq!(
            ".foo, .bar".parse::<Selector>().unwrap().to_string(),
            ".foo, .bar",
        );
    }

    #[test]
    fn test_parse_current_class() {
        assert_eq!(
            "&.foo".parse::<Selector>().unwrap(),
            Selector::Current(Box::new(Selector::Class(
                "foo".into(),
                Box::new(Selector::Accept)
            )))
        );
    }

    #[test]
    fn test_parse_class() {
        assert_eq!(
            ".foo".parse::<Selector>().unwrap(),
            Selector::Class("foo".into(), Box::new(Selector::Accept))
        );
    }

    #[test]
    fn test_parse_parent() {
        assert_eq!(
            "&.foo > .bar".parse::<Selector>().unwrap(),
            Selector::Class(
                "bar".into(),
                Box::new(Selector::Parent(Box::new(Selector::Current(Box::new(
                    Selector::Class("foo".into(), Box::new(Selector::Accept))
                )))))
            )
        );

        assert_eq!(
            ".foo > &.bar".parse::<Selector>().unwrap(),
            Selector::Current(Box::new(Selector::Class(
                "bar".into(),
                Box::new(Selector::Parent(Box::new(Selector::Class(
                    "foo".into(),
                    Box::new(Selector::Accept)
                ))))
            )))
        );
    }

    #[test]
    fn test_either() {
        assert_eq!(
            "&.foo, .bar".parse::<Selector>().unwrap(),
            Selector::Either(vec!(
                Box::new(Selector::Current(Box::new(Selector::Class(
                    "foo".into(),
                    Box::new(Selector::Accept)
                )))),
                Box::new(Selector::Class("bar".into(), Box::new(Selector::Accept)))
            ))
        );
    }
}
