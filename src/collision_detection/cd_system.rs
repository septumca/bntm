use std::collections::HashSet;

use macroquad::{prelude::*};

use crate::{components::movable::Movable};

pub const BOUNCE_VALUE: f32 = 4.;

type CDData = (usize, usize);
type CDElem = (usize, Rect);

#[derive(PartialEq, Eq, Debug)]
pub enum CollisionAxis {
  X,
  Y,
  Both,
}

pub fn get_collisions(elems: &Vec<CDElem>) -> HashSet<CDData> {
  let mut collisions = HashSet::new();
  for index_a in 0..elems.len() {
    for index_b in (index_a+1)..elems.len() {
      if elems[index_a].1.overlaps(&elems[index_b].1) {
        collisions.insert((elems[index_a].0, elems[index_b].0));
      }
    }
  }
  collisions
}

pub fn get_collision_axis(
  ma: &Movable,
  mb: &Movable, delta_t: f32
) -> CollisionAxis {
  let ra = ma.offset_bounds(vec2(ma.next_vel_imp(delta_t).0.x, 0.));
  let rb = mb.offset_bounds(vec2(mb.next_vel_imp(delta_t).0.x, 0.));
  let x_overlaps = ra.overlaps(&rb);

  let ra = ma.offset_bounds(vec2(0., ma.next_vel_imp(delta_t).0.y));
  let rb = mb.offset_bounds(vec2(0., mb.next_vel_imp(delta_t).0.y));
  let y_overlaps = ra.overlaps(&rb);

  if x_overlaps && y_overlaps {
    return CollisionAxis::Both;
  }

  if x_overlaps {
    return CollisionAxis::X;
   }

  if y_overlaps {
    return CollisionAxis::Y;
  }

  CollisionAxis::Both
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
      self.last_step = self.act_step.clone();
      self.act_step = collisions;
    }

    pub fn get_collided(&self) -> HashSet<CDData> {
      self.act_step.clone()
    }

    pub fn get_just_collided(&self) -> HashSet<CDData> {
      &self.act_step - &self.last_step
    }

    pub fn just_collided(&self, value: CDData) -> bool {
      !self.last_step.contains(&value)
    }
}


#[cfg(test)]
mod tests {
  use super::*;
  const DT: f32 = 0.0165;

  mod cd_system {
    use super::*;

    fn create() -> CDSystem {
      CDSystem::new()
    }

    #[test]
    fn update() {
      let mut cd = create();
      let collisions: HashSet<CDData> = HashSet::from([(0, 1), (1, 2)]);
      let collisions2: HashSet<CDData> = HashSet::from([(3, 4)]);

      cd.update(collisions.clone());
      assert_eq!(cd.act_step, collisions);

      cd.update(collisions2.clone());
      assert_eq!(cd.last_step, collisions);
      assert_eq!(cd.act_step, collisions2);
    }

    #[test]
    fn just_collided() {
      let mut cd = create();
      let collisions: HashSet<CDData> = HashSet::from([(0, 1), (1, 2)]);
      let collisions2: HashSet<CDData> = HashSet::from([(3, 4), (0, 1)]);

      cd.update(collisions.clone());
      cd.update(collisions2.clone());
      assert_eq!(cd.get_just_collided(), HashSet::from([(3, 4)]));
    }
  }

  #[test]
  fn collision_y() {
    let ma = Movable::new()
      .with_size((16., 16.))
      .with_pos(vec2(630.0, 446.0633))
      .with_vel(vec2(0.0, -50.0));
    let mb = Movable::new()
      .with_size((16., 16.))
      .with_pos(vec2(636.0, 430.0));

    let ca = get_collision_axis(&ma, &mb, DT);

    assert_eq!(ca, CollisionAxis::Y);
  }

  #[test]
  fn collision_corner() {
    let ma = Movable::new()
      .with_size((16., 16.))
      .with_pos(vec2(619.72003, 413.71997))
      .with_vel(vec2(50.0, 50.0));
    let mb = Movable::new()
      .with_size((16., 16.))
      .with_pos(vec2(636.0, 430.0));

    let ca = get_collision_axis(&ma, &mb, DT);

    assert_eq!(ca, CollisionAxis::Both);
  }
}
