use std::collections::HashSet;

use rand::{SeedableRng, RngCore};
use rand::rngs::{SmallRng};
use serde::{Deserialize, Serialize};

use crate::rope::*;
use crate::dense_grid::{I2, DenseGrid};

#[derive(Default, Serialize)]
pub struct GeneratedStructure {
  floor_pinned_nodes : Vec<usize>,
  nodes : Vec<usize>,
  ropes : Vec<usize>,
  ropes_nodraw : Vec<usize>,
}

impl GeneratedStructure {
  //pub fn from_intermediate(int: &IntermediateStructure) -> Self {
    
  //}

  pub fn from_blueprint(blue : &Blueprint, world : &mut World, point_on_world : Vec2, world_centre : Vec2, scale : f32) -> Self {
    let mut realised_node_ids : Vec<usize> = vec![];
    let mut floor_pinned_nodes : Vec<usize> = vec![];
    let mut nodes : Vec<usize> = vec![];

   
    let normal = point_on_world.sub(world_centre).norm();
    let tangent = Vec2::new(-normal.y, normal.x);
    

    let world_radius = point_on_world.dist(world_centre);
    let theta = scale.atan2(world_radius);
    let angle_base = normal.y.atan2(normal.x);

    for (node_pos, is_floor_pinned) in &blue.nodes {
      //let transformed = offset.add(node_pos.mult(scale));

      /*
      // Linear + offset transformation
      let offset = point_on_world;
      let mut p = offset;
      p = p.add(tangent.mult(node_pos.x * scale));
      p = p.add(normal.mult(node_pos.y * scale));
      */

      /*
      // Curved transformation
      //let polar_angle = node_pos.x * theta;
      //let polar_r = world_radius + node_pos.y * scale;

      //let p = world_centre.add(Vec2::new(
        //polar_r * polar_angle.cos(),
        //polar_r * polar_angle.sin()));
      */
      
      // Hybrid
      let base_point_polar_angle = angle_base + (node_pos.x * theta) / (1.0 + 0.0125 * node_pos.y);
      let base_point = world_centre.add(Vec2::new(
        world_radius * base_point_polar_angle.cos(),
        world_radius * base_point_polar_angle.sin()));

      let p = base_point.add(normal.mult(node_pos.y * scale));


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
    let mut ropes_nodraw : Vec<usize> = vec![];

    for (from, to, visible) in &blue.ropes {
      let rope_id = world.add_rope(realised_node_ids[*from], realised_node_ids[*to]);
      if (*visible) {
        ropes.push(rope_id);
      }
      else {
        ropes_nodraw.push(rope_id)
      }
    }

    Self {
      floor_pinned_nodes,
      nodes,
      ropes,
      ropes_nodraw,
    }
  }
}

#[derive(Default)]
pub struct Blueprint {
  nodes : Vec<(Vec2, bool)>,
  ropes : Vec<(usize, usize, bool)>,

  node_grid : DenseGrid::<Option<usize>>,
  rope_hashset : HashSet::<(usize, usize)>,
}

impl Blueprint {
  fn add_off_grid(&mut self, pos : Vec2) -> usize {
    let id = self.nodes.len();
    self.nodes.push((pos, pos.y == 0.0));
    id
  }

  fn try_add_node(&mut self, pos : I2) {
    let grid_id = self.node_grid.get_mut(pos);
    if (grid_id.is_none()) {
      let grounded = pos.y == 0;
      let id = self.nodes.len();
      self.nodes.push((pos.to_v2(), grounded));
      *grid_id = Some(id);
    }
  }

  fn try_add_rope(&mut self, from : I2, to : I2, visible: bool) {
    if let Some(from_node_id) = self.node_grid.get(from) {
      if let Some(to_node_id) = self.node_grid.get(to) {
        self.ropes.push((from_node_id, to_node_id, visible));
      }
    }
  }

  fn try_add_rope_pos_id(&mut self, from : I2, to_node_id : usize, visible : bool) {
    if let Some(from_node_id) = self.node_grid.get(from) {
      self.ropes.push((from_node_id, to_node_id, visible));
    }
  }
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

