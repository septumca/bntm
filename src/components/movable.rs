use macroquad::{prelude::*};

use crate::utils::EPSILON;


pub struct Movable {
  pub pos: Vec2,
  pub vel: Vec2,
  pub imp: Vec2,
  pub friction: f32,
  pub speed: f32,
  weight: f32,
  bounds: Rect,
}

impl Movable {
  pub fn new() -> Self {
    let size = (4., 4.);
    let pos = Vec2::ZERO;
    Self {
      pos,
      vel: Vec2::ZERO,
      imp: Vec2::ZERO,
      friction: 0.5,
      speed: 50.,
      weight: 100.,
      bounds: Rect::new(pos.x, pos.y, size.0, size.1)
    }
  }

  fn change_pos(&mut self, pos: Vec2) {
    self.pos = pos;
    self.bounds.move_to(self.pos - vec2(self.bounds.w / 2., self.bounds.h / 2.));
  }

  pub fn with_size(mut self, size: (f32, f32)) -> Self {
    self.bounds.w = size.0;
    self.bounds.h = size.1;
    self
  }

  pub fn with_pos(mut self, pos: Vec2) -> Self {
    self.change_pos(pos);
    self
  }

  pub fn with_vel(mut self, vel: Vec2) -> Self {
    self.vel = vel;
    self
  }

  pub fn set_vel(&mut self, vel: Vec2) {
    self.vel = vel;
  }

  pub fn add_impuls(&mut self, imp: Vec2) {
    self.imp += imp;
  }

  pub fn bounds(&self) -> &Rect {
    &self.bounds
  }

  pub fn next_vel_imp(&self, delta_t: f32) -> (Vec2, Vec2) {
    let new_imp = self.imp * self.friction;
    let impuls = if new_imp.length_squared() > EPSILON { new_imp } else { Vec2::ZERO };
    ((self.vel + impuls) * delta_t, impuls)
  }

  pub fn update(&mut self, delta_t: f32) {
    let (vel, imp) = self.next_vel_imp(delta_t);
    self.imp = imp;
    self.change_pos(self.pos + vel);
  }
}
