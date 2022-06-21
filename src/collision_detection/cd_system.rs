use std::collections::HashSet;

use macroquad::{prelude::*};

use crate::{utils::rect_from_pos, components::movable::Movable};

pub const BOUNCE_VALUE: f32 = 100.;

pub type CDData = (usize, usize);

#[derive(PartialEq, Eq, Debug)]
pub enum CollisionAxis {
  X,
  Y,
  Both,
}


pub fn get_collision_axis(
  ma: &Movable,
  mb: &Movable, delta_t: f32
) -> CollisionAxis {
  let ra = rect_from_pos(
    ma.pos + vec2(ma.next_vel_imp(delta_t).0.x, 0.),
    (ma.bounds().w, ma.bounds().h)
  );
  let rb = rect_from_pos(
    mb.pos + vec2(mb.next_vel_imp(delta_t).0.x, 0.),
    (mb.bounds().w, mb.bounds().h)
  );

  if ra.overlaps(&rb) {
   return CollisionAxis::X;
  }

  let ra = rect_from_pos(
    ma.pos + vec2(0., ma.next_vel_imp(delta_t).0.y),
    (ma.bounds().w, ma.bounds().h)
  );
  let rb = rect_from_pos(
    mb.pos + vec2(0., mb.next_vel_imp(delta_t).0.y),
    (mb.bounds().w, mb.bounds().h)
  );

  if ra.overlaps(&rb) {
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

    pub fn get_just_collided(&self) -> HashSet<CDData> {
      &self.act_step - &self.last_step
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
