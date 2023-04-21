use crate::math::{vec3::Vec3, vec2::Vec2};

//credit to Djuk1c, abstracting your code into a game engine bcuz i can
#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub texture: Vec2,
    pub color: u32,
    pub lit: f32,
}
impl Vertex {
    pub fn new(pos: Vec3, normal: Vec3, texture: Vec2, color: u32, lit: f32) -> Self {
        Self { pos, normal, texture, color, lit }
    }
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub v: [Vertex; 3],
}
impl Triangle {
    pub fn new(
        p1: Vertex,
        p2: Vertex,
        p3: Vertex,
    ) -> Self {
        Self {
            v: [p1, p2, p3]
        }
    }
}

pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