  pub fn gen_intermediate(&mut self) -> IntermediateStructure {
    let mut int = IntermediateStructure::default(); 

    for i in 0..4 {
      int.set(I2::new(0, i), CellState::Scaffolding);
    }

    for i in 0..2 {
      int.set(I2::new(1, i), CellState::Scaffolding);
    }

    for i in 0..4 {
      int.set(I2::new(2, i), CellState::Scaffolding);
    }

    int.set(I2::new(0, 4), CellState::Roof);
    int.set(I2::new(2, 4), CellState::Roof);

    int
  }

  pub fn gen(&mut self) -> Blueprint {
    self.gen_intermediate().to_blueprint()
  }

  pub fn gen_basic(&mut self) -> Blueprint {
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

    blue.ropes.push((0, 1, true));
    blue.ropes.push((0, 2, true));
    blue.ropes.push((0, 3, true));

    blue.ropes.push((1, 2, true));
    blue.ropes.push((1, 3, true));

    blue.ropes.push((2, 3, true));

    for i in 0..height {
      let left_node_id = blue.nodes.len();
      blue.nodes.push((Vec2::new(-0.5, 2.0 + i as f32), false));
      let right_node_id = blue.nodes.len();
      blue.nodes.push((Vec2::new(0.5, 2.0 + i as f32), false));
      blue.ropes.push((left_node_id, right_node_id, true));
      let below_left_node_id = left_node_id - 2;
      let below_right_node_id = left_node_id - 1;

      blue.ropes.push((left_node_id, below_left_node_id, true));
      blue.ropes.push((left_node_id, below_right_node_id, true));
      blue.ropes.push((right_node_id, below_right_node_id, true));
      blue.ropes.push((right_node_id, below_left_node_id, true));
    }

    blue
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CellState {
  Empty,
  Roof,
  Scaffolding,
} 

impl Default for CellState {
  fn default() -> Self {
    Self::Empty
  }
}

#[derive(Default)]
struct IntermediateStructure {
  grid : DenseGrid<CellState>,
}

impl IntermediateStructure {
  pub fn is_solid(&mut self, pos : I2) -> bool {
    *self.grid.get_mut(pos) != CellState::Empty
  }

  pub fn set(&mut self, pos : I2, val : CellState) {
    *self.grid.get_mut(pos) = val
  }

  pub fn to_blueprint(&mut self) -> Blueprint {
    let mut blue = Blueprint::default();

    // Fix w now to make sure mutating the grid doesnt change this
    let w = self.grid.max_x();
    let h = self.grid.max_y();

    // Scan across and up
    let mut p_ground = I2::new(-w, 0);

    while (p_ground.x <= w) {
      let mut p = p_ground;

      while (p.y < h) {

        match self.grid.get(p) {
          CellState::Scaffolding => {
              let corners = [
                I2::new(0, 0),
                I2::new(1, 0),
                I2::new(0, 1),
                I2::new(1, 1),
              ];

              for offset in &corners {
                blue.try_add_node(p + *offset);
              }

              let top_left = p + I2::new(0, 1);
              let top_right = p + I2::new(1, 1);
              let bottom_left = p;
              let bottom_right = p + I2::new(1, 0);

              // Internal ropes
              blue.try_add_rope(top_left, bottom_right, false);
              blue.try_add_rope(bottom_left, top_right, false);

              // Hoz
              blue.try_add_rope(top_left, top_right, true);
              blue.try_add_rope(bottom_left, bottom_right, true);

              // Vert
              blue.try_add_rope(top_left, bottom_left, true);
              blue.try_add_rope(top_right, bottom_right, true);
            },
            CellState::Roof => {
              let roof_top = p.to_v2().add(Vec2::new(0.5, 1.));
              let bottom_left = p;
              let bottom_right = p + I2::new(1, 0);

              blue.try_add_node(bottom_left);
              blue.try_add_node(bottom_right);
              let roof_top_id = blue.add_off_grid(roof_top);

              blue.try_add_rope(bottom_left, bottom_right, true);
              blue.try_add_rope_pos_id(bottom_left, roof_top_id, true);
              blue.try_add_rope_pos_id(bottom_right, roof_top_id, true);
            },
            _ => {},
        }

        p.y += 1;
      }

      p_ground.x += 1;
    }

    blue
  }
}