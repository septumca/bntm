use macroquad::prelude::*;


#[derive(Debug, Clone)]
pub struct Timer {
  act: f32,
  threshold: f32,
  just_over: bool,
  repeat: bool,
}

impl Timer {
  pub fn new(threshold: f32) -> Self {
    Self { act: 0., threshold, repeat: true, just_over: false }
  }

  pub fn new_timeout(threshold: f32) -> Self {
    Self { act: 0., threshold, repeat: false, just_over: false }
  }

  pub fn reset(&mut self) {
    self.act = 0.;
    self.just_over = false;
  }

  pub fn set_treshold(&mut self, threshold: f32) {
    self.threshold = threshold;
  }

  pub fn is_over(&self) -> bool {
    self.act > self.threshold
  }

  pub fn is_just_over(&self) -> bool {
    self.just_over
  }

  pub fn update(&mut self, delta_t: f32) {
    if self.is_over() && !self.repeat {
      return;
    }
    let updated_time = self.act + delta_t;
    let over_threshold = updated_time > self.threshold;

    if self.just_over && !over_threshold {
      self.just_over = false;
    } else if over_threshold && !self.just_over {
      self.just_over = true;
    }
    self.act = if over_threshold && self.repeat { 0. } else { updated_time };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create() -> Timer {
    Timer {
      act: 0.,
      threshold: 0.5,
      repeat: true,
      just_over: false,
    }
  }

  #[test]
  fn update() {
    let mut time = create();
    time.update(0.1);
    assert_eq!(time.act, 0.1);
    time.update(0.1);
    assert_eq!(time.act, 0.2);
    time.update(0.1);
    assert_eq!(time.act, 0.3);
  }

  #[test]
  fn update_over() {
    let mut time = create();
    time.update(0.4);
    assert_eq!(time.act, 0.4);
    time.update(0.1);
    assert_eq!(time.act, 0.5);
    time.update(0.1);
    assert_eq!(time.act, 0.);
  }

  #[test]
  fn update_over_not_repeat() {
    let mut time = create();
    time.repeat = false;
    time.update(0.4);
    assert_eq!(time.act, 0.4);
    time.update(0.1);
    assert_eq!(time.act, 0.5);
    time.update(0.1);
    assert_eq!(time.act, 0.6);
    assert_eq!(time.is_just_over(), true);
    time.update(0.2);
    assert_eq!(time.act, 0.6);
    assert_eq!(time.is_just_over(), true);
  }

  #[test]
  fn update_just_over() {
    let mut time = create();
    time.repeat = true;
    time.update(0.4);
    assert_eq!(time.act, 0.4);
    time.update(0.1);
    assert_eq!(time.act, 0.5);
    time.update(0.1);
    assert_eq!(time.act, 0.);
    assert_eq!(time.is_just_over(), true);
    time.update(0.2);
    assert_eq!(time.act, 0.2);
    assert_eq!(time.is_just_over(), false);
    time.update(0.4);
    assert_eq!(time.act, 0.);
    assert_eq!(time.is_just_over(), true);
    time.update(0.1);
    assert_eq!(time.act, 0.1);
    assert_eq!(time.is_just_over(), false);
    time.update(0.1);
    assert_eq!(time.act, 0.2);
    assert_eq!(time.is_just_over(), false);
  }
}
