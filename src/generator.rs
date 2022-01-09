use std::collections::HashSet;
use std::hash::Hash;

use rand::RngCore;
use serde::{Deserialize, Serialize};
use froggy_rand::FroggyRand;

use crate::dense_grid::{DenseGrid, I2};
use crate::rope::*;

#[derive(Default, Serialize)]
pub struct GeneratedStructure {
    floor_pinned_nodes: Vec<usize>,
    nodes: Vec<usize>,
    ropes: Vec<usize>,
    ropes_nodraw: Vec<usize>,
}

impl GeneratedStructure {
    pub fn from_blueprint(
        blue: &Blueprint,
        world: &mut World,
        transform : &Box<dyn crate::blueprint_to_world_transform::BlueprintToWorldTransform>,
    ) -> Self {

        let mut generated = Self::default();
        let mut realised_node_ids: Vec<usize> = vec![];

        for (node_pos, is_floor_pinned) in &blue.nodes {
            let p = transform.transform(*node_pos);

            let id = world.add_node(p.x, p.y);
            realised_node_ids.push(id);
            if (*is_floor_pinned) {
                generated.floor_pinned_nodes.push(id);
            } else {
                generated.nodes.push(id);
            }
        }

        for (from, to, visible) in &blue.ropes {
            let rope_id = world.add_rope(realised_node_ids[*from], realised_node_ids[*to]);
            if (*visible) {
                generated.ropes.push(rope_id);
            } else {
                generated.ropes_nodraw.push(rope_id)
            }
        }

        generated
    }
}

#[derive(Default)]
pub struct Blueprint {
    nodes: Vec<(Vec2, bool)>,
    ropes: Vec<(usize, usize, bool)>,

    node_grid: DenseGrid<Option<usize>>,
    rope_hashset: HashSet<(usize, usize)>,
}

impl Blueprint {
    fn add_off_grid(&mut self, pos: Vec2) -> usize {
        let id = self.nodes.len();
        self.nodes.push((pos, pos.y == 0.0));
        id
    }

    fn try_add_node(&mut self, pos: I2) {
        let grid_id = self.node_grid.get_mut(pos);
        if (grid_id.is_none()) {
            let grounded = pos.y == 0;
            let id = self.nodes.len();
            self.nodes.push((pos.to_v2(), grounded));
            *grid_id = Some(id);
        }
    }

    fn try_add_rope(&mut self, from: I2, to: I2, visible: bool) {
        if let Some(from_node_id) = self.node_grid.get(from) {
            if let Some(to_node_id) = self.node_grid.get(to) {
              self.try_add_rope_ids(from_node_id, to_node_id, visible);
            }
        }
    }

    fn try_add_rope_pos_id(&mut self, from: I2, to_node_id: usize, visible: bool) {
        if let Some(from_node_id) = self.node_grid.get(from) {
            self.try_add_rope_ids(from_node_id, to_node_id, visible);
        }
    }

    fn try_add_rope_ids(&mut self, from : usize, to : usize, visible: bool) {
        if (from == to) {
          return;
        }

        let smaller = from.min(to);
        let larger = from.max(to);

        if (self.rope_hashset.insert((smaller, larger))) {
            self.ropes.push((from, to, visible));
        }
    }
}

pub struct Generator {
    froggy_rand : FroggyRand,
}

impl Generator {
    pub fn new(input_seed: u8) -> Self {
        /*
        let mut seed: <SmallRng as SeedableRng>::Seed = Default::default();
        seed[0] = input_seed;

        Self {
            rng: SmallRng::from_seed(seed),
        }
        */
        Self {
            froggy_rand: FroggyRand::new(rand::thread_rng().next_u64()),
        }
    }

    fn add_tower_roof<T : Hash + Copy>(&self, int : &mut IntermediateStructure, seed : T, x : i32, y : i32) {
        let height = self.froggy_rand.gen_froggy(("height", seed), 1., 4., 3).round() as u32;

        for iy in 0..height {
            int.set(I2::new(x, y + iy as i32), CellState::EdgeBlock);
        }

        let roof = self.froggy_rand.gen_unit(("roof", x, seed)) < 0.5;
        
        if (roof) {
            int.set(I2::new(x, y + height as i32), CellState::Roof);
        }
    }

    fn gen_gothic_house<T : Hash + Copy>(&self, seed : T) -> IntermediateStructure {
        let mut int = IntermediateStructure::default();

        let half_width = *self.froggy_rand.choose(("half_width", seed), &[1, 2, 3]);

        for x in -half_width..=half_width {
            self.add_tower_roof(&mut int, ("roofing", x, seed), x, 0);
        }

        int
    }

