use std::cell::{Cell, RefCell};

use crate::{
  components::movable::Movable,
   collision_detection::cd_system::{
    get_collision_axis,
    CollisionAxis,
    BOUNCE_VALUE
  }
};
use macroquad::{prelude::*};

use super::{action::Action, ai::Ai};


const MOVE_PROXIMITY_EPSILON: f32 = 5.;

#[derive(Clone)]
pub enum ActorKind {
  Player,
  Enemy,
  Projectile
}

#[derive(Clone)]
pub enum ActorState {
  MovingTo(Vec2),
  Idle,
}


// #[derive(Clone)]
pub struct Actor {
  pub movable: RefCell<Movable>,
  pub kind: ActorKind,
  pub color: Color,
  pub ai: Option<Ai>,
  pub state: ActorState,
}


impl Actor {
  pub fn new(movable: Movable, color: Color, kind: ActorKind) -> Self {
    Self {
      state: ActorState::Idle,
      movable: RefCell::new(movable),
      color,
      kind,
      ai: None
    }
  }

  pub fn with_ai(mut self, ai: Ai) -> Self {
    self.ai = Some(ai);
    self
  }

  pub fn perform(&mut self) -> Option<Action> {
    match &self.ai {
      Some(_ai) => {
        None
      },
      None => {
        if is_mouse_button_pressed(MouseButton::Left) {
          return Some(Action::WalkTo(Vec2::from(mouse_position())));
        }
        None
      }
    }
  }

  pub fn set_state(&mut self, state: ActorState) {
    self.state = state;
    match self.state {
      ActorState::MovingTo(tp) => {
        self.movable.borrow_mut().set_vel_to_target(tp);
      },
      ActorState::Idle => {
        self.movable.borrow_mut().set_vel(Vec2::ZERO);
      },
    };
  }

  pub fn update(&mut self, delta_t: f32) {
    self.movable.borrow_mut().update(delta_t);

    match self.state {
      ActorState::MovingTo(tp) => {
        if self.movable.borrow().distance_to_squared(tp) < MOVE_PROXIMITY_EPSILON {
          self.set_state(ActorState::Idle);
        }
      },
      _ => {}
    };
  }
}


fn bounce_resolution(aa: &Actor, ab: &Actor, delta_t: f32) {
  let mut ma = aa.movable.borrow_mut();
  let mut mb = ab.movable.borrow_mut();

  let (impa, impb) = match get_collision_axis(&ma, &mb, delta_t) {
    CollisionAxis::X => {
      (vec2(mb.vel.x, ma.vel.y), vec2(ma.vel.x, mb.vel.y))
    },
    CollisionAxis::Y => {
      (vec2(ma.vel.x, mb.vel.y), vec2(mb.vel.x, ma.vel.y))
    },
    CollisionAxis::Both => {
      (vec2(mb.vel.x, mb.vel.y), vec2(ma.vel.x, ma.vel.y))
    },
  };

  let impa = impa / ma.weight * BOUNCE_VALUE;
  let impb = impb / mb.weight * BOUNCE_VALUE;

  ma.set_vel(Vec2::ZERO);
  ma.add_impuls(impa);

  mb.set_vel(Vec2::ZERO);
  mb.add_impuls(impb);
}

pub fn resolve_collision(aa: &Actor, ab: &Actor, delta_t: f32) {
  match (&aa.kind, &ab.kind) {
    (ActorKind::Enemy | ActorKind::Player, ActorKind::Projectile) => {
      //apply projectile
    },
    (ActorKind::Projectile, ActorKind::Enemy | ActorKind::Player) => {
      //apply projectile
    },
    (ActorKind::Enemy, ActorKind::Enemy)  => {
      bounce_resolution(aa, ab, delta_t);
    },
    (ActorKind::Enemy, ActorKind::Player)  => {
      //apply -hp
      bounce_resolution(aa, ab, delta_t);
    },
    (ActorKind::Player, ActorKind::Enemy)  => {
      //apply -hp
      bounce_resolution(aa, ab, delta_t);
    },
    _ => {}
  }
}
