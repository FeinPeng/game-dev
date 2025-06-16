use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use std::f32::consts::PI;
use std::ops::Neg;

use crate::ball::Ball;
use crate::brick::brick_ball::BRICK_BALL_SIZE;
use crate::brick::BallInHand;
use crate::{GameState, WINDOW_HEIGHT, WINDOW_WIDTH};

use super::{
    OriginalVel, TargetForce, DAMPING_COEFFICIENT, FLUID_DENSITY, MAGNUS_COEFFICIENT, STIFFNESS,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ContactEvent>()
            .add_event::<ContactEventPart2>()
            .add_event::<ForceEvent>()
            .add_systems(
                Update,
                (
                    ball_collision_handle,
                    contact_event_handle,
                    contact_event_part2_handle,
                    apply_elastic_force,
                )
                    .chain()
                    .run_if(in_state(GameState::Gaming)),
            )
            .add_systems(Update, (despawn_ball).run_if(in_state(GameState::Gaming)));
    }
}

#[derive(Event)]
pub struct ContactEvent {
    other_children_entity: Entity,
    ball_children_entity: Entity,
    direction: Vec2,
    depth: f32,
}

#[derive(Event)]
pub struct ContactEventPart2 {
    other_velocity: Velocity,
    ball_parent_entity: Entity,
    direction: Vec2,
    depth: f32,
    friction: f32,
    resititution: f32,
    ball_mass: ColliderMassProperties,
    other_mass: ColliderMassProperties,
}

fn ball_collision_handle(
    rapier_context: ReadRapierContext,
    mut q_ball_children: Query<
        (&Children, &OriginalVel, &mut Velocity),
        (With<Ball>, Without<BallInHand>),
    >,
    mut contact_event_writer: EventWriter<ContactEvent>,
    time: Res<Time>,
) {
    for (children, ball_original_vel, mut ball_velocity) in q_ball_children.iter_mut() {
        // collider是附加在ball的子实体上
        let ball_children_entity = children[0];
        let mut direction = Vec2::ZERO;
        let mut depth = 0.0;
        let mut in_contact = false;
        let mut other_children_entity: Option<Entity> = None;

        for contact_pair in rapier_context
            .single()
            .contact_pairs_with(ball_children_entity)
        {
            // 根据Collision groups 来判断有没有active_contact
            if contact_pair.has_any_active_contact() {
                in_contact = true;
                for manifold in contact_pair.manifolds() {
                    depth = manifold.point(0).unwrap().dist().abs();
                    if contact_pair.collider1() == ball_children_entity {
                        direction = manifold.local_n2();
                        other_children_entity = Some(contact_pair.collider2());
                    } else {
                        direction = manifold.local_n1();
                        other_children_entity = Some(contact_pair.collider1());
                    }

                    for s in &manifold.raw.data.solver_contacts {
                        println!("point: {}", s.point);
                        println!("dist: {}", s.dist);
                        println!("fric: {}", s.friction);
                        println!("rest: {}", s.restitution);
                        println!("tangent_vel: {}", s.tangent_velocity);
                        println!("warmstart_impluse: {}", s.warmstart_impulse);
                        println!("warmstart_tangnt_impluse: {}", s.warmstart_tangent_impulse);
                    }
                }
            }
            // println!("normal: {}", direction);
            // q_parent.get(contact_pair.collider2()).ok().unwrap().get()
        }

        if in_contact {
            contact_event_writer.send(ContactEvent {
                other_children_entity: other_children_entity.unwrap(),
                ball_children_entity,
                direction,
                depth,
            });
        } else {
            // 动能守恒
            ball_velocity.linvel =
                ball_velocity.linvel.normalize_or_zero() * ball_original_vel.0.linvel.length();
            apppy_magnus_force(&mut ball_velocity, time.delta_secs());
        }
    }
}

fn contact_event_handle(
    mut contact_event_reader: EventReader<ContactEvent>,
    mut contact_event_writer: EventWriter<ContactEventPart2>,
    q_parent: Query<&Parent, With<Collider>>,
    q_restitution: Query<&Restitution>,
    q_friction: Query<&Friction>,
    q_velocity: Query<&Velocity>,
    q_mass: Query<&ColliderMassProperties>,
) {
    for &ContactEvent {
        other_children_entity,
        ball_children_entity,
        direction,
        depth,
    } in contact_event_reader.read()
    {
        let ball_parent_entity = q_parent.get(ball_children_entity).ok().unwrap().get();
        let other_parent_entity = q_parent.get(other_children_entity).ok().unwrap().get();

        let other_velocity = q_velocity.get(other_parent_entity).unwrap();

        let ball_mass = q_mass.get(ball_children_entity).unwrap();
        let other_mass = q_mass.get(other_children_entity).unwrap();

        let ball_fric_coe = q_friction
            .get(ball_children_entity)
            .ok()
            .unwrap()
            .coefficient;
        let other_fric_coe = q_friction
            .get(other_children_entity)
            .ok()
            .unwrap()
            .coefficient;
        let ball_rest_coe = q_restitution
            .get(ball_children_entity)
            .ok()
            .unwrap()
            .coefficient;
        let other_rest_coe = q_restitution
            .get(other_children_entity)
            .ok()
            .unwrap()
            .coefficient;

        let avg_fric_coe = (ball_fric_coe + other_fric_coe) / 2.0;
        let avg_rest_coe = (ball_rest_coe + other_rest_coe) / 2.0;
        contact_event_writer.send(ContactEventPart2 {
            other_velocity: *other_velocity,
            ball_parent_entity,
            direction,
            depth,
            friction: avg_fric_coe,
            resititution: avg_rest_coe,
            ball_mass: *ball_mass,
            other_mass: *other_mass,
        });
    }
}

