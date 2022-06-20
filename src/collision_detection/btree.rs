use std::collections::HashSet;

use macroquad::{prelude::*};

use crate::components::movable::Movable;

use super::cd_system::CDData;


type BTElem<'a> = (usize, &'a Movable);

#[derive(Clone)]
pub enum BTreeSplit {
  Horizontal,
  Vertical,
}

pub struct BTree<'a> {
  bounds: Rect,
  split: BTreeSplit,
  elems: Vec<BTElem<'a>>,
  children: Option<(Box<BTree<'a>>, Box<BTree<'a>>)>,
  treshold: usize,
}

impl<'a> BTree<'a> {
  pub fn new(bounds: Rect, treshold: usize, split: BTreeSplit) -> Self {
    BTree { bounds, split, treshold, elems: vec![], children: None }
  }

  pub fn split(&self) -> (Rect, Rect, BTreeSplit) {
    match self.split {
      BTreeSplit::Horizontal => {
        let ra = Rect::new(self.bounds.x, self.bounds.y, self.bounds.w, self.bounds.h/2.);
        let rb = Rect::new(self.bounds.x, self.bounds.y + self.bounds.h/2., self.bounds.w, self.bounds.h/2.);
        (ra, rb, BTreeSplit::Vertical)
      },
      BTreeSplit::Vertical => {
        let ra = Rect::new(self.bounds.x, self.bounds.y, self.bounds.w/2., self.bounds.h);
        let rb = Rect::new(self.bounds.x + self.bounds.w/2., self.bounds.y, self.bounds.w/2., self.bounds.h);
        (ra, rb, BTreeSplit::Horizontal)
      }
    }
  }

  pub fn insert(&mut self, value: BTElem<'a>) {
    if !self.bounds.overlaps(&value.1.bounds()) {
      return;
    }

    match &mut self.children {
      &mut Some(ref mut ch) => {
        ch.0.insert(value);
        ch.1.insert(value);
      },
      &mut None => {
        if self.elems.len() + 1 > self.treshold {
          let (ra, rb, split) = self.split();
          let mut bta = BTree::new(ra, self.treshold, split.clone());
          let mut btb = BTree::new(rb, self.treshold, split.clone());

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
            if self.elems[index_a].1.bounds().overlaps(&self.elems[index_b].1.bounds()) {
              collisions.insert((self.elems[index_a].0, self.elems[index_b].0));
            }
          }
        }
        collisions
      }
    }
  }

  pub fn draw(&self, thickness: f32) {
    draw_rectangle_lines(self.bounds.x, self.bounds.y, self.bounds.w, self.bounds.h, thickness, YELLOW);
    match &self.children {
      Some(ch) => {
        ch.0.draw(thickness + 0.5);
        ch.1.draw(thickness + 0.5);
      },
      None => {
        draw_text(format!("{}", self.elems.len()).as_str(), self.bounds.x + 2. , self.bounds.y + 10., 16., WHITE);
      }
    }
  }
}
