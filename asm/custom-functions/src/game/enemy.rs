use core::ffi::c_void;

use crate::{game::{actor::AcObjBase, collision::Acch}, system::math::Vec3f};

#[repr(C)]
pub struct AcEnBase {
    pub obj_base:            AcObjBase,
    pub pad0:                [u8; 0x28],
    pub fatal_blow_position: Vec3f,
    // other_stuff
}

// bokoblin
#[repr(C)]
pub struct AcEBc {
    pub enemy_base:          AcEnBase,
    pub pad0:                [u8; 0x2F8],
    pub obj_acch:    Acch,
    pub pad1: [u8; 0x95C],
    pub target_angle: i16,
    // other stuff
}


// probably wrong
#[repr(C)]
struct EnemyLinkedList {
    prev: *mut c_void,
    next: *mut c_void,
}

extern "C" {
    static ENEMY_LIST: EnemyLinkedList;
}

pub fn get_first_enemy() -> Option<*mut c_void> {
    unsafe {
        if ENEMY_LIST.prev.is_null() {
            return None;
        }

        Some(ENEMY_LIST.prev)
    }
}
