use super::*;

use std::collections::HashSet;

/// メッセージボックス。
pub struct MessageBox {
    sprite: Sprite,
    message: Option<Text>,
}

impl MessageBox {
    pub fn new(image_id: &'static str, uv: Vec4) -> Self {
        let sprite = Sprite::new(image_id)
            .with_scl(Vec2::new(SCENE_WIDTH, SCENE_HEIGHT * 0.30))
            .with_pos(Vec3::new(0.0, SCENE_HEIGHT * 0.70, 41.0))
            .with_uv(uv)
            .with_col(Vec4::new(1.0, 1.0, 1.0, 0.5))
            .with_is_ui(true)
            .with_coods(CoordinateSystem::Canvas)
            .with_align(Alignment::TopLeft);
        Self {
            sprite,
            message: None,
        }
    }

    pub fn set_message(&mut self, font_name: &'static str, message: String, height: f32) {
        let message = Text::new(font_name, message, height).with_pos(Vec3::new(
            48.0,
            SCENE_HEIGHT * 0.70 + 48.0,
            40.0,
        ));
        self.message = Some(message);
    }

    pub fn collect_characters(&self, chars: &mut HashSet<(&'static str, char)>) {
        if let Some(n) = &self.message {
            n.collect_characters(chars);
        }
    }

    pub fn push_to(
        &mut self,
        instances: &mut Vec<InstanceMeta>,
        mngrs: &Managers,
        should_push_text: bool,
    ) {
        self.sprite.push_to(instances);
        if let Some(message) = &mut self.message {
            message.push_to(instances, mngrs, should_push_text);
        }
    }
}
