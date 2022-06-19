use macroquad::{prelude::*};
use std::{collections::HashMap};
use macroquad::telemetry;

mod utils;

use utils::*;

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
  let elements = 2048;
  let treshold = 512;
  let mut movables: HashMap<usize, Actor> = HashMap::new();
  let mut cdsystem = CDSystem::new();
  let split = if screen_width() > screen_height() { BTreeSplit::Vertical } else { BTreeSplit::Horizontal };

  loop {
    let delta_t = get_frame_time();
    if is_key_released(KeyCode::A) {
      movables = generate_random(elements);
    }

    if is_key_released(KeyCode::S) {
      movables = generate_two_opposite();
    }

    if is_key_released(KeyCode::D) {
      movables = generate_two_inside();
    }

    clear_background(DARKGRAY);

    if movables.len() == 0 {
      next_frame().await;
      continue;
    }

    let mut bt = BTree::new(Rect::new(0., 0., screen_width(), screen_height()), treshold, split.clone());
    {
      #[cfg(debug_assertions)]
      let _z = telemetry::ZoneGuard::new("bt");

      {
        let _z = telemetry::ZoneGuard::new("BTree - insert");
        for (k, (m, _)) in &movables {
          bt.insert((*k, m.bounds()));
        }
      }

      cdsystem.update(bt.get_collisions());
    }

    {
      #[cfg(debug_assertions)]
      let _z = telemetry::ZoneGuard::new("collision detection and resoultion");

      for (ka, kb) in cdsystem.get_just_collided() {
        let (ma, _c) = movables.get(&ka).unwrap();
        let (mb, _c) = movables.get(&kb).unwrap();

        let ca = get_collision_axis(&ma, &mb, delta_t);

        let (vela, velb) = match ca {
          CollisionAxis::X => {
            (vec2(-ma.vel.x, ma.vel.y), vec2(-mb.vel.x, mb.vel.y))
          },
          CollisionAxis::Y => {
            (vec2(ma.vel.x, -ma.vel.y), vec2(mb.vel.x, -mb.vel.y))
          },
          CollisionAxis::Both => {
            (vec2(-ma.vel.x, -ma.vel.y), vec2(-mb.vel.x, -mb.vel.y))
          },
        };
        movables.get_mut(&ka).unwrap().0.set_vel(vela);
        movables.get_mut(&kb).unwrap().0.set_vel(velb);
      }
    }

    {
      #[cfg(debug_assertions)]
      let _z = telemetry::ZoneGuard::new("update");

      for (m, _) in movables.values_mut() {
        m.update(delta_t);

        if m.bounds().right() > screen_width() || m.bounds().left() < 0. {
          m.vel.x = -m.vel.x;
        }
        if m.bounds().bottom() > screen_height() || m.bounds().top() < 0. {
          m.vel.y = -m.vel.y;
        }
      }
    }

    for (m, c) in movables.values() {
      let r = m.bounds();
      draw_rectangle(r.x, r.y, r.w, r.h, *c);
    }

    #[cfg(debug_assertions)]
    {
      macroquad_profiler::profiler(Default::default());
      bt.draw(1.);
    }

    draw_text(format!("{}", get_fps()).as_str(), 5., 20., 32., WHITE);
    draw_text(format!("{:?}", mouse_position()).as_str(), 5., screen_height() - 20., 32., WHITE);

    next_frame().await
  }
}