    fn gen_industrial<T : Hash + Copy>(&self, seed : T) -> IntermediateStructure {
        let mut int = IntermediateStructure::default();

        let block_count = *self.froggy_rand.choose(("block_count", seed), &[2, 3]);
        let mut width = *self.froggy_rand.choose(("width", seed), &[3, 3, 5]);
        let mut half_width = width / 2;

        let mut y = 0;

        for i in 0..block_count {
            let stilt_height = self.froggy_rand.gen_froggy(("stilt_height", i, seed), 1.2, 3., 3).round() as i32;
            int.set_block(I2::new(-half_width, y), I2::new(1, stilt_height), CellState::Scaffolding);
            int.set_block(I2::new(half_width, y), I2::new(1, stilt_height), CellState::Scaffolding);
            y += stilt_height;

            let width_mod = *self.froggy_rand.choose(("width_mod", i, seed), &[-2, 0, 0, 2]);
            width += width_mod;
            half_width += width_mod / 2;

            if (width <= 1) {
                return int;
            }

            let block_height = self.froggy_rand.gen_froggy(("block_height", i, seed), 1., 2.8, 3).round() as i32;
            int.set_block(I2::new(-half_width, y), I2::new(width, block_height), CellState::EdgeBlock);
            y += block_height;
        }

        for x in -half_width..half_width {
            self.add_tower_roof(&mut int, ("roofing", x, seed), x, y);
        }

        int
    }

    fn gen_tower<T : Hash>(&self, seed : T) -> IntermediateStructure {
        let mut int = IntermediateStructure::default();

        let height = self.froggy_rand.gen_froggy(("tower_height", seed), 3., 6., 3).round() as u32;

        for y in 0..height {
            int.set(I2::new(0, y as i32), CellState::EdgeBlock);
        }

        int.set(I2::new(0, height as i32), CellState::Roof);

        int
    }

    pub fn gen(&self) -> Blueprint {
        match self.froggy_rand.gen_usize_range("type", 0, 2) {
            0 => self.gen_gothic_house("gothic"),
            1 => self.gen_industrial("industrial"),
            _ => self.gen_tower("tower"),
        }.to_blueprint()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CellState {
    Empty,
    Roof,
    EdgeBlock,
    Scaffolding,
}

impl Default for CellState {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Default)]
struct IntermediateStructure {
    grid: DenseGrid<CellState>,
}

impl IntermediateStructure {
    pub fn is_solid(&mut self, pos: I2) -> bool {
        *self.grid.get_mut(pos) != CellState::Empty
    }

    pub fn set(&mut self, pos: I2, val: CellState) {
        *self.grid.get_mut(pos) = val
    }

    pub fn set_block(&mut self, corner : I2, size : I2, val : CellState) {
        for ix in 0..size.x {
            for iy in 0..size.y {
                self.set(I2::new(corner.x + ix, corner.y + iy), val);
            }
        }
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
                let cell_state = self.grid.get(p); 
                match cell_state {
                    CellState::EdgeBlock | CellState::Scaffolding => {
                        let corners = [I2::new(0, 0), I2::new(1, 0), I2::new(0, 1), I2::new(1, 1)];

                        for offset in &corners {
                            blue.try_add_node(p + *offset);
                        }

                        let top_left = p + I2::new(0, 1);
                        let top_right = p + I2::new(1, 1);
                        let bottom_left = p;
                        let bottom_right = p + I2::new(1, 0);

                        let draw_all_ropes = cell_state == CellState::Scaffolding;

                        // Internal ropes
                        blue.try_add_rope(top_left, bottom_right, draw_all_ropes);
                        blue.try_add_rope(bottom_left, top_right, draw_all_ropes);

                        // Hoz
                        let draw_up_rope = if draw_all_ropes { true } else {
                            match self.grid.get(p + I2::new(0, 1)) {
                                CellState::Empty => true,
                                CellState::Roof => false,
                                _ => false,
                            }

                            /*
                            // This looks kinda cool
                            true
                            */
                        };
                        blue.try_add_rope(top_left, top_right, draw_up_rope);

                        let draw_down_rope = draw_all_ropes || self.grid.get(p + I2::new(0, -1)) != CellState::EdgeBlock;
                        blue.try_add_rope(bottom_left, bottom_right, draw_down_rope);

                        // Vert
                        let draw_left_rope = draw_all_ropes || self.grid.get(p + I2::new(-1, 0)) != CellState::EdgeBlock;
                        blue.try_add_rope(top_left, bottom_left, draw_left_rope);

                        let draw_right_rope = draw_all_ropes || self.grid.get(p + I2::new(1, 0)) != CellState::EdgeBlock;
                        blue.try_add_rope(top_right, bottom_right, draw_right_rope);
                    }
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
                    }
                    _ => {}
                }

                p.y += 1;
            }

            p_ground.x += 1;
        }

        blue
    }
}