#[derive(Event)]
pub struct ForceEvent {
    ball_entity: Entity,
    other_velocity: Velocity,
    direction: Vec2,
    friction: f32,
    restitution: f32,
    ball_mass: ColliderMassProperties,
    other_mass: ColliderMassProperties,
}

fn contact_event_part2_handle(
    mut contact_event_reader: EventReader<ContactEventPart2>,
    mut force_events: EventWriter<ForceEvent>,
    mut q_ball_target_force: Query<&mut TargetForce, With<Ball>>,
) {
    for &ContactEventPart2 {
        other_velocity,
        ball_parent_entity,
        direction,
        depth,
        friction,
        resititution,
        ball_mass,
        other_mass,
    } in contact_event_reader.read()
    {
        let mut target_force = if let Ok(tf) = q_ball_target_force.get_mut(ball_parent_entity) {
            tf
        } else {
            continue;
        };
        target_force.0 += depth * STIFFNESS * direction;
        force_events.send(ForceEvent {
            ball_entity: ball_parent_entity,
            other_velocity,
            direction,
            friction,
            restitution: resititution,
            ball_mass,
            other_mass,
        });
    }
}

fn apply_elastic_force(
    mut force_events: EventReader<ForceEvent>,
    mut q_velocity: Query<&mut Velocity, With<Ball>>,
    mut q_ball_target_force: Query<&mut TargetForce, With<Ball>>,
    time: Res<Time>,
) {
    for &ForceEvent {
        ball_entity,
        other_velocity,
        direction,
        friction,
        restitution,
        ball_mass,
        other_mass,
    } in force_events.read()
    {
        let mut ball_velocity = if let Ok(res) = q_velocity.get_mut(ball_entity) {
            res
        } else {
            panic!()
        };
        let mut target_force = if let Ok(res) = q_ball_target_force.get_mut(ball_entity) {
            res
        } else {
            panic!()
        };
        // println!("target force: {}", target_force.0);

        let ball_mass_properties = match ball_mass {
            ColliderMassProperties::MassProperties(mp) => mp,
            _ => unreachable!(),
        };
        let other_mass_properties = match other_mass {
            ColliderMassProperties::MassProperties(mp) => mp,
            _ => unreachable!(),
        };
        // 相对速度 （假设球体无旋转，简化模型）
        let v_rel = ball_velocity.linvel - other_velocity.linvel;
        // println!(
        //     "v_rel: {}, ball_v: {}, other_v:{}",
        //     v_rel, ball_velocity.linvel, other_velocity.linvel
        // );
        // 切向相对速度
        let v_tan = v_rel - (v_rel * direction) * direction;
        // println!("v_tan: {}", v_tan);
        // 法向冲量
        let j_n = restitution * target_force.0 * time.delta_secs();
        // 切向冲量
        let j_t = v_tan * ball_mass_properties.mass * friction * time.delta_secs();
        // 跟新线速度
        ball_velocity.linvel += (j_t.neg() * 10.0 + j_n) / ball_mass_properties.mass;
        // 跟新角速度
        let s: f32 = if v_rel.x < 0.0 {
            1.0
        } else if v_rel.x > 0.0 {
            -1.0
        } else {
            0.0
        };
        ball_velocity.angvel += j_t.length() * s / (ball_mass_properties.mass);
        ball_velocity.angvel -= DAMPING_COEFFICIENT * ball_velocity.angvel * time.delta_secs();
        *target_force = TargetForce::default();
    }
}

fn apppy_magnus_force(ball_velocity: &mut Velocity, delta_secs: f32) {
    let magnus_force = FLUID_DENSITY
        * PI
        * ball_velocity.angvel.neg()
        * ball_velocity.linvel.length()
        * (BRICK_BALL_SIZE / 2.0)
        * MAGNUS_COEFFICIENT
        * Vec2::new(1.0, 0.0)
        * 0.001;

    // Updateprintln!("magnus f: {}", magnus_force);
    ball_velocity.angvel -= DAMPING_COEFFICIENT * ball_velocity.angvel * delta_secs;
    ball_velocity.linvel += magnus_force * delta_secs;
}

fn despawn_ball(mut commands: Commands, q_ball: Query<(&Transform, Entity), With<Ball>>) {
    for (ball_transform, ball_entity) in q_ball.iter() {
        let pos = ball_transform.translation;
        let h_w_w = WINDOW_WIDTH / 2.0;
        let h_w_h = WINDOW_HEIGHT / 2.0;
        if pos.x < -h_w_w || pos.x > h_w_w || pos.y > h_w_h || pos.y < -(h_w_h + 100.0) {
            commands.entity(ball_entity).despawn_recursive();
        }
    }
}
