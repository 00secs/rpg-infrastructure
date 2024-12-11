use super::*;

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

    pub fn set_message(
        &mut self,
        mngrs: &mut Managers,
        font_name: &'static str,
        message: String,
        height: f32,
    ) {
        let message = Text::new(mngrs, font_name, message, height).with_pos(Vec3::new(
            48.0,
            SCENE_HEIGHT * 0.70 + 48.0,
            40.0,
        ));
        self.message = Some(message);
    }

    pub fn push_to(&mut self, instances: &mut Vec<InstanceMeta>) {
        self.sprite.push_to(instances);
        if let Some(message) = &mut self.message {
            message.push_to(instances);
        }
    }
}
