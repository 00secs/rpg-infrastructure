use super::*;

use wgpu::*;

/// ビットマップデータから画像のテクスチャビューを作成する関数。
pub fn create_image_texture_view(
    device: &Device,
    queue: &Queue,
    width: u32,
    height: u32,
    bitmap: &[u8],
) -> TextureView {
    let size = Extent3d {
        width: width,
        height: height,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let texture_view = texture.create_view(&TextureViewDescriptor::default());

    queue.write_texture(
        texture.as_image_copy(),
        slice_to_u8slice(bitmap),
        ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(width * 4),
            rows_per_image: None,
        },
        size,
    );
    queue.submit(None);

    texture_view
}
