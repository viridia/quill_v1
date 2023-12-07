//! **Quill** is a UI framework for the Bevy game engine. It's meant to provide a simple API for
//! constructing reactive user interfaces, similar to frameworks like React and Solid, but built on
//! a foundation of Bevy ECS state management.

#![warn(missing_docs)]
mod cursor;
mod node_span;
mod plugin;
mod scrolling;
mod style;
mod view;

pub use cursor::Cursor;
pub use node_span::NodeSpan;
#[doc(inline)]
pub use prelude::*;
pub use scrolling::*;

/// Common imports
pub mod prelude {
    pub use crate::plugin::QuillPlugin;
    pub use crate::style::*;
    pub use crate::view::*;
}
