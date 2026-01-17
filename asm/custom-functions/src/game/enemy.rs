use core::ffi::c_void;

use crate::{game::{actor::AcObjBase, collision::Acch, player}, system::math::Vec3f};

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
    fn ActorLink__doFinalBlow(link: *mut player::ActorLink, enemy: *mut AcEnBase);
}

pub fn get_first_enemy() -> Option<*mut c_void> {
    unsafe {
        if ENEMY_LIST.prev.is_null() {
            return None;
        }

        Some(ENEMY_LIST.prev)
    }
}

pub fn simulate_eb(force_to_origin: bool) {
    if let Some(link) = player::as_mut() {
        if let Some(enemy) = link.get_targeted_actor() {
            if force_to_origin {
                let fb_coords = enemy.fatal_blow_position;
                enemy.fatal_blow_position = Vec3f::zero();
                unsafe {
                    ActorLink__doFinalBlow(player::as_mut().unwrap(), enemy);
                }
                enemy.fatal_blow_position = fb_coords;
            } else {
                unsafe {
                    ActorLink__doFinalBlow(player::as_mut().unwrap(), enemy);
                }
            }
        }
    }
}