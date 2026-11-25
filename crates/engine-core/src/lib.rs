//! Core engine types, shared interfaces, and the main loop.
//!
//! `engine-core` provides the foundation every other crate builds on:
//! an [`App`] that owns a [`bevy_ecs`] [`World`] and its schedules, a
//! [`Plugin`] trait, and the shared [`Transform`] and [`Time`] types.

pub mod app;
pub mod stage;
pub mod time;
pub mod transform;

pub use app::{App, Plugin};
pub use stage::Stage;
pub use time::Time;
pub use transform::Transform;

pub use glam;
pub use glam::{Quat, Vec3};

pub use bevy_ecs::prelude::*;
