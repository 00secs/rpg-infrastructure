use crate::{engine::resource::ResourceManager, EError};
use glam::Vec4;
use std::collections::HashMap;
use wgpu::*;

type Key = (&'static str, char);

const CHARS_TEXTURE_WIDTH: u32 = 8192;
const CHARS_TEXTURE_HEIGHT: u32 = 8192;
const CHARACTER_HEIGHT: u32 = 24;
const CHARACTER_IMAGES_ROW_COUNT: usize = (CHARS_TEXTURE_HEIGHT / CHARACTER_HEIGHT) as usize;

/// 文字画像の情報。
pub struct CharacterImage {
    /// CharacterImagesTextureAtlasで登録されているキー。
    /// (フォント名, 文字)
    key: Key,
    /// 文字画像のためのテクスチャアトラス上のUV座標。
    pub uv: Vec4,
    /// 文字画像のためのテクスチャアトラス上の幅。
    pub width: f32,
    /// 文字画像のためのテクスチャアトラス上の高さ。
    pub height: f32,
    /// 文字画像のためのテクスチャアトラス上のスケールでのY座標のオフセット。
    pub y_offset: f32,
}
impl CharacterImage {
    /// heightを基準の高さにしたときの(幅,高さ,Y座標のオフセット)を取得するメソッド。
    pub fn scale(&self, height: f32) -> (f32, f32, f32) {
        let r = height / CHARACTER_HEIGHT as f32;
        (self.width * r, self.height * r, self.y_offset * r)
    }
}

/// 文字画像のためのテクスチャアトラスを管理するオブジェクト。
pub struct CharacterImagesTextureAtlas {
    /// 文字画像のためのテクスチャアトラス。
    pub texture: Texture,
    /// 何行目か。
    index: usize,
    /// 行中の左から何px目か。
    offset: u32,
    /// 文字画像の情報。
    character_images: Vec<Vec<CharacterImage>>,
    /// 登録されている文字画像。
    /// (フォント名, 文字)をキーに持ち、(行, 列)を値に持つ。
    registered_characters: HashMap<Key, (usize, usize)>,
}

impl CharacterImagesTextureAtlas {
    pub fn new(device: &Device, queue: &Queue) -> Self {
        // テクスチャアトラスを作成
        let texture_size = Extent3d {
            width: CHARS_TEXTURE_WIDTH,
            height: CHARS_TEXTURE_HEIGHT,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // テクスチャアトラスを0にクリア
        queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &vec![0; 4 * CHARS_TEXTURE_WIDTH as usize * CHARS_TEXTURE_HEIGHT as usize],
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * CHARS_TEXTURE_WIDTH),
                rows_per_image: None,
            },
            texture_size,
        );

        let mut character_images = Vec::with_capacity(CHARACTER_IMAGES_ROW_COUNT);
        for _ in 0..CHARACTER_IMAGES_ROW_COUNT {
            character_images.push(Vec::new());
        }

        Self {
            texture,
            index: 0,
            offset: 0,
            character_images,
            registered_characters: HashMap::new(),
        }
    }

    pub fn load(
        &mut self,
        rs_mngr: &mut ResourceManager,
        queue: &Queue,
        font_name: &'static str,
        character: char,
    ) -> Result<bool, EError> {
        // 存在チェック
        if self
            .registered_characters
            .contains_key(&(font_name, character))
        {
            return Ok(false);
        }

        // 文字画像を取得
        let result = rs_mngr.rasterize_character(font_name, character, CHARACTER_HEIGHT as f32)?;

        // 右に行けない場合、行を移動して、移動先の行をクリア
        // クリアしたか否か、記憶
        let cleared = if self.offset + result.width >= CHARS_TEXTURE_WIDTH {
            self.offset = 0;
            if self.index + 1 >= CHARACTER_IMAGES_ROW_COUNT {
                self.index = 0;
            } else {
                self.index += 1;
            }
            for n in self.character_images[self.index].iter() {
                self.registered_characters.remove(&n.key);
            }
            self.character_images[self.index].clear();
            true
        } else {
            false
        };

        // テクスチャアトラスに描き込み
        let x = self.offset;
        let y = self.index as u32 * CHARACTER_HEIGHT;
        queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d { x, y, z: 0 },
                aspect: TextureAspect::All,
            },
            &result.texture,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * result.width),
                rows_per_image: None,
            },
            Extent3d {
                width: result.width,
                height: result.height,
                depth_or_array_layers: 1,
            },
        );

        // 登録
        let character_image = CharacterImage {
            key: (font_name, character),
            uv: Vec4::new(
                x as f32 / CHARS_TEXTURE_WIDTH as f32,
                y as f32 / CHARS_TEXTURE_HEIGHT as f32,
                result.width as f32 / CHARS_TEXTURE_WIDTH as f32,
                result.height as f32 / CHARS_TEXTURE_HEIGHT as f32,
            ),
            width: result.width as f32,
            height: result.height as f32,
            y_offset: result.y_offset,
        };
        self.registered_characters.insert(
            (font_name, character),
            (self.index, self.character_images[self.index].len()),
        );
        self.character_images[self.index].push(character_image);
        self.offset += result.width;

        Ok(cleared)
    }

    pub fn get(&self, font_name: &'static str, character: char) -> Option<&CharacterImage> {
        self.registered_characters
            .get(&(font_name, character))
            .map(|(i, j)| &self.character_images[*i][*j])
    }
}
