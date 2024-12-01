use crate::EError;
use ab_glyph::*;
use png::Decoder;
use std::{collections::HashMap, fs::File, io::Read};

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
    ) -> Result<(Vec<u8>, u32, u32), EError> {
        // フォントを取得
        self.load_font(font_name)?;
        let font = &self.fonts[font_name].as_scaled(PxScale::from(height));

        // グリフを取得
        let outlined_glyph = font
            .outline_glyph(font.scaled_glyph(character))
            .ok_or(format!("failed to outline {character}."))?;

        // ビットマップ全体のサイズを取得
        let ww = outlined_glyph.px_bounds().width().ceil() as usize;
        let wh = (font.ascent() + outlined_glyph.px_bounds().max.y).ceil() as usize;
        let mut texture = vec![0; 4 * ww * wh];

        // グリフ全体のオフセットを取得
        let oy = (font.ascent() + outlined_glyph.px_bounds().min.y).floor() as usize;

        // ラスタライズ
        outlined_glyph.draw(|x, y, c| {
            let x = x as usize;
            let y = oy + y as usize;
            let idx = 4 * ww * y + 4 * x;
            texture[idx] = 0xff;
            texture[idx + 1] = 0xff;
            texture[idx + 2] = 0xff;
            texture[idx + 3] = (c * 255.0) as u8;
        });

        Ok((texture, ww as u32, wh as u32))
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
