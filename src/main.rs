use core::cell::RefCell;
use crate::components::actor::Actor;
use components::actor::{resolve_collision};
use macroquad::prelude::*;
use std::{collections::HashMap};
use macroquad::telemetry;

mod utils;
mod collision_detection;
mod components;
// mod timer;
// mod animation;

use crate::components::movable::Movable;

use utils::*;
use collision_detection::cd_system::{CDSystem, get_collisions, line_line_collision};


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
  // let actors: HashMap<usize, RefCell<Actor>> = generate_player_and_enemies(5);
  let actors: HashMap<usize, RefCell<Actor>> = HashMap::new();
  let mut cdsystem = CDSystem::new();


  let la = (10., 10., 800., 800.);
  let lb = (10., 700., 800., 50.);

  loop {
    let delta_t = get_frame_time();
    clear_background(BLACK);

    let i = line_line_collision(
      la.0, la.1, la.2, la.3,
      lb.0, lb.1, lb.2, lb.3
    );
    draw_line(la.0, la.1, la.2, la.3, 2., GREEN);
    draw_line(lb.0, lb.1, lb.2, lb.3, 2., BLUE);
    if let Some(i) = i {
      draw_circle(i.x, i.y, 5., RED);
    }
    draw_text(format!("{:?}", i).as_str(), 5., 20., 32., WHITE);

    if actors.is_empty() {
      next_frame().await;
      continue;
    }

    {
      let actors_bounds = &actors.iter().map(|(k, a)| (*k, a.borrow().movable.bounds())).collect();
      cdsystem.update(get_collisions(actors_bounds));

      for (ka, kb) in cdsystem.get_collided() {
        let aa = &mut actors.get(&ka).unwrap().borrow_mut();
        let ab = &mut actors.get(&kb).unwrap().borrow_mut();

        resolve_collision(aa, ab, delta_t);
      }
    }

    {
      #[cfg(debug_assertions)]
      let _z = telemetry::ZoneGuard::new("update");

      for (_k, a) in actors.iter() {
        let mut a = a.borrow_mut();
        if let Some(next_state) = a.decide(&actors) {
          a.set_state(next_state);
        }
        a.update(delta_t);
        let m = &mut a.movable;

        if m.bounds().right() > screen_width() ||
            m.bounds().left() < 0. ||
            m.bounds().bottom() > screen_height() ||
            m.bounds().top() < 0.
        {
          m.set_vel(Vec2::ZERO);
        }
      }
    }

    for a in actors.values() {
      let r = &a.borrow().movable.bounds();
      draw_rectangle(r.x, r.y, r.w, r.h, a.borrow().color);
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
