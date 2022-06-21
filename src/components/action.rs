
use macroquad::{prelude::*};

use super::actor::{Actor, ActorState};

pub enum Action {
  WalkTo(Vec2),
  Idle,
  Follow(usize),
}

pub enum ActionResult {
  Alter(Action),
  Succeed,
  Fail
}

impl Action {
  pub fn apply(&self, source: &mut Actor) -> ActionResult {
    match self {
      &Action::WalkTo(target) => {
        source.set_state(ActorState::MovingTo(target));
      },
      &Action::Idle => {},
      &Action::Follow(_ak) => {},
    };

    ActionResult::Succeed
  }
}
