//! The [`App`] container and the [`Plugin`] trait.

use std::collections::HashMap;

use bevy_ecs::prelude::{IntoScheduleConfigs, Resource};
use bevy_ecs::schedule::Schedule;
use bevy_ecs::system::ScheduleSystem;
use bevy_ecs::world::World;

use crate::Stage;

/// Owns the ECS [`World`] and one [`Schedule`] per [`Stage`].
pub struct App {
    world: World,
    schedules: HashMap<Stage, Schedule>,
    startup_ran: bool,
}

impl App {
    /// Creates an empty [`App`] with one schedule per [`Stage`].
    pub fn new() -> Self {
        let schedules = [
            Stage::Startup,
            Stage::FixedUpdate,
            Stage::Update,
            Stage::Render,
        ]
        .into_iter()
        .map(|stage| (stage, Schedule::new(stage)))
        .collect();

        Self {
            world: World::new(),
            schedules,
            startup_ran: false,
        }
    }

    /// Adds a plugin to the [`App`], letting it register systems and resources.
    pub fn add_plugin(&mut self, plugin: &dyn Plugin) -> &mut Self {
        plugin.build(self);
        self
    }

    /// Registers systems for the given [`Stage`].
    pub fn add_systems<M>(
        &mut self,
        stage: Stage,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.schedules
            .entry(stage)
            .or_insert_with(|| Schedule::new(stage))
            .add_systems(systems);
        self
    }

    /// Inserts a resource into the [`World`], replacing any existing value.
    pub fn insert_resource<R: Resource>(&mut self, value: R) -> &mut Self {
        self.world.insert_resource(value);
        self
    }

    /// Returns a shared reference to the [`World`].
    pub fn world(&self) -> &World {
        &self.world
    }

    /// Returns a mutable reference to the [`World`].
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Advances the app by one frame.
    ///
    /// [`Stage::Startup`] runs once on the first call; [`Stage::FixedUpdate`],
    /// [`Stage::Update`], and [`Stage::Render`] run in that order on every call.
    pub fn update(&mut self) {
        if !self.startup_ran {
            if let Some(schedule) = self.schedules.get_mut(&Stage::Startup) {
                schedule.run(&mut self.world);
            }
            self.startup_ran = true;
        }

        for stage in [Stage::FixedUpdate, Stage::Update, Stage::Render] {
            if let Some(schedule) = self.schedules.get_mut(&stage) {
                schedule.run(&mut self.world);
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// A plugin that configures an [`App`] when added.
pub trait Plugin {
    /// Registers this plugin's systems and resources on `app`.
    fn build(&self, app: &mut App);
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::{Commands, ResMut, Resource};

    use super::{App, Plugin};
    use crate::Stage;

    #[derive(Resource, Default)]
    struct StartupCount(u32);

    fn count_startup(mut count: ResMut<StartupCount>) {
        count.0 += 1;
    }

    #[test]
    fn startup_runs_once() {
        let mut app = App::new();
        app.insert_resource(StartupCount::default());
        app.add_systems(Stage::Startup, count_startup);

        app.update();
        app.update();
        app.update();

        let count = app.world().get_resource::<StartupCount>().unwrap();
        assert_eq!(count.0, 1);
    }

    #[derive(Resource, Default)]
    struct Log(Vec<&'static str>);

    fn log_startup(mut log: ResMut<Log>) {
        log.0.push("Startup");
    }

    fn log_fixed_update(mut log: ResMut<Log>) {
        log.0.push("FixedUpdate");
    }

    fn log_update(mut log: ResMut<Log>) {
        log.0.push("Update");
    }

    fn log_render(mut log: ResMut<Log>) {
        log.0.push("Render");
    }

    #[test]
    fn stages_run_in_order() {
        let mut app = App::new();
        app.insert_resource(Log::default());
        app.add_systems(Stage::Startup, log_startup);
        app.add_systems(Stage::FixedUpdate, log_fixed_update);
        app.add_systems(Stage::Update, log_update);
        app.add_systems(Stage::Render, log_render);

        app.update();
        app.update();

        let log = app.world().get_resource::<Log>().unwrap();
        assert_eq!(
            log.0,
            vec![
                "Startup",
                "FixedUpdate",
                "Update",
                "Render",
                "FixedUpdate",
                "Update",
                "Render",
            ]
        );
    }

    #[derive(Resource, Default)]
    struct Marker;

    fn insert_marker(mut commands: Commands) {
        commands.insert_resource(Marker);
    }

    struct MarkerPlugin;

    impl Plugin for MarkerPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Stage::Startup, insert_marker);
        }
    }

    #[test]
    fn plugin_systems_run() {
        let mut app = App::new();
        app.add_plugin(&MarkerPlugin);
        app.update();

        assert!(app.world().get_resource::<Marker>().is_some());
    }
}
