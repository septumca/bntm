

use std::{collections::HashMap, cell::RefCell};

use macroquad::prelude::*;
use crate::{Movable, utils::EPSILON};

use super::movable::{shove_resolution};

pub const PLAYER_KEY: usize = 0;

pub enum ActorKind {
  Player,
  Enemy
}

pub enum ActorState {
  Idle,
  MovingTo(Vec2)
}


pub struct Actor {
  pub movable: Movable,
  pub color: Color,
  pub kind: ActorKind,
  pub state: ActorState,
}

impl Actor {
  pub fn new(movable: Movable, color: Color, kind: ActorKind) -> Self {
    Self {
      movable,
      color,
      kind,
      state: ActorState::Idle
    }
  }

  pub fn new_player(movable: Movable) -> Self {
    Self::new(movable, BLUE, ActorKind::Player)
  }

  pub fn new_enemy(movable: Movable) -> Self {
    Self::new(movable, RED, ActorKind::Enemy)
  }

  pub fn decide(&mut self, actors: &HashMap<usize, RefCell<Actor>>) -> Option<ActorState> {
    match &self.kind {
      ActorKind::Player => {
        if is_mouse_button_pressed(MouseButton::Left) {
          let tp = Vec2::from(mouse_position());
          if self.movable.distance_to_squared(tp) > EPSILON  {
            return Some(ActorState::MovingTo(tp));
          }
        }
        if let ActorState::MovingTo(tp) = self.state {
          if self.movable.distance_to_squared(tp) < EPSILON  {
            return Some(ActorState::Idle);
          }
        }

        None
      },
      ActorKind::Enemy => {
        actors
          .get(&PLAYER_KEY)
          .map(|p| ActorState::MovingTo(p.borrow().movable.pos()))
      },
    }
  }

  pub fn set_state(&mut self, state: ActorState) {
    match state {
      ActorState::MovingTo(tp) => {
        self.movable.set_vel_to_target(tp);
      },
      ActorState::Idle => {
        self.movable.set_vel(Vec2::ZERO);
      }
    }
    self.state = state;
  }

  pub fn update(&mut self, delta_t: f32) {
    self.movable.update(delta_t);
  }
}


pub fn resolve_collision(aa: &mut Actor, ab: &mut Actor, _delta_t: f32) {
  match (&aa.kind, &ab.kind) {
    // (ActorKind::Enemy, ActorKind::Enemy)  => {
    //   //bounce
    // },
    // (ActorKind::Enemy, ActorKind::Player)  => {
    //   //apply -hp
    //   bounce_resolution(aa, ab, delta_t);
    // },
    // (ActorKind::Player, ActorKind::Enemy)  => {
    //   //apply -hp
    //   bounce_resolution(aa, ab, delta_t);
    // },
    _ => {
      shove_resolution(&mut aa.movable, &mut ab.movable);
    }
  }
}