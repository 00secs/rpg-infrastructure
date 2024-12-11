use super::*;

use ab_glyph::*;
use png::Decoder;
use std::{collections::HashMap, fs::File, io::Read};

pub struct CharacterRasterizedResult {
    pub texture: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub y_offset: f32,
}

/// 外部リソースを管理するオブジェクト。
//
// NOTE: 将来的にリソースをすべて.datファイルにまとめて
//       常にオープンしておいて適宜シークして各リソースを読み出すために
//       このオブジェクトを介す。
pub struct ResourceManager {
    fonts: HashMap<String, FontVec>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    pub fn load_png(&self, id: &str) -> Result<(Vec<u8>, u32, u32), EError> {
        // TODO: .datファイルから読み出す
        let file = File::open(format!("res/{id}.png"))?;

        let mut reader = Decoder::new(file).read_info()?;
        let mut texture = vec![0; reader.output_buffer_size()];
        let output_info = reader.next_frame(&mut texture)?;

        Ok((texture, output_info.width, output_info.height))
    }

    pub fn rasterize_character(
        &mut self,
        font_name: &str,
        character: char,
        height: f32,
    ) -> Result<CharacterRasterizedResult, EError> {
        // フォントを取得
        self.load_font(font_name)?;
        let font = &self.fonts[font_name].as_scaled(PxScale::from(height));

        // グリフを取得
        let Some(outlined_glyph) = font.outline_glyph(font.scaled_glyph(character)) else {
            let ww = font.h_advance(font.glyph_id(character)).ceil() as usize;
            let wh = 2;
            let texture: Vec<u8> = vec![0x00; 4 * ww * wh];
            return Ok(CharacterRasterizedResult {
                texture,
                width: ww as u32,
                height: wh as u32,
                y_offset: 0.0,
            });
        };

        // ビットマップを作成
        let ww = outlined_glyph.px_bounds().width().ceil() as usize;
        let wh = outlined_glyph.px_bounds().height().ceil() as usize;
        let mut texture = vec![0xff; 4 * ww * wh];

        // ラスタライズ
        outlined_glyph
            .draw(|x, y, c| texture[4 * ww * y as usize + 4 * x as usize + 3] = (c * 255.0) as u8);

        Ok(CharacterRasterizedResult {
            texture,
            width: ww as u32,
            height: wh as u32,
            y_offset: font.ascent() + outlined_glyph.px_bounds().min.y,
        })
    }

    fn load_font(&mut self, font_name: &str) -> Result<(), EError> {
        if self.fonts.contains_key(font_name) {
            Ok(())
        } else {
            // TODO: .datファイルから読み出す。
            let mut file = File::open(format!("res/{font_name}"))?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;

            self.fonts
                .insert(font_name.to_string(), FontVec::try_from_vec(buf)?);
            Ok(())
        }
    }
}
