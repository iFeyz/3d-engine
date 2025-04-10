use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BoundingBox {
    pub min: [f32; 3],
    pub _pad0: u32,  // Padding pour l'alignement
    pub max: [f32; 3],
    pub _pad1: u32,  // Padding pour l'alignement
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Triangle {
    pub v0: [f32; 3],
    pub _pad0: u32,
    pub v1: [f32; 3],
    pub _pad1: u32,
    pub v2: [f32; 3],
    pub _pad2: u32,
    pub normal: [f32; 3],
    pub material_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub position: [f32; 3],
    pub _pad0: u32,
    pub color: [f32; 3],
    pub intensity: f32,
} 