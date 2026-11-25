//! The [`Transform`] component that places entities in space.

use bevy_ecs::prelude::Component;
use glam::{Quat, Vec3};

/// Position, orientation, and scale of an entity.
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct Transform {
    /// Translation in world space.
    pub translation: Vec3,
    /// Rotation as a unit quaternion.
    pub rotation: Quat,
    /// Scale along each axis; `Vec3::ONE` means no scaling.
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_identity() {
        let transform = Transform::default();
        assert_eq!(transform.translation, Vec3::ZERO);
        assert_eq!(transform.rotation, Quat::IDENTITY);
        assert_eq!(transform.scale, Vec3::ONE);
    }
}
