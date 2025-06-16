use std::ops::Neg;

use bevy::prelude::*;

use crate::ball::Ball;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inventory::new(
            6,
            &[Ball::Tennis, Ball::Tennis, Ball::Tennis],
        ));
    }
}

#[derive(Resource)]
pub struct Inventory {
    capacity: usize,
    index: usize, // 从1开始
    pub balls: Vec<Option<Ball>>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            capacity: 3,
            index: 0,
            balls: Vec::new(),
        }
    }
}

impl Inventory {
    pub fn new(capacity: usize, balls: &[Ball]) -> Self {
        let mut invent = Self {
            capacity,
            index: balls.len(),
            balls: Vec::new(),
        };
        for i in 0..capacity {
            invent.balls.push(balls.get(i).cloned());
        }
        invent
    }

    pub fn push(&mut self, ball: Ball) -> Result<(), ()> {
        if self.capacity == 0 {
            return Err(());
        }
        if self.index < self.capacity {
            self.balls[self.index] = Some(ball);
            self.index += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn pop(&mut self) -> Option<Ball> {
        if self.index > 0 {
            self.index -= 1;
            self.balls[self.index].take()
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<Ball> {
        if self.index > 0 {
            self.balls.get(self.index - 1).cloned().unwrap()
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.index == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn expansion(&mut self, num: usize) {
        self.capacity += num;
        for _ in 0..num {
            self.balls.push(None);
        }
        // println!("{:#?}", self.balls);
    }

    pub fn redusing(&mut self, num: usize) {
        if self.capacity == 0 {
            return;
        }
        let diff = self.capacity as i32 - num as i32;
        let mut n: i32 = num as i32;
        if diff < 0 {
            self.capacity = 0;
            n = diff.neg();
        } else {
            self.capacity -= num;
        }
        if self.capacity < self.index {
            self.index = self.capacity;
        }
        for _ in 0..n {
            self.balls.pop();
        }
        println!("{:#?}", self.balls);
    }

    pub fn clear(&mut self) {
        for ball in self.balls.iter_mut() {
            *ball = None;
        }
    }

    pub fn get(&self, index: usize) -> Option<Ball> {
        if index > self.capacity - 1 {
            None
        } else {
            self.balls.get(index).cloned().unwrap()
        }
    }
}
