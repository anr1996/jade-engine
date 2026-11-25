//! The [`Stage`] enum that groups systems by phase of a frame.

use bevy_ecs::schedule::ScheduleLabel;

/// The phases of a single frame.
///
/// Each stage owns its own schedule and runs in a fixed order inside
/// [`App::update`](crate::App::update).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub enum Stage {
    /// Runs once, on the very first [`App::update`](crate::App::update) call.
    Startup,
    /// Runs every frame at a fixed timestep, before [`Stage::Update`].
    FixedUpdate,
    /// Runs every frame for ordinary game logic.
    Update,
    /// Runs every frame after [`Stage::Update`], reserved for rendering.
    Render,
}
