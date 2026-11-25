//! The [`Time`] resource that tracks frame timing.

use bevy_ecs::prelude::Resource;

/// Time information for the current frame.
///
/// The fields are filled in by a later task; for now they default to zero.
#[derive(Debug, Clone, Copy, PartialEq, Resource, Default)]
pub struct Time {
    /// Seconds since the previous frame.
    pub delta: f32,
    /// Seconds since the application started.
    pub elapsed: f32,
}
