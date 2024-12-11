use super::*;

use character::CharacterImage;

/// 文字列コンポーネント。
pub struct Text {
    char_images: Vec<(CharacterImage, Uuid)>,
    pos: Vec3,
    col: Vec4,
    param: Vec4,
    coords: CoordinateSystem,
    align: Alignment,
    width: f32,
    height: f32,
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
    pub fn new(mngrs: &mut Managers, font_name: &'static str, text: String, height: f32) -> Self {
        let mut width = 0.0;
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
            let char_image = mngrs
                .gr_mngr
                .get_character_image(font_name, c)
                .unwrap()
                .clone();
            // 幅を取得
            let (w, _, _) = char_image.scale(height);
            // 終了
            width += w;
            char_images.push((char_image, Uuid::new_v4()));
        }
        Self {
            char_images,
            pos: Vec3::ZERO,
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            param: Vec4::new(1.0, 0.0, 0.0, 0.0),
            coords: CoordinateSystem::Canvas,
            align: Alignment::TopLeft,
            width,
            height,
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
    pub fn push_to(&mut self, instances: &mut Vec<InstanceMeta>) {
        if self.char_images.is_empty() {
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
            Alignment::Center => pos.x -= self.width / 2.0,
            Alignment::TopLeft => {
                let (w, _, _) = self.char_images[0].0.scale(self.height);
                pos.x += w / 2.0;
                pos.y -= self.height / 2.0;
            }
        };

        for (n, uuid) in &self.char_images {
            let (w, h, oy) = n.scale(self.height);
            instances.push(InstanceMeta {
                instance: BaseInstance {
                    _world: Mat4::from_scale_rotation_translation(
                        Vec3::new(w, h, 1.0),
                        Quat::IDENTITY,
                        Vec3::new(pos.x + w / 2.0, pos.y - h / 2.0 - oy, pos.z),
                    ),
                    _uv: n.uv,
                    _color: self.col,
                    _param: self.param,
                },
                uuid: uuid.clone(),
                updated: self.should_push,
                image_id: "chars",
                depth: pos.z,
            });
            pos.x += w;
        }

        self.should_push = false;
    }
}
