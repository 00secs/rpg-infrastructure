use crate::{anything_to_u8slice, engine::graphic::model, slice_to_u8slice};
use glam::{Mat4, Vec4};
use std::{borrow::Cow, collections::HashMap, mem, ops::Range};
use wgpu::{util::*, *};

const SHADER: &str = "
struct Camera {
    projection: mat4x4<f32>,
    view: mat4x4<f32>,
}
@group(0)
@binding(0)
var<uniform> camera: Camera;

struct Instance {
    world: mat4x4<f32>,
    tex_coord: vec4<f32>,
}
@group(0)
@binding(1)
var<uniform> instances: array<Instance, 16>;

@group(1)
@binding(0)
var image_texture: texture_2d<f32>;

@group(1)
@binding(1)
var image_sampler: sampler;

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

    result.position = camera.projection * camera.view * instances[instance_index].world * vertex_input.position;

    result.tex_coord = vec2<f32>(
        instances[instance_index].tex_coord.x + instances[instance_index].tex_coord.z * vertex_input.tex_coord.x,
        instances[instance_index].tex_coord.y + instances[instance_index].tex_coord.w * vertex_input.tex_coord.y,
    );

    return result;
}

@fragment
fn fs_main(vertex_outout: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(image_texture, image_sampler, vertex_outout.tex_coord);
}
";

const MAX_INSTANCE_COUNT: u32 = 16;

/// カメラの構造体。
pub struct Camera {
    pub _projection: Mat4,
    pub _view: Mat4,
}

/// インスタンスの構造体。
#[derive(Clone)]
pub struct Instance {
    pub _world: Mat4,
    pub _tex_coord: Vec4,
}

/// 普通のレンダーパイプライン。
///
/// - 深度テストあり
/// - アルファブレンディングあり
/// - 拡大/縮小ともにアンチエイリアシングなし
pub struct BasePipeline {
    render_pipeline: RenderPipeline,
    camera_buffer: Buffer,
    instance_buffer: Buffer,
    sampler: Sampler,
    bind_group_1_layout: BindGroupLayout,
    bind_group_0: BindGroup,
    bind_group_1s: HashMap<&'static str, BindGroup>,
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
                            mem::size_of::<Instance>() as u64 * MAX_INSTANCE_COUNT as u64,
                        ),
                    },
                    count: None,
                },
            ],
        });

        // group(1)のレイアウトを定義
        let bind_group_1_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // パイプラインのレイアウトを定義
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_0_layout, &bind_group_1_layout],
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
            _view: Mat4::IDENTITY,
        };
        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: anything_to_u8slice(&camera),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // インスタンス配列のバッファを作成
        let instances = (0..MAX_INSTANCE_COUNT)
            .into_iter()
            .map(|_| Instance {
                _world: Mat4::IDENTITY,
                _tex_coord: Vec4::new(0.0, 0.0, 1.0, 1.0),
            })
            .collect::<Vec<Instance>>();
        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: slice_to_u8slice(instances.as_slice()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // サンプラを作成
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: None,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        // group(0)のバッファを作成
        //
        // NOTE: group(1)と異なり各フレームで一度しか更新予定がないため、
        //       一個のバインドグループを作り、バインドされているバッファを更新する。
        let bind_group_0 = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_0_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: instance_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            render_pipeline,
            camera_buffer,
            instance_buffer,
            sampler,
            bind_group_1_layout,
            bind_group_0,
            bind_group_1s: HashMap::new(),
        }
    }

    /// 画像に関するバインドグループを作成するメソッド。
    ///
    /// 既に何らかのバインドグループがidで登録済みであった場合、上書きする。
    //
    // NOTE: group(0)と異なり各フレームで何度も更新予定があるため、
    //       予めバインドグループを作成し各インスタンシング毎にセットする。
    pub fn load_bind_group_for_image(
        &mut self,
        device: &Device,
        id: &'static str,
        texture_view: &TextureView,
    ) {
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_1_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        self.bind_group_1s.insert(id, bind_group);
    }

    /// カメラバッファを更新するメソッド。
    ///
    /// WARN: カメラバッファは各フレームの描画開始前に更新すべし。
    pub fn update_camera(&self, queue: &Queue, camera: &Camera) {
        queue.write_buffer(&self.camera_buffer, 0, anything_to_u8slice(camera));
    }

    /// インスタンスバッファを更新するメソッド。
    ///
    /// WARN: インスタンスバッファは各フレームの描画開始前に更新すべし。
    /// WARN: インスタンスバッファを超過しているか否か、判定しない。
    pub fn update_instances(&self, queue: &Queue, offset: u32, instances: &[Instance]) {
        queue.write_buffer(
            &self.instance_buffer,
            offset as u64,
            slice_to_u8slice(instances),
        );
    }

    /// 描画を開始するメソッド。
    pub fn start(&self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group_0, &[]);
    }

    /// モデルをセットするメソッド。
    ///
    /// WARN: このメソッドは描画開始後に・かつ必ず一度以上呼ぶべし。
    pub fn set_model(&self, render_pass: &mut RenderPass<'_>, model: &model::Model) {
        render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
        render_pass.set_index_buffer(model.index_buffer.slice(..), IndexFormat::Uint16);
    }

    /// 描画を行うメソッド。
    ///
    /// WARN: このメソッドは描画開始後に呼ぶべし。
    /// WARN: バインドグループが作成されていない場合、描画自体が無視される。
    /// WARN: インスタンスバッファを超過しているか否か、判定しない。
    pub fn render<'a>(
        &self,
        render_pass: &mut RenderPass<'_>,
        bind_group_id: &'static str,
        model_index_count: u32,
        instances_range: Range<u32>,
    ) {
        if let Some(n) = self.bind_group_1s.get(bind_group_id) {
            render_pass.set_bind_group(1, n, &[]);
            render_pass.draw_indexed(0..model_index_count, 0, instances_range);
        }
    }
}
