use crate::slice_to_u8slice;
use std::mem;
use wgpu::{util::*, *};

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

/// モデルデータの構造体。
pub struct Model {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_count: usize,
}

/// 正方形モデルを作成する関数。
pub fn create_square_model(device: &Device) -> Model {
    const VERTICES: &[Vertex] = &[
        // 左下
        Vertex {
            _position: [-0.5, -0.5, 0.0, 1.0],
            _tex_coord: [0.0, 1.0],
        },
        // 左上
        Vertex {
            _position: [-0.5, 0.5, 0.0, 1.0],
            _tex_coord: [0.0, 0.0],
        },
        // 右上
        Vertex {
            _position: [0.5, 0.5, 0.0, 1.0],
            _tex_coord: [1.0, 0.0],
        },
        // 右下
        Vertex {
            _position: [0.5, -0.5, 0.0, 1.0],
            _tex_coord: [1.0, 1.0],
        },
    ];
    const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: slice_to_u8slice(VERTICES),
        usage: BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: slice_to_u8slice(INDICES),
        usage: BufferUsages::INDEX,
    });
    Model {
        vertex_buffer,
        index_buffer,
        index_count: INDICES.len(),
    }
}
