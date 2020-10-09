//! The Cat is my favorite window manager.
//!
//! The Cat's mother is the lovely Penrose TWM library, and this library is its father.  Where
//! Penrose is inclusive and permits many things, this library is particular, it specifies the
//! things I would like my WM to do.
//!

#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// The default workspace names.
pub const DEFAULT_WORKSPACE_NAMES: &[&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

pub mod bars;
pub mod layouts;
pub mod widgets;
