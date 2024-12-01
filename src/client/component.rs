use crate::engine::{graphic::pipeline::BaseInstance, Managers};
use glam::*;

/// スプライトコンポーネント。
///
/// - 中央原点
pub struct Sprite {
    pub instance: BaseInstance,
}
impl Sprite {
    pub fn new(scl: Vec3, rot: Quat, pos: Vec3, uv: Vec4) -> Self {
        Self {
            instance: BaseInstance {
                _world: Mat4::from_scale_rotation_translation(scl, rot, pos),
                _tex_coord: uv,
            },
        }
    }

    pub fn push_to(&self, instances: &mut Vec<BaseInstance>) {
        instances.push(self.instance.clone())
    }
}

/// 文字列コンポーネント。
///
/// - 左上原点
/// - 左上寄せ
pub struct Text {
    pub instances: Vec<BaseInstance>,
}
impl Text {
    pub fn new(
        mngrs: &mut Managers,
        font_name: &'static str,
        text: String,
        pos: Vec3,
        height: f32,
    ) -> Self {
        let mut pos = pos;
        let mut instances = Vec::new();
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
            let char_image = mngrs.gr_mngr.get_character_image(font_name, c).unwrap();

            // インスタンス作成
            let (w, h, oy) = char_image.scale(height);
            instances.push(BaseInstance {
                _world: Mat4::from_scale_rotation_translation(
                    Vec3::new(w, h, 1.0),
                    Quat::IDENTITY,
                    Vec3::new(pos.x + w / 2.0, pos.y - h / 2.0 - oy, pos.z),
                ),
                _tex_coord: char_image.uv,
            });

            // 右へシフト
            pos.x += w;
        }
        Self { instances }
    }

    pub fn push_to(&self, instances: &mut Vec<BaseInstance>) {
        for n in self.instances.iter() {
            instances.push(n.clone());
        }
    }
}
