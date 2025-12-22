#[path ="./enginecore.rs"]
mod engine_core;
use raylib;

#[allow(unused)]
#[allow(nonstandard_style)]
pub enum Engine_enum_shape_3D {
    AS_BALL{r: f32},
    AS_RECTANGLE{x: f32, y: f32, z: f32},
}

#[allow(unused)]
#[allow(nonstandard_style)]
pub struct Engine_object_3D {
    position: raylib::ffi::Vector3,
    rendermodel: Option<raylib::ffi::Model>,
    color: raylib::ffi::Color,
}

#[allow(unused)]
#[allow(nonstandard_style)]
impl Engine_object_3D {
    pub fn new(position: raylib::ffi::Vector3, c: raylib::ffi::Color, shape: Option<Engine_enum_shape_3D>) -> Self{
        let res: Option<raylib::ffi::Model>;
        match shape {
            Some(Data) => {
                match Data {
                    Engine_enum_shape_3D::AS_BALL { r } => {
                        unsafe {
                            res = Some(raylib::ffi::LoadModelFromMesh(raylib::ffi::GenMeshSphere(r, 16, 16)));
                        }
                    },
                    Engine_enum_shape_3D::AS_RECTANGLE { x, y, z } => {
                        unsafe {
                            res = Some(raylib::ffi::LoadModelFromMesh(raylib::ffi::GenMeshCube(x, y, z)));
                        }
                    }
                }
            },
            None => {
                res = None;
            },
        }
        Self { position, rendermodel: res, color: c }
    }
    pub fn draw(&mut self) {
        if let Some(Data) = self.rendermodel {
            unsafe {
                raylib::ffi::DrawModel(Data, self.position, 1.0, self.color);
            }
        }
    }
}

// that's for using new function in FFI instead of prelude, more convinient for imo
#[allow(unused)]
#[allow(nonstandard_style)]
pub trait Easypaint {
    fn new(r: u8, g: u8, b: u8, a: u8) -> raylib::ffi::Color;
    fn from_prelude(p: raylib::prelude::Color) -> raylib::ffi::Color;
}
impl Easypaint for raylib::ffi::Color {
    fn new(r: u8, g: u8, b: u8, a: u8) -> raylib::ffi::Color {
        raylib::ffi::Color{r, g, b, a}
    }
    fn from_prelude(p: raylib::prelude::Color) -> raylib::ffi::Color {
        raylib::ffi::Color{r: p.r, g: p.g, b: p.b, a: p.a}
    }
}

