use std::collections::HashSet;

use macroquad::{prelude::*};

use super::cd_system::CDData;

const MAX_DEPTH: usize = 16;

type BTElem<'a> = (usize, &'a Rect);

#[derive(Clone)]
pub enum BTreeSplit {
  Horizontal,
  Vertical,
}

pub struct BTree<'a> {
  depth: usize,
  bounds: Rect,
  split: BTreeSplit,
  elems: Vec<BTElem<'a>>,
  children: Option<(Box<BTree<'a>>, Box<BTree<'a>>)>,
  treshold: usize,
}

impl<'a> BTree<'a> {
  pub fn root(bounds: Rect, treshold: usize) -> Self {
    let split = if bounds.w > bounds.h {
      BTreeSplit::Vertical
    } else {
      BTreeSplit::Horizontal
    };
    BTree::new(0, bounds, treshold, split)
  }

  pub fn new(
    depth: usize,
    bounds: Rect,
    treshold: usize,
    split: BTreeSplit
  ) -> Self {
    BTree { depth, bounds, split, treshold, elems: vec![], children: None }
  }

  pub fn split(&self) -> (Rect, Rect, BTreeSplit) {
    match self.split {
      BTreeSplit::Horizontal => {
        let ra = Rect::new(
          self.bounds.x,
          self.bounds.y,
          self.bounds.w,
          self.bounds.h/2.
        );
        let rb = Rect::new(
          self.bounds.x,
          self.bounds.y + self.bounds.h/2.,
          self.bounds.w,
          self.bounds.h/2.
        );
        (ra, rb, BTreeSplit::Vertical)
      },
      BTreeSplit::Vertical => {
        let ra = Rect::new(
          self.bounds.x,
          self.bounds.y,
          self.bounds.w/2.,
          self.bounds.h
        );
        let rb = Rect::new(
          self.bounds.x + self.bounds.w/2.,
          self.bounds.y,
          self.bounds.w/2.,
          self.bounds.h
        );
        (ra, rb, BTreeSplit::Horizontal)
      }
    }
  }

  pub fn insert(&mut self, value: BTElem<'a>) {
    if !self.bounds.overlaps(value.1) {
      return;
    }

    match self.children {
      Some(ref mut ch) => {
        ch.0.insert(value);
        ch.1.insert(value);
      },
      None => {
        if self.elems.len() + 1 > self.treshold && self.depth < MAX_DEPTH {
          let (ra, rb, split) = self.split();
          let mut bta = BTree::new(
            self.depth + 1,
            ra,
            self.treshold,
            split.clone()
          );
          let mut btb = BTree::new(
            self.depth + 1,
            rb,
            self.treshold,
            split
          );

          for elem in self.elems.clone() {
            bta.insert(elem);
            btb.insert(elem);
          }
          self.elems.clear();

          bta.insert(value);
          btb.insert(value);
          self.children = Some((Box::new(bta), Box::new(btb)));
        } else {
          self.elems.push(value);
        }
      },
    }
  }

  pub fn get_collisions(&self) -> HashSet<CDData> {
    match &self.children {
      Some(ch) => {
        &ch.0.get_collisions() | &ch.1.get_collisions()
      },
      None => {
        let mut collisions = HashSet::new();
        for index_a in 0..self.elems.len() {
          for index_b in (index_a+1)..self.elems.len() {
            if self.elems[index_a].1.overlaps(self.elems[index_b].1) {
              collisions.insert((self.elems[index_a].0, self.elems[index_b].0));
            }
          }
        }
        collisions
      }
    }
  }

  #[cfg(debug_assertions)]
  pub fn draw(&self, thickness: f32) {
    draw_rectangle_lines(
      self.bounds.x,
      self.bounds.y,
      self.bounds.w,
      self.bounds.h,
      thickness,
      YELLOW
    );
    match &self.children {
      Some(ch) => {
        ch.0.draw(thickness + 0.5);
        ch.1.draw(thickness + 0.5);
      },
      None => {
        draw_text(
          format!("{}", self.elems.len()).as_str(),
          self.bounds.x + 2. ,
          self.bounds.y + 10.,
          16.,
          WHITE
        );
      }
    }
  }
}



#[cfg(test)]
mod tests {
  use crate::components::movable::Movable;
  use super::*;
  const W: f32 = 64.;
  const H: f32 = 32.;
  const SIZE: f32 = 8.;

  fn create<'a>() -> BTree<'a> {
    BTree::new(
      0,
      Rect::new(0., 0., W, H),
      2,
      BTreeSplit::Vertical
    )
  }

  fn create_rect(pos: Vec2) -> Rect {
    *Movable::new().with_pos(pos).with_size((SIZE, SIZE)).bounds()
  }

  #[test]
  fn insert() {
    let mut bt = create();
    let r = &create_rect(vec2(4., 4.));
    bt.insert((1, r));

    assert_eq!(bt.elems.len(), 1);
    assert_eq!(bt.elems[0].0, 1);
    assert!(bt.children.is_none());
  }

  mod insert_over_treshold {
    use super::*;

    #[test]
    fn simple() {
      let mut bt = create();
      let r = &create_rect(vec2(4., 4.));
      bt.insert((1, r));

      let r = &create_rect(vec2(16., 4.));
      bt.insert((1, r));

      let r = &create_rect(vec2(40., 4.));
      bt.insert((1, r));

      assert_eq!(bt.elems.len(), 0);
      assert!(bt.children.is_some());
      let children = bt.children.unwrap();

      assert_eq!(children.0.bounds.x, 0.);
      assert_eq!(children.0.bounds.y, 0.);
      assert_eq!(children.0.bounds.w, 32.);
      assert_eq!(children.0.bounds.h, 32.);

      assert_eq!(children.1.bounds.x, 32.);
      assert_eq!(children.1.bounds.y, 0.);
      assert_eq!(children.1.bounds.w, 32.);
      assert_eq!(children.1.bounds.h, 32.);

      assert_eq!(children.0.elems.len(), 2);
      assert_eq!(children.1.elems.len(), 1);
    }

    #[test]
    fn one_in_two_trees() {
      let mut bt = create();
      let r = &create_rect(vec2(4., 4.));
      bt.insert((1, r));

      let r = &create_rect(vec2(56., 4.));
      bt.insert((2, r));

      let r = &create_rect(vec2(32., 24.));
      bt.insert((3, r));

      assert_eq!(bt.elems.len(), 0);
      assert!(bt.children.is_some());
      let children = bt.children.unwrap();

      assert_eq!(children.0.elems.len(), 2);
      assert_eq!(children.1.elems.len(), 2);
    }

    #[test]
    fn max_depth() {
      let mut bt = create();
      let r = &create_rect(vec2(0., 0.));
      bt.insert((1, r));

      let r = &create_rect(vec2(0., 0.));
      bt.insert((2, r));

      let r = &create_rect(vec2(0., 0.));
      bt.insert((3, r));

      assert_eq!(bt.elems.len(), 0);
      assert!(bt.children.is_some());

      for _ in 0..MAX_DEPTH {
        let children = bt.children.unwrap();
        bt = *children.0;
      }

      assert!(bt.children.is_none());
      assert_eq!(bt.elems.len(), 3);
    }
  }
}
