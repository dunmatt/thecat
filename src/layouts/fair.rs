//! The fair window layout is one that attempts to give all windows equal space.
//!
//! It is not always possible to give all windows equal space and tile the available
//! area, so there are two variants of the fair layout: horizontal and vertical.
//! Vertical fair layouts put the lucky windows above one another, and horizontal
//! fair layouts put them beside one another.
//!

/// This is the window aspect ratio that the tiling algorithm is trying to approximate.
pub const TARGET_ASPECT_RATIO: f32 = 16.0 / 9.0;
