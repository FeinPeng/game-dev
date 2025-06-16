use bevy_rapier2d::prelude::*;

pub const GROUP_SENSOR_DEAD_ZONE: Group = Group::GROUP_1; // 球的死亡区域

pub const GROUP_BALL: Group = Group::GROUP_2; // 球

pub const GROUP_ENEMY: Group = Group::GROUP_3; // 敌人

pub const GROUP_WALL: Group = Group::GROUP_4; // 墙

pub const GROUP_BRICK: Group = Group::GROUP_5; // 砖块

pub const GROUP_TRANSPARANT_WALL: Group = Group::GROUP_6; // 透明墙，用来阻挡砖块，但不阻挡球

pub const GROUP_DOOR: Group = Group::GROUP_7; // 门Sensor

pub const GROUP_ITEM: Group = Group::GROUP_8; // Item
