use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    ball::DamageCoefficient,
    brick::Brick,
    events::{Damage, DamageEvent},
    world::map::room::control::ChooseState,
    GameState,
};

use super::Pride;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PrideHitEvent>().add_systems(
            Update,
            (
                handle_ball_collision_event,
                handle_pride_hit,
                restore_pride_color,
            )
                .run_if(in_state(GameState::Gaming))
                .run_if(in_state(ChooseState::PreChoosing)),
        );
    }
}

#[derive(Event)]
pub struct PrideHitEvent(pub Entity);

#[derive(Component)]
pub struct HitEffect {
    pub original_color: Color,
    pub timer: Timer,
}

fn handle_ball_collision_event(
    mut collision_events: EventReader<CollisionEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    mut pride_hit_events: EventWriter<PrideHitEvent>,
    q_pride: Query<Entity, With<Pride>>,
    q_parent: Query<&Parent>,
    q_damage: Query<&Damage>,
    q_damage_coefficient: Query<&DamageCoefficient>,
    q_brick: Query<Entity, With<Brick>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity_a, entity_b, _) = event {
            if q_pride.contains(*entity_a) {
                // a为Enemy, b为other
                let a_parent = q_parent.get(*entity_a).unwrap().get();
                let b_parent = q_parent.get(*entity_b).unwrap().get();

                if q_brick.contains(b_parent) {
                    let brick_entity = q_brick.get_single().unwrap();
                    damage_events.send(DamageEvent {
                        offender: a_parent,
                        victim: brick_entity,
                        damage: Damage::new(20.0),
                    });
                } else {
                    let b_damage_coe = q_damage_coefficient.get(b_parent).unwrap();
                    let b_damage = *q_damage.get(b_parent).unwrap();
                    damage_events.send(DamageEvent {
                        offender: b_parent,
                        victim: a_parent,
                        damage: Damage::new(b_damage.value * b_damage_coe.0),
                    });
                    pride_hit_events.send(PrideHitEvent(a_parent));
                }
            }
            if q_pride.contains(*entity_b) {
                // b为Enemy, a为other
                let a_parent = q_parent.get(*entity_a).unwrap().get();
                let b_parent = q_parent.get(*entity_b).unwrap().get();
                if q_brick.contains(a_parent) {
                    let brick_entity = q_brick.get_single().unwrap();
                    damage_events.send(DamageEvent {
                        offender: b_parent,
                        victim: brick_entity,
                        damage: Damage::new(20.0),
                    });
                } else {
                    let a_damage_coe = q_damage_coefficient.get(a_parent).unwrap();
                    let a_damage = *q_damage.get(a_parent).unwrap();
                    damage_events.send(DamageEvent {
                        offender: a_parent,
                        victim: b_parent,
                        damage: Damage::new(a_damage.value * a_damage_coe.0),
                    });
                    pride_hit_events.send(PrideHitEvent(b_parent));
                }
            }
        }
    }
}

fn handle_pride_hit(
    mut commands: Commands,
    mut pride_hit_events: EventReader<PrideHitEvent>,
    mut pridees: Query<(&mut Sprite, Option<&mut HitEffect>)>,
) {
    for PrideHitEvent(pride_entity) in pride_hit_events.read() {
        if let Ok((mut sprite, hit_effect)) = pridees.get_mut(*pride_entity) {
            match hit_effect {
                // 已有受击效果，重置计时器
                Some(mut effect) => {
                    effect.timer.reset();
                }
                // 无受击效果：记录原始颜色并添加组件
                None => {
                    let original_color = sprite.color;
                    sprite.color = Color::Srgba(Srgba::RED);
                    commands.entity(*pride_entity).insert(HitEffect {
                        original_color,
                        timer: Timer::from_seconds(0.1, TimerMode::Once),
                    });
                }
            }
        }
    }
}

fn restore_pride_color(
    mut commands: Commands,
    mut pridees: Query<(Entity, &mut Sprite, &mut HitEffect)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut effect) in &mut pridees {
        effect.timer.tick(time.delta());
        if effect.timer.finished() {
            sprite.color = effect.original_color;
            commands.entity(entity).remove::<HitEffect>();
        }
    }
}
