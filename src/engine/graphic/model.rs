use std::mem;
use wgpu::*;

/// 一頂点のデータの構造体。
///
/// 本エンジンにおける3Dモデルデータは必ずこのレイアウトを守ること。
pub struct Vertex {
    pub _position: [f32; 4],
    pub _tex_coord: [f32; 2],
}

/// 頂点バッファのレイアウト。
pub const VERTEX_BUFFER_LAYOUTS: &[VertexBufferLayout] = &[VertexBufferLayout {
    array_stride: mem::size_of::<Vertex>() as u64,
    step_mode: VertexStepMode::Vertex,
    attributes: &[
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 0,
            shader_location: 0,
        },
        VertexAttribute {
            format: VertexFormat::Float32x2,
            offset: mem::size_of::<[f32; 4]>() as u64,
            shader_location: 1,
        },
    ],
}];
