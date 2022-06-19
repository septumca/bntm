use std::collections::HashMap;
use std::collections::{HashSet};

use macroquad::prelude::*;
use macroquad::telemetry;

type BTElem = (usize, Rect);
pub type Actor = (Movable, Color);
type CDData = (usize, usize);

const EPSILON: f32 = 0.004;


#[derive(Clone, Copy)]
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

  pub fn bounds(&self) -> Rect {
    self.bounds
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

pub fn generate_random(count: usize) -> HashMap<usize, Actor> {
  let mut movables: HashMap<usize, Actor>  = HashMap::new();

  for k in 0..count {
    let m = Movable::new().with_size((6., 6.));
    let x = rand::gen_range::<f32>(0. + m.bounds.w / 2., screen_width() - m.bounds.h / 2.);
    let y = rand::gen_range::<f32>(0. + m.bounds.w / 2., screen_height() - m.bounds.h / 2.);

    let vx = rand::gen_range::<f32>(-1., 1.);
    let vy = rand::gen_range::<f32>(-1., 1.);

    let c = Color::new(
      rand::gen_range::<f32>(0., 1.),
      rand::gen_range::<f32>(0., 1.),
      rand::gen_range::<f32>(0., 1.),
      1.
    );

    let m = m.with_pos(vec2(x, y)).with_vel(vec2(vx, vy).normalize() * m.speed);

    movables.insert(k, (m, c));
  }
  movables
}

pub fn generate_two_opposite() -> HashMap<usize, Actor> {
  let mut movables: HashMap<usize, Actor>  = HashMap::new();

  let ma = Movable::new().with_size((32., 32.));
  let mb = Movable::new().with_size((32., 32.));

  let posa = vec2(screen_width() / 4., screen_height() / 2.);
  let vela = vec2(1., 0.) * ma.speed;
  let posb = vec2(screen_width() / 4. * 3., screen_height() / 2.);
  let velb = vec2(-1., 0.) * ma.speed;
  let ca = RED;
  let cb = BLUE;

  movables.insert(0, (ma.with_pos(posa).with_vel(vela), ca));
  movables.insert(1, (mb.with_pos(posb).with_vel(velb), cb));

  movables
}

pub fn generate_two_inside() -> HashMap<usize, Actor> {
  let mut movables: HashMap<usize, Actor>  = HashMap::new();

  let ma = Movable::new().with_size((32., 32.));
  let mb = Movable::new().with_size((32., 32.));

  let posa = vec2(screen_width() / 2. - ma.bounds.w / 4., screen_height() / 2.);
  let vela = vec2(1., 0.) * ma.speed;
  let posb = vec2(screen_width() / 2. +  mb.bounds.w / 4., screen_height() / 2.);
  let velb = vec2(-1., 0.) * mb.speed;
  let ca = RED;
  let cb = BLUE;

  movables.insert(0, (ma.with_pos(posa).with_vel(vela), ca));
  movables.insert(1, (mb.with_pos(posb).with_vel(velb), cb));

  movables
}

#[derive(PartialEq, Eq, Debug)]
pub enum CollisionAxis {
  X,
  Y,
  Both,
}

#[derive(Clone)]
pub enum BTreeSplit {
  Horizontal,
  Vertical,
}

pub struct BTree {
  bounds: Rect,
  split: BTreeSplit,
  elems: Vec<BTElem>,
  children: Option<(Box<BTree>, Box<BTree>)>,
  treshold: usize,
}

impl BTree {
  pub fn new(bounds: Rect, treshold: usize, split: BTreeSplit) -> Self {
    BTree { bounds, split, treshold, elems: vec![], children: None }
  }

  pub fn split(&self) -> (Rect, Rect, BTreeSplit) {
    match self.split {
      BTreeSplit::Horizontal => {
        let ra = Rect::new(self.bounds.x, self.bounds.y, self.bounds.w, self.bounds.h/2.);
        let rb = Rect::new(self.bounds.x, self.bounds.y + self.bounds.h/2., self.bounds.w, self.bounds.h/2.);
        (ra, rb, BTreeSplit::Vertical)
      },
      BTreeSplit::Vertical => {
        let ra = Rect::new(self.bounds.x, self.bounds.y, self.bounds.w/2., self.bounds.h);
        let rb = Rect::new(self.bounds.x + self.bounds.w/2., self.bounds.y, self.bounds.w/2., self.bounds.h);
        (ra, rb, BTreeSplit::Horizontal)
      }
    }
  }

  pub fn insert(&mut self, value: BTElem) {
    if !self.bounds.overlaps(&value.1) {
      return;
    }

    match &mut self.children {
      &mut Some(ref mut ch) => {
        ch.0.insert(value);
        ch.1.insert(value);
      },
      &mut None => {
        if self.elems.len() + 1 > self.treshold {
          let (ra, rb, split) = self.split();
          let mut bta = BTree::new(ra, self.treshold, split.clone());
          let mut btb = BTree::new(rb, self.treshold, split.clone());

          for elem in self.elems.clone() {
            bta.insert(elem);
            btb.insert(elem);
          }
          self.elems.clear();

          bta.insert(value);
          btb.insert(value);
          self.children = Some((Box::new(bta), Box::new(btb)));
        } else {
          self.elems.push(value);
        }
      },
    }
  }

  pub fn get_collisions(&self) -> HashSet<CDData> {
    let _z = telemetry::ZoneGuard::new("BTree - get_collisions");

    match &self.children {
      Some(ch) => {
        &ch.0.get_collisions() | &ch.1.get_collisions()
      },
      None => {
        let mut collisions = HashSet::new();
        for index_a in 0..self.elems.len() {
          for index_b in (index_a+1)..self.elems.len() {
            if self.elems[index_a].1.overlaps(&self.elems[index_b].1) {
              collisions.insert((self.elems[index_a].0, self.elems[index_b].0));
            }
          }
        }
        collisions
      }
    }
  }

  pub fn draw(&self, thickness: f32) {
    draw_rectangle_lines(self.bounds.x, self.bounds.y, self.bounds.w, self.bounds.h, thickness, YELLOW);
    match &self.children {
      Some(ch) => {
        ch.0.draw(thickness + 0.5);
        ch.1.draw(thickness + 0.5);
      },
      None => {
        draw_text(format!("{}", self.elems.len()).as_str(), self.bounds.x + 2. , self.bounds.y + 10., 16., WHITE);
      }
    }
  }
}


pub fn get_collision_axis(ma: &Movable, mb: &Movable, delta_t: f32) -> CollisionAxis {
  let ra = rect_from_pos(ma.pos + vec2(ma.next_vel_imp(delta_t).0.x, 0.), (ma.bounds.w, ma.bounds.h));
  let rb = rect_from_pos(mb.pos + vec2(mb.next_vel_imp(delta_t).0.x, 0.), (mb.bounds.w, mb.bounds.h));

  if ra.overlaps(&rb) {
   return CollisionAxis::X;
  }

  let ra = rect_from_pos(ma.pos + vec2(0., ma.next_vel_imp(delta_t).0.y), (ma.bounds.w, ma.bounds.h));
  let rb = rect_from_pos(mb.pos + vec2(0., mb.next_vel_imp(delta_t).0.y), (mb.bounds.w, mb.bounds.h));

  if ra.overlaps(&rb) {
    return CollisionAxis::Y;
  }

  return CollisionAxis::Both;
}

pub struct CDSystem {
  last_step: HashSet<CDData>,
  act_step: HashSet<CDData>,
}

impl CDSystem {
    pub fn new() -> Self {
      Self { last_step: HashSet::new(), act_step: HashSet::new() }
    }

    pub fn update(&mut self, collisions: HashSet<CDData>) {
      let _z = telemetry::ZoneGuard::new("CDSystem - update");
      self.last_step = self.act_step.clone();
      self.act_step = collisions;
    }

    pub fn get_just_collided(&self) -> HashSet<CDData> {
      let _z = telemetry::ZoneGuard::new("CDSystem - get_just_collided");
      &self.act_step - &self.last_step
    }
}

#[inline(always)]
pub fn rect_from_pos(pos: Vec2, size: (f32, f32)) -> Rect {
  Rect::new(pos.x - size.0 / 2., pos.y - size.0 / 2., size.0, size.1)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn collision_y() {
    let dt: f32 = 0.0165;
    let ma = Movable::new().with_size((16., 16.)).with_pos(vec2(630.0, 446.0633)).with_vel(vec2(0.0, -50.0));
    let mb = Movable::new().with_size((16., 16.)).with_pos(vec2(636.0, 430.0));

    let ca = get_collision_axis(&ma, &mb, dt);

    assert_eq!(ca, CollisionAxis::Y);
  }

  #[test]
  fn collision_corner() {
    let dt: f32 = 0.0165;
    let ma = Movable::new().with_size((16., 16.)).with_pos(vec2(619.72003, 413.71997)).with_vel(vec2(50.0, 50.0));
    let mb = Movable::new().with_size((16., 16.)).with_pos(vec2(636.0, 430.0));

    let ca = get_collision_axis(&ma, &mb, dt);

    assert_eq!(ca, CollisionAxis::Both);
  }
}
