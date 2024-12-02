// NOTE: 定義するだけ定義しておいて使わなくても良いので。
#![allow(dead_code)]

use crate::{
    engine::{
        graphic::{character::CharacterImage, pipeline::BaseInstance},
        Managers,
    },
    SCENE_HEIGHT, SCENE_WIDTH,
};
use glam::*;

/// 座標系を指定する列挙型。
pub enum CoordinateSystem {
    /// ワールド座標系。
    /// - 画面中央原点
    /// - 左から右へ [-SCENE_WIDTH/2, SCENE_WIDTH/2]
    /// - 下から上へ [-SCENE_HEIGHT/2, SCENE_HEIGHT/2]
    World,
    /// キャンバス座標系。
    /// - 画面左上原点
    /// - 左から右へ [0, SCENE_WIDTH]
    /// - 上から下へ [0, SCENE_HEIGHT]
    Canvas,
}

/// アラインメント(コンポーネントの原点の位置)を指定する列挙型。
pub enum Alignment {
    Center,
    TopLeft,
}

/// スプライトコンポーネント。
pub struct Sprite {
    pub scl: Vec2,
    pub rot: Quat,
    pub pos: Vec3,
    pub uv: Vec4,
    pub col: Vec4,
    pub param: Vec4,
    pub coords: CoordinateSystem,
    pub align: Alignment,
}
impl Sprite {
    pub fn new() -> Self {
        Self {
            scl: Vec2::new(1.0, 1.0),
            rot: Quat::IDENTITY,
            pos: Vec3::ZERO,
            uv: Vec4::new(0.0, 0.0, 1.0, 1.0),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            param: Vec4::ZERO,
            coords: CoordinateSystem::World,
            align: Alignment::Center,
        }
    }
    pub fn set_scl(mut self, scl: Vec2) -> Self {
        self.scl = scl;
        self
    }
    pub fn set_rot(mut self, rot: Quat) -> Self {
        self.rot = rot;
        self
    }
    pub fn set_pos(mut self, pos: Vec3) -> Self {
        self.pos = pos;
        self
    }
    pub fn set_uv(mut self, uv: Vec4) -> Self {
        self.uv = uv;
        self
    }
    pub fn set_col(mut self, col: Vec4) -> Self {
        self.col = col;
        self
    }
    pub fn set_coods(mut self, coords: CoordinateSystem) -> Self {
        self.coords = coords;
        self
    }
    pub fn set_align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }
    pub fn set_is_ui(mut self, is_ui: bool) -> Self {
        if is_ui {
            self.param.x = 1.0;
        } else {
            self.param.x = 0.0;
        }
        self
    }
    pub fn push_to(&self, instances: &mut Vec<BaseInstance>) {
        let pos = match self.coords {
            CoordinateSystem::World => self.pos,
            CoordinateSystem::Canvas => Vec3::new(
                -SCENE_WIDTH / 2.0 + self.pos.x,
                SCENE_HEIGHT / 2.0 - self.pos.y,
                self.pos.z,
            ),
        };
        let pos = match self.align {
            Alignment::Center => pos,
            Alignment::TopLeft => {
                Vec3::new(pos.x + self.scl.x / 2.0, pos.y - self.scl.y / 2.0, pos.z)
            }
        };
        instances.push(BaseInstance {
            _world: Mat4::from_scale_rotation_translation(
                Vec3::new(self.scl.x, self.scl.y, 1.0),
                self.rot,
                pos,
            ),
            _uv: self.uv,
            _color: self.col,
            _param: self.param,
        })
    }
}

/// 文字列コンポーネント。
pub struct Text {
    pub char_images: Vec<CharacterImage>,
    pub height: f32,
    pub pos: Vec3,
    pub col: Vec4,
    pub param: Vec4,
    pub coords: CoordinateSystem,
    pub align: Alignment,
}
impl Text {
    pub fn new(mngrs: &mut Managers, font_name: &'static str, text: String) -> Self {
        let mut char_images = Vec::new();
        for c in text.chars() {
            // 文字画像をロード
            // WARN: 文字画像のロードに失敗した場合、文字がスキップされる。
            // TODO: クリアに対応する
            if mngrs
                .gr_mngr
                .load_character_image(&mut mngrs.rs_mngr, font_name, c)
                .is_err()
            {
                continue;
            }
            // 文字画像情報を取得
            char_images.push(
                mngrs
                    .gr_mngr
                    .get_character_image(font_name, c)
                    .unwrap()
                    .clone(),
            );
        }
        Self {
            char_images,
            height: 24.0,
            pos: Vec3::ZERO,
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            param: Vec4::ZERO,
            coords: CoordinateSystem::Canvas,
            align: Alignment::TopLeft,
        }
    }
    pub fn set_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
    pub fn set_pos(mut self, pos: Vec3) -> Self {
        self.pos = pos;
        self
    }
    pub fn set_col(mut self, col: Vec4) -> Self {
        self.col = col;
        self
    }
    pub fn set_coods(mut self, coords: CoordinateSystem) -> Self {
        self.coords = coords;
        self
    }
    pub fn set_align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }
    pub fn set_is_ui(mut self, is_ui: bool) -> Self {
        if is_ui {
            self.param.x = 1.0;
        } else {
            self.param.x = 0.0;
        }
        self
    }
    pub fn push_to(&self, instances: &mut Vec<BaseInstance>) {
        // 座標系変換
        let mut pos = match self.coords {
            CoordinateSystem::World => self.pos,
            CoordinateSystem::Canvas => Vec3::new(
                -SCENE_WIDTH / 2.0 + self.pos.x,
                SCENE_HEIGHT / 2.0 - self.pos.y,
                self.pos.z,
            ),
        };
        // アライン
        match self.align {
            Alignment::TopLeft => (),
            Alignment::Center => {
                let mut width = 0.0;
                for n in self.char_images.iter() {
                    width += n.scale(self.height).0;
                }
                pos.x -= width / 2.0;
                pos.y += self.height / 2.0;
            }
        };
        // インスタンスデータ生成
        for n in self.char_images.iter() {
            let (w, h, oy) = n.scale(self.height);
            instances.push(BaseInstance {
                _world: Mat4::from_scale_rotation_translation(
                    Vec3::new(w, h, 1.0),
                    Quat::IDENTITY,
                    Vec3::new(pos.x + w / 2.0, pos.y - h / 2.0 - oy, pos.z),
                ),
                _uv: n.uv,
                _color: self.col,
                _param: self.param,
            });
            pos.x += w;
        }
    }
    pub fn len(&self) -> u32 {
        self.char_images.len() as u32
    }
}
