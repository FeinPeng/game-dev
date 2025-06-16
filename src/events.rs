use bevy::prelude::*;

use crate::{brick::stats::Pressure, GameState};

#[derive(Component, Clone, Copy, PartialEq, PartialOrd)]
pub struct Damage {
    pub value: f32,
}

impl Damage {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Event)]
pub struct DamageEvent {
    pub offender: Entity,
    pub victim: Entity,
    pub damage: Damage,
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>().add_systems(
            Update,
            (handle_damage_events).run_if(in_state(GameState::Gaming)),
        );
    }
}

fn handle_damage_events(
    mut events: EventReader<DamageEvent>,
    mut q_pressure: Query<&mut Pressure>,
) {
    for event in events.read() {
        if let Ok(mut pressure) = q_pressure.get_mut(event.victim) {
            pressure.current = (pressure.current + event.damage.value).clamp(0.0, pressure.max);
            // println!("pressure: {}", pressure.current);
        }
    }
}
