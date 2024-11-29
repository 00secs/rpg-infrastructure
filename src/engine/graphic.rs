mod model;
pub mod pipeline;

use crate::EError;
use futures::executor;
use std::{ops::Range, sync::Arc};
use wgpu::*;
use winit::window::Window;

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

        Ok(Self {
            surface,
            device,
            queue,
            base_pipeline,
            square_model,
        })
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
    /// * instances_range - 描画するインスタンスのインスタンスバッファ上のインデックス範囲。
    ///
    /// 垂直同期を取るため、スレッドが待機される。
    pub fn render(&self, instances_range: Range<u32>) {
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

        self.base_pipeline.render(
            &mut command_encoder,
            &render_target_view,
            &self.square_model,
            instances_range,
        );

        self.queue.submit(Some(command_encoder.finish()));
        surface_texture.present();
    }
}
