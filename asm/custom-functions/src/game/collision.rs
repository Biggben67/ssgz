use core::ffi::c_void;

use crate::{game::actor::AcObjBase, system::math::{Vec3f, Vec3s}};

extern "C" {
    static mut BGS_INSTANCE: *mut BgS;
    fn Acch__GroundCheck(acch: *mut Acch, context: *mut BgS, unk: bool);
    fn BgS__GroundCross(context: *mut BgS, ground_check: *mut BgS_GndChk) -> f32;
    fn Acch__CrrPos(acch: *mut Acch, context: *mut BgS);
    fn Acch__ChkGndHit(acch: *const Acch) -> bool;
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct CcD_Stts {
    pub at_apid: i32,
    pub at_old_apid: i32,
    pub tg_apid: i32,
    pub tg_old_api: i32,
    pub field_0x10: Vec3f,
    pub field_0x1c: Vec3f,
    pub cc_move: Vec3f,
    pub actor_ptr: *mut AcObjBase,
    pub rank: i32,
}

#[repr(C)]
// #[derive(Clone)]
pub struct Acch {
    pub pad0: [u8; 0x40],
    pub flags: u32,
    pub pos_ptr: *mut Vec3f,
    pub old_pos_ptr: *mut Vec3f,
    pub speed: Vec3f, // dunno what for
    pub speed_ptr: *mut Vec3f,
    pub angle_ptr: *mut Vec3s,
    pub pad1: [u8; 0x3C], // 0x9C - 0x4C
    pub my_obj: *mut AcObjBase,
    pub pad2: [u8; 0x10],
    pub ground_height: f32,
    pub pad3: [u8; 0x120],
    pub ground_check: BgS_GndChk,
    pub pad4: [u8; 0x17C],
    // more stuff
}

impl Acch {
    pub fn check_ground_find(&self) -> bool {
        self.flags & 0x40 != 0
    }

    pub fn check_ground_hit(&self) -> bool {
        unsafe { Acch__ChkGndHit(self) }
    }

    pub fn do_ground_check(&mut self) {
        if unsafe { !BGS_INSTANCE.is_null() } {
            unsafe { Acch__GroundCheck(self, BGS_INSTANCE, true); }
        }
    }

    pub fn do_collision_checks(&mut self) {
        if unsafe { !BGS_INSTANCE.is_null() } {
            unsafe { Acch__CrrPos(self, BGS_INSTANCE); }
        }
    }

    pub unsafe fn shallow_copy(&self) -> Self {
        unsafe { core::ptr::read(self) }
    }
}

#[repr(C)]
pub struct BgS {
    pub base: [u8; 0x2EF0],
    // more stuff
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct BgS_GndChk {
    pub pad0: [u8; 0x54],
    pub pos: Vec3f,
    // more stuff
}



pub fn get_ground_height(acch: &mut Acch) -> f32 {
    if unsafe { BGS_INSTANCE.is_null() } {
        return -1e9;
    }
    unsafe { BgS__GroundCross(BGS_INSTANCE, &mut acch.ground_check as *mut BgS_GndChk) }
}

pub unsafe fn make_dummy_collider(acch: &mut Acch) -> (AcObjBase, Acch) {
    let mut dummy_obj = unsafe { (*acch.my_obj).shallow_copy() };
    let mut dummy_coll = unsafe { acch.shallow_copy() };

    // Update Acch pointers
    dummy_coll.angle_ptr = &mut dummy_obj.ac_base.rotation;
    dummy_coll.speed_ptr = &mut dummy_obj.velocity;
    dummy_coll.pos_ptr = &mut dummy_obj.ac_base.position;
    dummy_coll.old_pos_ptr = &mut dummy_obj.ac_base.position_copy; // might be wrong
    dummy_coll.my_obj = &mut dummy_obj;

    // Update Obj pointers
    dummy_obj.ac_base.position_ptr = &mut dummy_obj.ac_base.position;
    dummy_obj.stts.actor_ptr = &mut dummy_obj;

    (dummy_obj, dummy_coll)
}