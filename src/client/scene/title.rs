use super::*;
use crate::{client::component::*, engine::graphic::RenderCommand, *};
use glam::*;

pub struct TitleScene {
    text: Text,
}

impl TitleScene {
    pub fn new(mngrs: &mut Managers) -> Scene {
        let bg = Sprite::new(
            Vec3::new(SCENE_WIDTH, SCENE_HEIGHT, 1.0),
            Quat::IDENTITY,
            Vec3::new(0.0, 0.0, 90.0),
            Vec4::new(0.0, 0.0, 0.625, 0.703125),
        );
        let text = Text::new(
            mngrs,
            "UtsukushiFONT.otf",
            "aあ|".to_string(),
            Vec3::ZERO,
            24.0,
        );

        let mut instances = Vec::new();
        bg.push_to(&mut instances);
        text.push_to(&mut instances);
        mngrs.gr_mngr.update_instances(0, &instances);

        Box::new(Self { text })
    }
}

impl SceneTrait for TitleScene {
    fn update(&mut self, mngrs: &mut Managers, _: Duration) -> NextScene {
        mngrs.gr_mngr.render(&[
            RenderCommand {
                image_id: "title",
                instances_range: 0..1 as u32,
            },
            RenderCommand {
                image_id: "chars",
                instances_range: 1..(1 + self.text.instances.len()) as u32,
            },
        ]);

        None
    }
}
