#![allow(unused_parens)]

mod dense_grid;
mod generator;
mod rope;
mod blueprint_to_world_transform;

use gms_binder::*;
use std::ffi::CString;
use std::os::raw::c_char;
use std::time::Instant;

use generator::*;
use rope::*;

static mut GLOBAL_STATE: Option<GlobalState> = None;

struct GlobalState {
    pub t: usize,
    pub world: World,
    pub last_tick: Instant,
}

impl GlobalState {
    fn new() -> Self {
        Self {
            t: 0,
            world: World::default(),
            last_tick: Instant::now(),
        }
    }
}

gms_bind_start!("rope_lib", "rope_lib.dll", "rope");

#[no_mangle]
#[gms_bind]
pub extern "C" fn reset() -> f64 {
    unsafe {
        GLOBAL_STATE = Some(GlobalState::new());
    }
    0.0
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn add_node(x: f64, y: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        let id = state.world.add_node(x as f32, y as f32);
        id as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn set_fixed(nid: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        let mut node = state.world.get_node_mut(nid.round() as usize);
        node.node_type = NodeType::Fixed;
        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn set_node_pos(nid: f64, x: f64, y: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        let mut node = state.world.get_node_mut(nid.round() as usize);
        node.pos.x = x as f32;
        node.pos.y = y as f32;
        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn add_rope(from: f64, to: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        let id = state
            .world
            .add_rope(from.round() as usize, to.round() as usize);
        id as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn tick() -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        state.t += 1;

        let new_last_tick = Instant::now();
        let since_start = new_last_tick.duration_since(state.last_tick);
        let micros_since = since_start.as_micros() as f32;
        const SIXTY_FPS_DUR_MICROS: f32 = 1_000_000.0 / 60.0;
        let norm_dt = micros_since / SIXTY_FPS_DUR_MICROS;

        state.world.tick(norm_dt);
        state.last_tick = new_last_tick;

        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn dry_tick() -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        state.last_tick = Instant::now();

        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_node_x(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        state.world.get_node(id.round() as usize).pos.x as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_node_y(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        state.world.get_node(id.round() as usize).pos.y as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn toggle_node(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        let mut node = state.world.get_node_mut(id.round() as usize);
        node.node_type = match node.node_type {
            NodeType::Free => NodeType::Fixed,
            NodeType::Fixed => NodeType::Free,
        };

        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_node_type(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_ref().unwrap();
        match state.world.get_node(id.round() as usize).node_type {
            NodeType::Free => 0.0,
            NodeType::Fixed => 1.0,
        }
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_rope_broken(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_ref().unwrap();
        if (state.world.get_rope(id.round() as usize).broken) {
            1.0
        } else {
            0.0
        }
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_rope_from(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_ref().unwrap();
        state.world.get_rope(id.round() as usize).from as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_rope_to(id: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_ref().unwrap();
        state.world.get_rope(id.round() as usize).to as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_sim_t() -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        state.t as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn add_static_force(x: f64, y: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        state.world.forces.push(Box::new(ConstantForce {
            force: Vec2::new(x as f32, y as f32),
        }));

        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn add_inverse_square_force(strength: f64, x: f64, y: f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        state.world.forces.push(Box::new(InverseSquareForce {
            strength: strength as f32,
            pos: Vec2::new(x as f32, y as f32),
        }));

        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn blueprint(x: f64, y: f64, world_x: f64, world_y: f64) -> *const c_char {
    unsafe {
        let mut gen = Generator::new(10);
        let blueprint = gen.gen();

        let state = GLOBAL_STATE.as_mut().unwrap();
        let transform = blueprint_to_world_transform::HybridTransform::new(Vec2::new(x as f32, y as f32), Vec2::new(world_x as f32, world_y as f32), 20.);
        let boxed_transform = Box::new(transform) as Box<dyn blueprint_to_world_transform::BlueprintToWorldTransform>;

        let generated = GeneratedStructure::from_blueprint(
            &blueprint,
            &mut state.world,
            &boxed_transform,
        );

        let json = serde_json::to_string(&generated).unwrap();
        println!("{}", json);
        let c_str_json = CString::new(json).unwrap();
        let p = c_str_json.as_ptr();
        std::mem::forget(c_str_json);
        p as *const c_char
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn free_string(s: *mut c_char) -> f64 {
    unsafe {
        if (!s.is_null()) {
            let _ = CString::from_raw(s);
        }

        0.0
    }
}

gms_bind_end!();
