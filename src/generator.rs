use rand::{SeedableRng, RngCore};
use rand::rngs::{SmallRng};
use serde::{Deserialize, Serialize};

use crate::rope::*;

#[derive(Default, Serialize)]
pub struct GeneratedStructure {
  floor_pinned_nodes : Vec<usize>,
  nodes : Vec<usize>,
  ropes : Vec<usize>,
}

impl GeneratedStructure {
  //pub fn from_intermediate(int: &IntermediateStructure) -> Self {
    
  //}

  pub fn from_blueprint(blue : &Blueprint, world : &mut World, offset : Vec2, scale : f32, tangent: Vec2, normal : Vec2) -> Self {
    let mut realised_node_ids : Vec<usize> = vec![];
    let mut floor_pinned_nodes : Vec<usize> = vec![];
    let mut nodes : Vec<usize> = vec![];

    for (node_pos, is_floor_pinned) in &blue.nodes {
      //let transformed = offset.add(node_pos.mult(scale));

      let mut p = offset;
      p = p.add(tangent.mult(node_pos.x * scale));
      p = p.add(normal.mult(node_pos.y * scale));

      let id = world.add_node(p.x, p.y);
      realised_node_ids.push(id);
      if (*is_floor_pinned) {
        floor_pinned_nodes.push(id);
      }
      else {
        nodes.push(id);
      }
    }

    let mut ropes : Vec<usize> = vec![];

    for (from, to) in &blue.ropes {
      let rope_id = world.add_rope(realised_node_ids[*from], realised_node_ids[*to]);
      ropes.push(rope_id);
    }

    Self {
      floor_pinned_nodes,
      nodes,
      ropes,
    }
  }
}

#[derive(Default)]
pub struct Blueprint {
  nodes : Vec<(Vec2, bool)>,
  ropes : Vec<(usize, usize)>,
}

pub struct Generator {
  rng : SmallRng,
}

impl Generator {
  pub fn new(input_seed : u8) -> Self {
    let mut seed : <SmallRng as SeedableRng>::Seed = Default::default();
    seed[0] = input_seed;

    Self {
      rng : SmallRng::from_seed(seed),
    }
  }

  pub fn gen(&mut self) -> Blueprint {
    //let height = 1 + self.rng.next_u32() % 5;
    let height = 5;

    //let mut int = IntermediateStructure::default();
    //for i in 0..=height {
    //  int.cells.push(Cell {
    //    offset: Vec2::new(-0.5, -(i as f32)),
    //    cell: CellState::Scaffolding,
    //  });
    //}
    //GeneratedStructure::from_intermediate(&int);

    let mut blue = Blueprint::default();
    blue.nodes.push((Vec2::new(-0.5, 0.), true));
    blue.nodes.push((Vec2::new(0.5, 0.), true));

    blue.nodes.push((Vec2::new(-0.5, 1.), false));
    blue.nodes.push((Vec2::new(0.5, 1.), false));

    blue.ropes.push((0, 1));
    blue.ropes.push((0, 2));
    blue.ropes.push((0, 3));

    blue.ropes.push((1, 2));
    blue.ropes.push((1, 3));

    blue.ropes.push((2, 3));

    for i in 0..height {
      let left_node_id = blue.nodes.len();
      blue.nodes.push((Vec2::new(-0.5, 2.0 + i as f32), false));
      let right_node_id = blue.nodes.len();
      blue.nodes.push((Vec2::new(0.5, 2.0 + i as f32), false));
      blue.ropes.push((left_node_id, right_node_id));
      let below_left_node_id = left_node_id - 2;
      let below_right_node_id = left_node_id - 1;

      blue.ropes.push((left_node_id, below_left_node_id));
      blue.ropes.push((left_node_id, below_right_node_id));
      blue.ropes.push((right_node_id, below_right_node_id));
      blue.ropes.push((right_node_id, below_left_node_id));
    }

    blue
  }
}

struct Cell {
  offset : Vec2,
  cell : CellState,
}

enum CellState {
  Scaffolding,
  Empty,
} 

#[derive(Default)]
struct IntermediateStructure {
  cells : Vec<Cell>,
}
