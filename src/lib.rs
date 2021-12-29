//use std::os::raw::c_char;
//use std::ffi::{CString};
#[macro_use]
extern crate gms_binder;
use gms_binder::*;

use std::time::Instant;

mod rope;

static mut GLOBAL_STATE : Option<GlobalState> = None;

struct GlobalState
{
    pub t : usize,
    pub rope_state : rope::World,
    pub last_tick : Instant,
}

impl GlobalState
{
    fn new() -> Self {
        Self {
            t: 0,
            rope_state: rope::World::default(),
            last_tick : Instant::now(),
        }
    }
}

//#macro_rules! bind {
//    ($name : expr, $expr : expr) => {
//        #[no_mangle]
//        pub extern "C" fn $name() -> f64 {
//            _ = $expr
//            0.0
//        }
//    };
//}
//
//bind!(reset, unsafe {
//    GLOBAL_STATE = Some(GlobalState::new());
//});
//
//bind_ret!(add_node, x, y, {
//    unsafe {
//        let state = GLOBAL_STATE.as_mut().unwrap();
//        let id = state.rope_state.add_node(x as f32, y as f32);
//        id
//    }
//}

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
pub extern "C" fn add_node(x : f64, y : f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        let id = state.rope_state.add_node(x as f32, y as f32);
        id as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn set_fixed(nid : f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        let mut node = state.rope_state.get_node_mut(nid.round() as usize);
        node.node_type = rope::NodeType::Fixed;
        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn add_rope(from : f64, to : f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        let id = state.rope_state.add_rope(from.round() as usize, to.round() as usize);
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
        const SIXTY_FPS_DUR_MICROS : f32 = 1_000_000.0 / 60.0;
        let norm_dt = micros_since / SIXTY_FPS_DUR_MICROS;

        state.rope_state.tick(norm_dt);
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
pub extern "C" fn get_node_x(id : f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        state.rope_state.get_node(id.round() as usize).pos.x as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_node_y(id : f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        state.rope_state.get_node(id.round() as usize).pos.y as f64
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn toggle_node(id : f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        let mut node = state.rope_state.get_node_mut(id.round() as usize);
        node.node_type = match node.node_type {
            rope::NodeType::Free => rope::NodeType::Fixed,
            rope::NodeType::Fixed => rope::NodeType::Free,
        };

        0.0
    }
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn get_node_type(id : f64) -> f64 {
    unsafe {
        let state = GLOBAL_STATE.as_mut().unwrap();
        match state.rope_state.get_node(id.round() as usize).node_type {
            rope::NodeType::Free => 0.0,
            rope::NodeType::Fixed => 1.0,
        }
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

gms_bind_end!();