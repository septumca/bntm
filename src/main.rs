use crate::components::actor::Actor;
use components::{actor::resolve_collision, action::Action};
use macroquad::{prelude::*};
use std::{collections::HashMap};
use macroquad::telemetry;

mod utils;
mod collision_detection;
mod components;
// mod timer;
// mod animation;

use utils::*;
use collision_detection::{
  cd_system::{CDSystem},
  btree::{BTree}
};


fn window_conf() -> Conf {
  Conf {
      window_title: "BNTM".to_owned(),
      window_width: 1280,
      window_height: 860,
      high_dpi: false,
      ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let elements = 1024*8;
  let treshold = 128;
  let mut actors: HashMap<usize, Actor> = generate_player_and_enemies(5);
  let mut cdsystem = CDSystem::new();


  loop {
    let delta_t = get_frame_time();
    if is_key_released(KeyCode::A) {
      actors = generate_random(elements);
    }

    if is_key_released(KeyCode::S) {
      actors = generate_two_opposite();
    }

    if is_key_released(KeyCode::D) {
      actors = generate_two_inside();
    }

    clear_background(BLACK);

    if actors.is_empty() {
      next_frame().await;
      continue;
    }

    {
      let mut bt = BTree::root(
        Rect::new(0., 0., screen_width(), screen_height()),
        treshold
      );

      #[cfg(debug_assertions)]
      let _z = telemetry::ZoneGuard::new("bt");

      {
        let _z = telemetry::ZoneGuard::new("BTree - insert");
        for (k, a) in &actors {
          bt.insert((*k, a.movable.bounds()));
        }
      }

      cdsystem.update(bt.get_collisions());

      #[cfg(debug_assertions)]
      bt.draw(1.);
    }

    {
      #[cfg(debug_assertions)]
      let _z = telemetry::ZoneGuard::new("collision detection and resoultion");

      for (ka, kb) in cdsystem.get_just_collided() {
        let mut aa = actors.get_mut(&ka).unwrap().clone();
        let mut ab = actors.get_mut(&kb).unwrap().clone();

        resolve_collision(&mut aa, &mut ab, delta_t);

        actors.insert(ka, aa);
        actors.insert(kb, ab);
      }
    }

    {
      #[cfg(debug_assertions)]
      let _z = telemetry::ZoneGuard::new("update");

      let mut actions: Vec<(usize, Action)> = vec![];

      for (k, a) in actors.iter_mut() {
        if let Some(action) = a.perform() {
          actions.push((*k, action));
        }

        a.update(delta_t);
        let m = &mut a.movable;

        if m.bounds().right() > screen_width() || m.bounds().left() < 0. {
          m.vel.x = -m.vel.x;
        }
        if m.bounds().bottom() > screen_height() || m.bounds().top() < 0. {
          m.vel.y = -m.vel.y;
        }
      }

      for (src, a) in actions {
        let src_actor = actors.get_mut(&src).unwrap();
        a.apply(src_actor);
      }
    }

    for a in actors.values() {
      let r = a.movable.bounds();
      draw_rectangle(r.x, r.y, r.w, r.h, a.color);
    }

    #[cfg(debug_assertions)]
    macroquad_profiler::profiler(Default::default());

    draw_text(format!("{}", get_fps()).as_str(), 5., 20., 32., WHITE);
    draw_text(
      format!("{:?}", mouse_position()).as_str(),
      5.,
      screen_height() - 20.,
      32.,
      WHITE
    );

    next_frame().await
  }
}
