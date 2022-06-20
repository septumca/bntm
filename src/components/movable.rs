use macroquad::{prelude::*};

use crate::utils::EPSILON;


pub struct Movable {
  pub pos: Vec2,
  pub vel: Vec2,
  pub imp: Vec2,
  pub speed: f32,
  friction: f32,
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

  fn update_bounds(&mut self) {
    self.bounds.move_to(self.pos - vec2(self.bounds.w / 2., self.bounds.h / 2.));
  }

  pub fn with_size(mut self, size: (f32, f32)) -> Self {
    self.bounds.w = size.0;
    self.bounds.h = size.1;
    self.update_bounds();
    self
  }

  pub fn with_pos(mut self, pos: Vec2) -> Self {
    self.pos = pos;
    self.update_bounds();
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
    ((self.vel + self.imp) * delta_t, impuls)
  }

  pub fn update(&mut self, delta_t: f32) {
    let (vel, imp) = self.next_vel_imp(delta_t);
    self.imp = imp;
    self.pos += vel;
    self.update_bounds();
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  const  DT: f32 = 0.0165;

  fn create() -> (Vec2, Vec2, Movable) {
    let pos = vec2(50., 50.);
    let m = Movable::new().with_pos(pos.clone());
    let vel = vec2(1., 0.) * m.speed;

    (pos, vel, m.with_vel(vel.clone()))
  }

  #[test]
  fn next_vel_imp() {
    let (_pos, orig_vel, mut m) = create();
    let orig_imp = vec2(10., 0.);
    m.add_impuls(orig_imp.clone());

    let (vel, imp) = m.next_vel_imp(DT);
    assert_eq!(vel, (orig_vel + orig_imp) * DT);
    assert_eq!(imp, orig_imp * m.friction);
  }

  #[test]
  fn with_size() {
    let (pos, _vel, m) = create();
    let size = (10. , 12.);
    let m = m.with_size(size.clone());

    assert_eq!(m.bounds.left(), pos.x - size.0 / 2.);
    assert_eq!(m.bounds.right(), pos.x + size.0 / 2.);
    assert_eq!(m.bounds.top(), pos.y - size.1 / 2.);
    assert_eq!(m.bounds.bottom(), pos.x + size.1 / 2.);
  }

  #[test]
  fn with_pos() {
    let (_pos, _vel, m) = create();
    let pos = Vec2::ONE;
    let size = (10. , 12.);
    let m = m.with_size(size.clone()).with_pos(pos);

    assert_eq!(m.bounds.left(), pos.x - size.0 / 2.);
    assert_eq!(m.bounds.right(), pos.x + size.0 / 2.);
    assert_eq!(m.bounds.top(), pos.y - size.1 / 2.);
    assert_eq!(m.bounds.bottom(), pos.x + size.1 / 2.);
  }
}