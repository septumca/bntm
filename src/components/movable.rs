use macroquad::{prelude::*};

use crate::{utils::EPSILON, collision_detection::cd_system::{get_collision_axis, CollisionAxis, BOUNCE_VALUE}};

#[derive(Debug)]
pub struct Movable {
  pos: Vec2,
  vel: Vec2,
  pub imp: Vec2,
  pub speed: f32,
  pub friction: f32,
  pub weight: f32,
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
      friction: 0.75,
      speed: 50.,
      weight: 80.,
      bounds: Rect::new(pos.x, pos.y, size.0, size.1)
    }
  }

  fn update_bounds(&mut self) {
    self.bounds.move_to(
      self.pos - vec2(self.bounds.w / 2., self.bounds.h / 2.)
    );
  }

  pub fn with_size(mut self, size: (f32, f32)) -> Self {
    self.bounds.w = size.0;
    self.bounds.h = size.1;
    self.update_bounds();
    self
  }

  pub fn with_speed(mut self, speed: f32) -> Self {
    self.speed = speed;
    self.vel = self.vel.normalize_or_zero() * self.speed;
    self
  }

  pub fn with_pos(mut self, pos: Vec2) -> Self {
    self.pos = pos;
    self.update_bounds();
    self
  }

  pub fn with_vel(mut self, vel: Vec2) -> Self {
    self.set_vel(vel);
    self
  }

  pub fn distance_to_squared(&self, target: Vec2) -> f32 {
    (self.pos - target).length_squared()
  }

  pub fn set_vel_to_target(&mut self, target: Vec2) {
    self.set_vel(target - self.pos);
  }

  pub fn set_vel(&mut self, vel: Vec2) {
    self.vel = vel.normalize_or_zero() * self.speed;
  }

  pub fn add_impuls(&mut self, imp: Vec2) {
    self.imp += imp;
  }

  pub fn bounds(&self) -> Rect {
    self.bounds
  }

  pub fn pos(&self) -> Vec2 {
    self.pos
  }

  pub fn offset_bounds(&self, offset: Vec2) -> Rect {
    self.bounds.offset(offset)
  }

  pub fn next_vel_imp(&self, delta_t: f32) -> (Vec2, Vec2) {
    let new_imp = self.imp * self.friction;
    let impuls = if new_imp.length_squared() > EPSILON {
      new_imp
    } else {
      Vec2::ZERO
    };
    ((self.vel + self.imp) * delta_t, impuls)
  }

  pub fn update(&mut self, delta_t: f32) {
    let (vel, imp) = self.next_vel_imp(delta_t);
    self.imp = imp;
    self.pos += vel;
    self.update_bounds();
  }
}

pub fn shove_resolution(ma: &mut Movable, mb: &mut Movable) {
  ma.add_impuls((ma.pos - mb.pos) * BOUNCE_VALUE);
  mb.add_impuls((mb.pos - ma.pos) * BOUNCE_VALUE);

}

pub fn bounce_resolution(ma: &mut Movable, mb: &mut Movable, delta_t: f32) {
  let (impa, impb) = match get_collision_axis(ma, mb, delta_t) {
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
  fn with_speed() {
    let (_pos, _vel, mut m) = create();
    m = m.with_speed(100.);

    assert_eq!(m.vel.length(), 100.);
  }

  #[test]
  fn with_vel() {
    let (_pos, _vel, mut m) = create();
    m = m.with_speed(100.);
    m = m.with_vel(vec2(0., 12.));

    assert_eq!(m.vel.x, 0.);
    assert_eq!(m.vel.y, 100.);
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

  #[test]
  fn update() {
    let (pos, _vel, m) = create();
    let mut m = m.with_speed(100.);

    m.update(1.);

    assert_eq!(m.pos.x, pos.x + 100.);

    m.add_impuls(vec2(0., 100.));

    m.update(1.);

    assert_eq!(m.pos.x, pos.x + 200.);
    assert_eq!(m.pos.y, pos.y + 100.);

    m.update(1.);

    assert_eq!(m.pos.y, pos.y + 100. + 100. * m.friction);
  }

  #[test]
  fn update_with_impuls() {
    let (pos, _vel, m) = create();
    let mut m = m.with_speed(100.);
    m.add_impuls(vec2(0., 100.));

    m.update(1.);

    assert_eq!(m.pos.x, pos.x + 100.);
    assert_eq!(m.pos.y, pos.y + 100.);

    m.update(1.);

    assert_eq!(m.pos.x, pos.x + 200.);
    assert_eq!(m.pos.y, pos.y + 100. + 100. * m.friction);
  }
}