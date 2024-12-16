use super::*;

use std::collections::HashSet;

/// 文字列コンポーネント。
pub struct Text {
    uuids: Vec<Uuid>,
    font_name: &'static str,
    text: String,
    height: f32,
    pos: Vec3,
    col: Vec4,
    param: Vec4,
    coords: CoordinateSystem,
    align: Alignment,
    should_push: bool,
}

impl Text {
    /// コンストラクタ。
    ///
    /// 初期設定として次が設定される：
    /// - 座標：(0, 0, 0)
    /// - 色：白
    /// - UIか：はい
    /// - 座標系：キャンバス座標系
    /// - アラインメント：左上詰め
    pub fn new(font_name: &'static str, text: String, height: f32) -> Self {
        let uuids = (0..text.chars().count()).map(|_| Uuid::new_v4()).collect();
        Self {
            uuids,
            font_name,
            text,
            height,
            pos: Vec3::ZERO,
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            param: Vec4::new(1.0, 0.0, 0.0, 0.0),
            coords: CoordinateSystem::Canvas,
            align: Alignment::TopLeft,
            should_push: true,
        }
    }
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.should_push = true;
    }
    pub fn set_col(&mut self, col: Vec4) {
        self.col = col;
        self.should_push = true;
    }
    pub fn set_is_ui(&mut self, is_ui: bool) {
        if is_ui {
            self.param.x = 1.0;
        } else {
            self.param.x = 0.0;
        }
        self.should_push = true;
    }
    pub fn set_coods(&mut self, coords: CoordinateSystem) {
        self.coords = coords;
        self.should_push = true;
    }
    pub fn set_align(&mut self, align: Alignment) {
        self.align = align;
        self.should_push = true;
    }
    pub fn with_pos(mut self, pos: Vec3) -> Self {
        self.set_pos(pos);
        self
    }
    pub fn with_col(mut self, col: Vec4) -> Self {
        self.set_col(col);
        self
    }
    pub fn with_is_ui(mut self, is_ui: bool) -> Self {
        self.set_is_ui(is_ui);
        self
    }
    pub fn with_coods(mut self, coords: CoordinateSystem) -> Self {
        self.set_coods(coords);
        self
    }
    pub fn with_align(mut self, align: Alignment) -> Self {
        self.set_align(align);
        self
    }
    pub fn collect_characters(&self, chars: &mut HashSet<(&'static str, char)>) {
        self.text.chars().for_each(|c| {
            chars.insert((self.font_name, c));
        });
    }
    pub fn push_to(
        &mut self,
        instances: &mut Vec<InstanceMeta>,
        mngrs: &Managers,
        should_push_text: bool,
    ) {
        let mut width = 0.0;
        let mut char_images = Vec::with_capacity(self.text.chars().count());
        for c in self.text.chars() {
            // WARN: 文字画像の情報を取得できなかった場合、その文字はスキップされる。
            if let Some(n) = mngrs.gr_mngr.get_character_image(self.font_name, c) {
                let (w, _, _, _, _) = n.scale(self.height);
                width += w;
                char_images.push(n);
            }
        }

        if char_images.is_empty() {
            return;
        }

        let mut pos = match self.coords {
            CoordinateSystem::World => self.pos,
            CoordinateSystem::Canvas => Vec3::new(
                -SCENE_WIDTH / 2.0 + self.pos.x,
                SCENE_HEIGHT / 2.0 - self.pos.y,
                self.pos.z,
            ),
        };
        match self.align {
            Alignment::Center => pos.x -= width / 2.0,
            Alignment::TopLeft => {
                let (w, _, _, _, _) = char_images[0].scale(self.height);
                pos.x += w / 2.0;
                pos.y -= self.height / 2.0;
            }
        };

        for (i, n) in char_images.iter().enumerate() {
            let (w, h, ox, oy, ad) = n.scale(self.height);
            instances.push(InstanceMeta {
                instance: BaseInstance {
                    _world: Mat4::from_scale_rotation_translation(
                        Vec3::new(w, h, 1.0),
                        Quat::IDENTITY,
                        Vec3::new(pos.x + w / 2.0 + ox, pos.y - h / 2.0 - oy, pos.z),
                    ),
                    _uv: n.uv,
                    _color: self.col,
                    _param: self.param,
                },
                uuid: self.uuids[i],
                updated: self.should_push || should_push_text,
                image_id: "chars",
                depth: pos.z,
            });
            pos.x += ad;
        }

        self.should_push = false;
    }
}
