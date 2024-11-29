use crate::engine::graphic::model;
use crate::{anything_to_u8slice, slice_to_u8slice};
use glam::{Mat4, Vec4};
use std::{borrow::Cow, mem};
use wgpu::{util::*, *};

const SHADER: &str = "
struct Camera {
    projection: mat4x4<f32>,
}
@group(0)
@binding(0)
var<uniform> camera: Camera;

struct Instance {
    model: mat4x4<f32>,
    tex_coord: vec4<f32>,
}
@group(0)
@binding(1)
var<uniform> instances: array<Instance, 16>;

// @group(1)
// @binding(0)
// var image_texture: texture_2d<f32>;

// @group(1)
// @binding(1)
// var image_sampler: sampler;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) tex_coord: vec2<f32>,
}
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
}

@vertex
fn vs_main(
    @builtin(instance_index) instance_index: u32,
    vertex_input: VertexInput,
) -> VertexOutput {
    var result: VertexOutput;

    result.position = camera.projection * instances[instance_index].model * vertex_input.position;

    result.tex_coord = vec2<f32>(
        instances[instance_index].tex_coord.x + instances[instance_index].tex_coord.z * vertex_input.tex_coord.x,
        instances[instance_index].tex_coord.y + instances[instance_index].tex_coord.w * vertex_input.tex_coord.y,
    );

    return result;
}

@fragment
fn fs_main(vertex_outout: VertexOutput) -> @location(0) vec4<f32> {
    // return textureSample(image_texture, image_sampler, vertex_outout.tex_coord);
    return vec4(1.0, 1.0, 1.0, 1.0);
}
";

const MAX_INSTANCE_COUNT: u64 = 16;

struct Camera {
    _projection: Mat4,
}

#[derive(Clone)]
struct Instance {
    _model: Mat4,
    _tex_coord: Vec4,
}

/// 普通のレンダーパイプライン。
///
/// カメラについて:
/// - 平行投影
/// - 位置は(0,0,0)で固定
/// - 深さは100で固定
///
/// 他:
/// - 深度テストあり
/// - アルファブレンディングあり
pub struct BasePipeline {
    render_pipeline: RenderPipeline,
    depth_texture_view: TextureView,
    _camera_buffer: Buffer,
    instance_buffer: Buffer,
    bind_group_0: BindGroup,
    // bind_group_1: BindGroup,
}

impl BasePipeline {
    pub fn new(
        device: &Device,
        color_target_state: ColorTargetState,
        width: u32,
        height: u32,
    ) -> Self {
        // WGSLからシェーダモジュールを作成
        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::from(SHADER)),
        });

        // group(0)のレイアウトを定義
        let bind_group_0_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(mem::size_of::<Camera>() as u64),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            mem::size_of::<Instance>() as u64 * MAX_INSTANCE_COUNT,
                        ),
                    },
                    count: None,
                },
            ],
        });

        // group(1)のレイアウトを定義
        // let bind_group_1_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        //     label: None,
        //     entries: &[
        //         BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: ShaderStages::FRAGMENT,
        //             ty: BindingType::Texture {
        //                 sample_type: TextureSampleType::Float { filterable: true },
        //                 view_dimension: TextureViewDimension::D2,
        //                 multisampled: false,
        //             },
        //             count: None,
        //         },
        //         BindGroupLayoutEntry {
        //             binding: 1,
        //             visibility: ShaderStages::FRAGMENT,
        //             ty: BindingType::Sampler(SamplerBindingType::Filtering),
        //             count: None,
        //         },
        //     ],
        // });

        // パイプラインのレイアウトを定義
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            // bind_group_layouts: &[&bind_group_0_layout, &bind_group_1_layout],
            bind_group_layouts: &[&bind_group_0_layout],
            push_constant_ranges: &[],
        });

        // パイプラインを作成
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: model::VERTEX_BUFFER_LAYOUTS,
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: color_target_state.format,
                    blend: Some(BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                    }),
                    write_mask: color_target_state.write_mask,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
            }),
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        /* 以降、リソース作成 */

        // 深度テクスチャのビューを作成
        let depth_texture_view = device
            .create_texture(&TextureDescriptor {
                label: None,
                size: Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32Float,
                usage: TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
            .create_view(&TextureViewDescriptor::default());

        // カメラのバッファを作成
        let half_width = width as f32 / 2.0;
        let half_height = height as f32 / 2.0;
        let camera: Camera = Camera {
            _projection: Mat4::orthographic_lh(
                -half_width,
                half_width,
                -half_height,
                half_height,
                0.0,
                100.0,
            ),
        };
        let _camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: anything_to_u8slice(&camera),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // インスタンス配列のバッファを作成
        let instances = (0..MAX_INSTANCE_COUNT)
            .into_iter()
            .map(|_| Instance {
                _model: Mat4::IDENTITY,
                _tex_coord: Vec4::new(0.0, 0.0, 0.0, 1.0),
            })
            .collect::<Vec<Instance>>();
        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: slice_to_u8slice(instances.as_slice()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // group(0)のバッファを作成
        let bind_group_0 = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_0_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: _camera_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: instance_buffer.as_entire_binding(),
                },
            ],
        });

        // group(1)のバッファを作成
        // let bind_group_1 = device.create_bind_group(&BindGroupDescriptor {
        //     label: None,
        //     layout: &bind_group_1_layout,
        //     entries: &[
        //         BindGroupEntry {
        //             binding: 0,
        //             resource: std::ptr::null(),
        //         },
        //         BindGroupEntry {
        //             binding: 1,
        //             resource: std::ptr::null(),
        //         },
        //     ],
        // });

        Self {
            render_pipeline,
            depth_texture_view,
            _camera_buffer,
            instance_buffer,
            bind_group_0,
            // bind_group_1,
        }
    }

    /// 描画を行うメソッド。
    ///
    /// 各フレームの最初に呼ばれるパイプラインであることを想定しているため、描画先テクスチャを(0,0,0,1)にクリアする。
    pub fn render<'a>(
        &self,
        command_encoder: &'a mut CommandEncoder,
        render_target_view: &TextureView,
        model: &model::Model,
    ) {
        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: render_target_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &self.depth_texture_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group_0, &[]);
        // render_pass.set_bind_group(0, &self.bind_group_1, &[]);

        render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
        render_pass.set_index_buffer(model.index_buffer.slice(..), IndexFormat::Uint16);

        render_pass.draw_indexed(0..model.index_count as u32, 0, 0..1);
    }
}
