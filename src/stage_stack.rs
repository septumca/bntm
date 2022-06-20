use super::{resources::{Resources}, playing::PlayingStage, game_over::GameOver};

#[derive(Debug, Clone)]
pub enum StageAction {
  GameQuit,
  StartGame,
  GameOver(usize),
  EndGame,
}

pub trait Stage {
  fn update(&mut self, resources: &Resources) -> Option<StageAction>;
  fn draw(&self, resources: &Resources);
}

pub struct StageStack {
  pub stack: Vec<Box<dyn Stage>>,
}


impl StageStack {
  pub fn new() -> Self {
    Self {
      stack: vec![],
    }
  }

  pub fn with_stages(mut self, stack: Vec<Box<dyn Stage>>) -> Self {
    self.stack = stack;

    self
  }

  pub fn is_empty(&self) -> bool {
    return self.stack.is_empty()
  }

  pub fn update(&mut self, resources: &Resources) {
    let mut action: Option<StageAction> = None;
    if let Some(stage) = self.stack.last_mut() {
      action = stage.update(resources);
    }

    match action {
      Some(StageAction::GameOver(s)) => {
        self.stack.pop();
        self.stack.push(Box::new(GameOver::new(s)));
      },
      Some(StageAction::GameQuit) => {
        self.stack.clear();
      },
      Some(StageAction::StartGame) => {
        self.stack.push(Box::new(PlayingStage::new()));
      }
      Some(StageAction::EndGame) => {
        self.stack.pop();
      },
      None => ()
    };
  }

  pub fn draw(&mut self, resources: &Resources) {
    if let Some(stage) = self.stack.last_mut() {
      stage.draw(resources);
    }
  }
}