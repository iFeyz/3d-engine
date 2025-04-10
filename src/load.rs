use std::path::Path;
use wgpu::util::DeviceExt;
use tobj;
use crate::types::{BoundingBox, Triangle};

pub fn load_mesh(device: &wgpu::Device, path: &Path) -> (wgpu::Buffer, usize, BoundingBox) {
    let (models, _) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..tobj::LoadOptions::default()
        }
    ).expect("Failed to load OBJ file");

    let mut triangles = Vec::new();

    for model in models {
        let mesh = &model.mesh;
        
        for f in 0..mesh.indices.len() / 3 {
            let i0 = mesh.indices[3 * f] as usize;
            let i1 = mesh.indices[3 * f + 1] as usize;
            let i2 = mesh.indices[3 * f + 2] as usize;

            let v0 = [
                mesh.positions[3 * i0],
                mesh.positions[3 * i0 + 1],
                mesh.positions[3 * i0 + 2],
            ];
            let v1 = [
                mesh.positions[3 * i1],
                mesh.positions[3 * i1 + 1],
                mesh.positions[3 * i1 + 2],
            ];
            let v2 = [
                mesh.positions[3 * i2],
                mesh.positions[3 * i2 + 1],
                mesh.positions[3 * i2 + 2],
            ];

            // Calculer la normale
            let edge1 = [
                v1[0] - v0[0],
                v1[1] - v0[1],
                v1[2] - v0[2],
            ];
            let edge2 = [
                v2[0] - v0[0],
                v2[1] - v0[1],
                v2[2] - v0[2],
            ];
            let normal = normalize_vector([
                edge1[1] * edge2[2] - edge1[2] * edge2[1],
                edge1[2] * edge2[0] - edge1[0] * edge2[2],
                edge1[0] * edge2[1] - edge1[1] * edge2[0],
            ]);

            triangles.push(Triangle {
                v0,
                _pad0: 0,
                v1,
                _pad1: 0,
                v2,
                _pad2: 0,
                normal,
                material_index: 0, // Par défaut
            });
        }
    }

    let triangle_count = triangles.len();
    
    // Remplir le reste du buffer avec des triangles invalides si nécessaire
    while triangles.len() < 1000 {
        triangles.push(Triangle {
            v0: [0.0; 3],
            _pad0: 0,
            v1: [0.0; 3],
            _pad1: 0,
            v2: [0.0; 3],
            _pad2: 0,
            normal: [0.0; 3],
            material_index: 0,
        });
    }

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Mesh Buffer"),
        contents: bytemuck::cast_slice(&triangles),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    // Calculer la bounding box
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    
    for triangle in &triangles {
        for vertex in [triangle.v0, triangle.v1, triangle.v2].iter() {
            for i in 0..3 {
                min[i] = min[i].min(vertex[i]);
                max[i] = max[i].max(vertex[i]);
            }
        }
    }

    let bounds = BoundingBox { 
        min,
        _pad0: 0,
        max,
        _pad1: 0,
    };

    (buffer, triangle_count, bounds)
}

fn normalize_vector(v: [f32; 3]) -> [f32; 3] {
    let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / length, v[1] / length, v[2] / length]
}