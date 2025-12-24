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
pub struct Engine_body_3D {
    position: raylib::ffi::Vector3,
    rendermodel: Option<raylib::ffi::Model>,
    geometry: Option<Engine_enum_shape_3D>,
    color: raylib::ffi::Color,
}

#[allow(unused)]
#[allow(nonstandard_style)]
impl Engine_body_3D {
    pub fn new(position: raylib::ffi::Vector3, geometry: Option<Engine_enum_shape_3D>, color: raylib::ffi::Color) -> Self {
        match geometry {
            Some(gm) => {
                match gm {
                    Engine_enum_shape_3D::AS_RECTANGLE { x, y, z } => {
                        unsafe {
                            return Self{position, rendermodel: Some(raylib::ffi::LoadModelFromMesh(raylib::ffi::GenMeshCube(x, y, z))), geometry: Some(gm), color};
                        }
                    },
                    Engine_enum_shape_3D::AS_BALL { r } => {
                        unsafe {
                            return Self {position, rendermodel: Some(raylib::ffi::LoadModelFromMesh(raylib::ffi::GenMeshSphere(r, 16, 16))), geometry: Some(gm), color};
                        }
                    }
                }
            },
            None => {},
        }
        Self {position , rendermodel: None, geometry, color }
    }
    pub fn Unload(&mut self) {
        if let Some(rm) = self.rendermodel {
            unsafe {
                raylib::ffi::UnloadModel(rm);
            }
            self.rendermodel = None;
        }
    }
    pub fn Draw(&mut self) {
        if let Some(rm) = self.rendermodel {
            unsafe {
                raylib::ffi::DrawModel(rm, self.position, 1.0, self.color);
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

#[allow(unused)]
#[allow(nonstandard_style)]
#[derive(Debug, Clone, Copy)]
pub struct Parent(Option<engine_core::Entity>);

#[allow(unused)]
#[allow(nonstandard_style)]
pub struct Children(Option<Vec<engine_core::Entity>>);


// Gotta rework it somehow so i dont need to ask for current entity in functions but still get current entity
#[allow(unused)]
#[allow(nonstandard_style)]
impl Parent {
    pub fn setParent(&mut self, currentEntity: engine_core::Entity, np:engine_core::Entity, univ: &mut engine_core::Universe){
        if let Some(p) = self.0 {
            let old_parent: engine_core::Entity = p;
            if old_parent == np {
                return;
            }
            if let Some(oldChildren) = univ.get_component_mut::<Children>(old_parent) {
                Children::removeChild(oldChildren, currentEntity);
            }
            self.0 = Some(np);
            if let Some(newChildren) = univ.get_component_mut::<Children>(np) {
                newChildren.addChild(currentEntity);
            } else {
                let mut nchildren: Children = Children(None);
                nchildren.addChild(currentEntity);
            }
        }
    }
    pub fn getParent(&self) -> Option<engine_core::Entity> {
        self.0
    }
    pub fn getParent_ref(&self) -> Option<&engine_core::Entity> {
        self.0.as_ref()
    }
    pub fn getParent_mut_ref(&mut self) -> Option<&mut engine_core::Entity> {
        self.0.as_mut()
    }
}

#[allow(unused)]
#[allow(nonstandard_style)]
pub trait GetCurrentEntity {
    fn GetContextEntity(&self) -> engine_core::Entity;
}

#[allow(unused)]
#[allow(nonstandard_style)]
pub struct EntityContext {
    currentEntity: engine_core::Entity,
}

impl GetCurrentEntity for EntityContext {
    fn GetContextEntity(&self) -> engine_core::Entity {
        self.currentEntity
    }
}

#[allow(unused)]
#[allow(nonstandard_style)]
impl Children {
    pub fn hasChildren(&self) -> bool{
        if let Some(c) = &self.0 {
            if c.len() > 0 {return true};
        }
        false
    }
    pub fn addChild(&mut self, child: engine_core::Entity) {
        if let Some(c) = &mut self.0 {
            c.push(child);
        } else {
            // handling problem where vector does not exists
            self.0 = Some(Vec::new());
            if let Some(handleNoChild) = &mut self.0 {
                handleNoChild.push(child);
            }
        }
    }
    pub fn removeChild(&mut self, child: engine_core::Entity) {
        if let Some(vec_children) = &mut self.0 {
            vec_children.retain(|&v_child| v_child != child);
        }
    }
}