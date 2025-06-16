mod init;
mod splash;

use bevy::prelude::*;

use crate::audio::Volume;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((init::InitPlugin, splash::SplashPlugin))
            .insert_resource(DisplayQuality::Medium);
    }
}

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
