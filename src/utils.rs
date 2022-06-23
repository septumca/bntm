use crate::components::actor::PLAYER_KEY;
use core::cell::RefCell;
use crate::components::actor::Actor;
use crate::components::movable::Movable;
use std::collections::HashMap;

use macroquad::prelude::*;



pub const EPSILON: f32 = 0.004;

pub fn generate_player_and_enemies(enemy_count: usize) -> HashMap<usize, RefCell<Actor>> {
  let mut actors: HashMap<usize, RefCell<Actor>>  = HashMap::new();

  let pm = Movable::new()
    .with_size((32., 32.))
    .with_pos(vec2(screen_width() / 2., screen_height() / 2.))
    .with_speed(70.)
    ;
  actors.insert(PLAYER_KEY, RefCell::new(Actor::new_player(pm)));

  for k in 0..enemy_count {
    let m = Movable::new().with_size((32., 32.));
    let x = rand::gen_range::<f32>(
      0. + m.bounds().w / 2.,
      screen_width() - m.bounds().h / 2.
    );
    let y = rand::gen_range::<f32>(
      0. + m.bounds().w / 2.,
      screen_height() - m.bounds().h / 2.
    );

    actors.insert(
      k+PLAYER_KEY+1,
      RefCell::new(
        Actor::new_enemy(m.with_pos(vec2(x, y)).with_speed(50.))
      )
    );
  }

  actors
}
