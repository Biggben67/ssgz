use core::f32::consts;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vec3s {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Matrix34f {
    pub m: [[f32; 4]; 3],
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Matrix44f {
    pub m: [[f32; 4]; 4],
}

extern "C" {
    pub fn C_MTXOrtho(
        m: *mut Matrix44f,
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
        near: f32,
        far: f32,
    );
    pub fn PSMTXIdentity(mtx: *mut Matrix34f);
    fn EGGSqrt(x: f32) -> f32;
    fn EGGSin(ang: f32) -> f32;
    fn EGGCos(ang: f32) -> f32;
    fn EGGAcos(x: f32) -> f32;
    fn EGGAtan2(y: f32, x: f32) -> f32;
}

// helper math functions

pub fn sqrt(x: f32) -> f32 {
    unsafe { EGGSqrt(x) }
}

pub fn sin(ang: f32) -> f32 {
    unsafe { EGGSin(ang) }
}

pub fn cos(ang: f32) -> f32 {
    unsafe { EGGCos(ang) }
}

pub fn acos(x: f32) -> f32 {
    unsafe { EGGAcos(x) }
}

pub fn atan2(y: f32, x: f32) -> f32 {
    unsafe { EGGAtan2(y, x) }
}

const MIN_ANGLE: f32 = consts::TAU / 65536f32;

pub fn short_to_rad(ang: i16) -> f32 {
    (ang as f32) * MIN_ANGLE
}

pub fn rad_to_deg(ang: f32) -> f32 {
    ang * 180f32 * consts::FRAC_1_PI
}

impl Vec3f {
    pub fn normalize(&mut self) {
        let len = self.len();
        if len != 0f32 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        sqrt(self.len_squared())
    }

    pub fn dot(a: &Vec3f, b: &Vec3f) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn from_short(ang: i16) -> Self {
        let rad = short_to_rad(ang);
        let x = sin(rad);
        let z = cos(rad);
        Self {
            x, y: 0f32, z
        }
    }

    pub fn sub(&mut self, other: &Vec3f) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}