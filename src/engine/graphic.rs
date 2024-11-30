mod image;
mod model;
pub mod pipeline;

use crate::{engine::resource::ResourceManager, EError};
use futures::executor;
use std::{collections::HashMap, ops::Range, sync::Arc};
use wgpu::*;
use winit::window::Window;

/// 1回のインスタンシングに必要なデータの集合体。
pub struct RenderCommand {
    pub image_id: &'static str,
    pub instances_range: Range<u32>,
}

/// WebGPUベースのレンダラ。
///
/// - リサイズ不可
/// - FIFO
pub struct GraphicManager<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    base_pipeline: pipeline::BasePipeline,
    square_model: model::Model,
    depth_texture_view: TextureView,
    image_texture_views: HashMap<&'static str, TextureView>,
}

impl<'a> GraphicManager<'a> {
    pub fn new(window: Arc<Window>) -> Result<Self, EError> {
        let backends = if cfg!(target_os = "windows") {
            Backends::DX12
        } else if cfg!(target_os = "macos") {
            Backends::METAL
        } else if cfg!(target_os = "linux") {
            Backends::VULKAN
        } else {
            Backends::all()
        };
        let instance = Instance::new(InstanceDescriptor {
            backends,
            ..Default::default()
        });

        let surface = instance.create_surface(Arc::clone(&window))?;

        let request = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        });
        let adapter = executor::block_on(request).ok_or("failed to get an adapter.".to_owned())?;

        let request = adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::default(),
                memory_hints: MemoryHints::MemoryUsage,
            },
            None,
        );
        let (device, queue) = executor::block_on(request)?;

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .filter(|f| f.is_srgb())
            .next()
            .map(|n| n.clone())
            .unwrap_or(surface_capabilities.formats[0]);
        surface.configure(
            &device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: window.inner_size().width,
                height: window.inner_size().height,
                present_mode: PresentMode::AutoVsync,
                view_formats: Vec::new(),
                alpha_mode: surface_capabilities.alpha_modes[0],
                desired_maximum_frame_latency: 2,
            },
        );

        let base_pipeline = pipeline::BasePipeline::new(
            &device,
            surface_format.into(),
            window.inner_size().width,
            window.inner_size().height,
        );

        let square_model = model::create_square_model(&device);

        let depth_texture_view = device
            .create_texture(&TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: window.inner_size().width,
                    height: window.inner_size().height,
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

        Ok(Self {
            surface,
            device,
            queue,
            base_pipeline,
            square_model,
            depth_texture_view,
            image_texture_views: HashMap::new(),
        })
    }

    /// 画像リソースをロードするメソッド。
    ///
    /// WARN: 既に画像リソースがidでロードされている場合、エラーを返す。
    pub fn load_image(
        &mut self,
        rs_mngr: &ResourceManager,
        id: &'static str,
    ) -> Result<(), EError> {
        if self.image_texture_views.contains_key(id) {
            return Err(format!("image '{id}' is already registered.").into());
        }
        let (bitmap, width, height) = rs_mngr.load_png(id)?;
        let image_texture_view = image::create_image_texture_view(
            &self.device,
            &self.queue,
            width,
            height,
            bitmap.as_slice(),
        );
        self.base_pipeline
            .load_bind_group_for_image(&self.device, id, &image_texture_view);
        self.image_texture_views.insert(id, image_texture_view);
        Ok(())
    }

    /// Baseレンダーパイプラインのカメラバッファを更新するメソッド。
    pub fn update_camera(&self, camera: &pipeline::BaseCamera) {
        self.base_pipeline.update_camera(&self.queue, camera);
    }

    /// Baseレンダーパイプラインのインスタンスバッファを更新するメソッド。
    ///
    /// WARN: インスタンスバッファを超過した分は無視される。
    pub fn update_instances(&self, offset: u32, instances: &[pipeline::BaseInstance]) {
        self.base_pipeline
            .update_instances(&self.queue, offset, instances);
    }

    /// 描画を行うメソッド。
    ///
    /// 垂直同期を取るため、スレッドが待機される。
    pub fn render(&self, commands: &[RenderCommand]) {
        let Ok(surface_texture) = self.surface.get_current_texture() else {
            // 描画先テクスチャの取得に失敗。
            // 警告レベルなので早期returnで済ます。
            return;
        };
        let render_target_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let mut command_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &render_target_view,
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

        self.base_pipeline.start(&mut render_pass);
        self.base_pipeline
            .set_model(&mut render_pass, &self.square_model);

        for n in commands {
            self.base_pipeline.render(
                &mut render_pass,
                n.image_id,
                self.square_model.index_count,
                n.instances_range.clone(),
            );
        }

        render_pass.forget_lifetime();
        self.queue.submit(Some(command_encoder.finish()));
        surface_texture.present();
    }
}
