use macroquad::prelude::*;

use crate::collision_detection::cd_system::line_line_collision;


#[derive(Debug)]
pub struct Corridor {
  start_a: Vec2,
  end_a: Vec2,
  start_b: Vec2,
  end_b: Vec2,
}

impl Corridor {
  pub fn new(start_a: Vec2, end_a: Vec2, start_b: Vec2, end_b: Vec2) -> Self {
    Self { start_a, end_a, start_b, end_b }
  }

  pub fn draw(&self) {
    draw_line(self.start_a.x, self.start_a.y, self.end_a.x, self.end_a.y, 4., PURPLE);
    draw_line(self.start_b.x, self.start_b.y, self.end_b.x, self.end_b.y, 4., PINK);
  }
}

pub fn build_corridors(cb: Vec<CorridorBuilder>) -> Vec<Corridor> {
  let mut corridors: Vec<Corridor> = vec![];

  match cb.len() {
    0 => {},
    1 => {
      corridors.push(cb[0].corridor_from_points(None));
    },
    l => {
      corridors.push(cb[0].corridor_from_points(None));
      for i in 1..l {
        let new_corridor = cb[i].corridor_from_points(corridors.last());
        corridors[i-1].end_a = new_corridor.start_a;
        corridors[i-1].end_b = new_corridor.start_b;
        corridors.push(new_corridor);
      }
    }
  };

  corridors
}

pub struct CorridorBuilder {
  start: Vec2,
  start_width: f32,
  end: Vec2,
  end_width: f32,
}

impl CorridorBuilder {
  pub fn new(start: Vec2, end: Vec2, start_width: f32, end_width: f32) -> Self {
    Self {
      start,
      start_width,
      end,
      end_width
    }
  }

  pub fn corridor_from_points(&self, previous: Option<&Corridor>) -> Corridor {
    let v = (self.end - self.start).perp().normalize();
    let wall_start_a = self.start + v * self.start_width / 2.;
    let wall_end_a = self.end + v * self.end_width / 2.;

    let wall_start_b = self.start - v * self.start_width / 2.;
    let wall_end_b = self.end - v * self.end_width / 2.;

    if let Some(prev) = previous {
      let intersection_a = line_line_collision(
        wall_end_a.x, wall_end_a.y, wall_start_a.x, wall_start_a.y,
        prev.start_a.x, prev.start_a.y, prev.end_a.x, prev.end_a.y
      );
      let intersection_b = line_line_collision(
        wall_end_b.x, wall_end_b.y, wall_start_b.x, wall_start_b.y,
        prev.start_b.x, prev.start_b.y, prev.end_b.x, prev.end_b.y
      );
      let i_a = intersection_a.expect("intersection A with previous wall is found");
      let i_b = intersection_b.expect("intersection B with previous wall is found");
      Corridor::new(i_a, wall_end_a, i_b, wall_end_b)
    } else {
      Corridor::new(wall_start_a, wall_end_a, wall_start_b, wall_end_b)
    }
  }

  pub fn draw(&self) {
    draw_circle(self.start.x, self.start.y, 5., RED);
    draw_circle(self.end.x, self.end.y, 5., RED);

    let v = (self.end - self.start).perp().normalize();

    let wall_start_a = self.start + v * self.start_width / 2.;
    let wall_end_a = self.end + v * self.end_width / 2.;

    let wall_start_b = self.start - v * self.start_width / 2.;
    let wall_end_b = self.end - v * self.end_width / 2.;

    draw_line(self.start.x, self.start.y, self.end.x, self.end.y, 2., YELLOW);

    draw_line(wall_start_a.x, wall_start_a.y, wall_end_a.x, wall_end_a.y, 4., GREEN);
    draw_line(wall_start_b.x, wall_start_b.y, wall_end_b.x, wall_end_b.y, 4., BLUE);
  }
}