use crate::components::{actor::{Actor,ActorKind}, movable::Movable, ai::Ai};
use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;


pub const EPSILON: f32 = 0.004;


pub fn generate_random(count: usize) -> HashMap<usize, Actor> {
  let mut actors: HashMap<usize, Actor>  = HashMap::new();

  for k in 0..count {
    let m = Movable::new().with_size((4., 4.));
    let x = rand::gen_range::<f32>(
      0. + m.bounds().w / 2.,
      screen_width() - m.bounds().h / 2.
    );
    let y = rand::gen_range::<f32>(
      0. + m.bounds().w / 2.,
      screen_height() - m.bounds().h / 2.
    );

    let vx = rand::gen_range::<f32>(-1., 1.);
    let vy = rand::gen_range::<f32>(-1., 1.);

    let colors = vec![RED, GREEN, BLUE];
    let c = colors.choose().unwrap_or(&RED);
    // let c = Color::new(
    //   rand::gen_range::<f32>(0., 1.),
    //   rand::gen_range::<f32>(0., 1.),
    //   rand::gen_range::<f32>(0., 1.),
    //   1.
    // );

    let m = m.with_pos(vec2(x, y)).with_vel(vec2(vx, vy));

    actors.insert(k, Actor::new(m, *c, ActorKind::Enemy));
  }
  actors
}

pub fn generate_two_opposite() -> HashMap<usize, Actor> {
  let mut actors: HashMap<usize, Actor>  = HashMap::new();

  let ma = Movable::new().with_size((32., 32.));
  let mb = Movable::new().with_size((32., 32.));

  let posa = vec2(screen_width() / 4., screen_height() / 2. - 200.);
  let vela = Vec2::ONE * ma.speed;
  let posb = vec2(screen_width() / 4. * 3., screen_height() / 2. - 200.);
  let mut velb = Vec2::ONE * ma.speed;
  velb.x = -velb.x;
  let ca = RED;
  let cb = BLUE;

  actors.insert(0,
    Actor::new(
      ma.with_pos(posa).with_vel(vela).with_speed(300.),
      ca,
      ActorKind::Enemy
    )
  );
  actors.insert(1,
    Actor::new(
      mb.with_pos(posb).with_vel(velb).with_speed(300.),
      cb,
      ActorKind::Enemy
    )
  );

  actors
}

pub fn generate_two_inside() -> HashMap<usize, Actor> {
  let mut actors: HashMap<usize, Actor>  = HashMap::new();

  let ma = Movable::new().with_size((32., 32.));
  let mb = Movable::new().with_size((32., 32.));

  let posa = vec2(
    screen_width() / 2. - ma.bounds().w / 4.,
    screen_height() / 2.
  );
  let vela = vec2(1., 0.);
  let posb = vec2(
    screen_width() / 2. +  mb.bounds().w / 4.,
    screen_height() / 2.
  );
  let velb = vec2(-1., 0.);
  let ca = RED;
  let cb = BLUE;

  actors.insert(
    0,
    Actor::new(
      ma.with_pos(posa).with_vel(vela),
      ca,
      ActorKind::Enemy
    )
  );
  actors.insert(1,
    Actor::new(
      mb.with_pos(posb).with_vel(velb),
      cb,
      ActorKind::Enemy
    )
  );

  actors
}

pub fn generate_player_and_enemies(enemy_count: usize) -> HashMap<usize, Actor> {
  let mut actors: HashMap<usize, Actor>  = HashMap::new();

  let pm = Movable::new().with_size((32., 32.)).with_pos(vec2(screen_width() / 2., screen_height() / 2.));
  actors.insert(0, Actor::new(pm, RED, ActorKind::Player));

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

    let colors = vec![GREEN, BLUE, PURPLE];
    let c = colors.choose().unwrap_or(&GREEN);

    actors.insert(
      k+1,
      Actor::new(
        m.with_pos(vec2(x, y)),
        *c,
        ActorKind::Enemy
      ).with_ai(Ai {})
    );
  }

  actors
}

#[inline(always)]
pub fn rect_from_pos(pos: Vec2, size: (f32, f32)) -> Rect {
  Rect::new(pos.x - size.0 / 2., pos.y - size.0 / 2., size.0, size.1)
}
