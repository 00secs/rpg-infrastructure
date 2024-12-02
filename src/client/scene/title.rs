use super::*;
use crate::{client::component::*, engine::graphic::RenderCommand, *};
use glam::*;
use std::f32;

// - 静的
//   - 0: 背景
//   - 1: ロゴ
// - 動的
//   - PRESS Z KEY TO START
pub struct TitleScene {
    total_time: f32,
    text: Text,
}

impl TitleScene {
    pub fn new(mngrs: &mut Managers) -> Scene {
        let _ = mngrs.gr_mngr.load_image(&mngrs.rs_mngr, "title");

        let bg = Sprite::new()
            .set_scl(Vec2::new(SCENE_WIDTH, SCENE_HEIGHT))
            .set_pos(Vec3::new(0.0, 0.0, 90.0))
            .set_uv(Vec4::new(0.0, 0.0, 0.625, 0.703125))
            .set_coods(CoordinateSystem::Canvas)
            .set_align(Alignment::TopLeft);
        let logo = Sprite::new()
            .set_scl(Vec2::new(500.0, 250.0))
            .set_pos(Vec3::new(100.0, 100.0, 80.0))
            .set_uv(Vec4::new(0.0, 0.75, 0.25, 0.25))
            .set_coods(CoordinateSystem::Canvas)
            .set_align(Alignment::TopLeft);
        let text = Text::new(
            mngrs,
            "UtsukushiFONT.otf",
            "PRESS Z KEY TO START".to_string(),
        )
        .set_pos(Vec3::new(640.0, 500.0, 10.0))
        .set_align(Alignment::Center)
        .set_coods(CoordinateSystem::Canvas);

        let mut instances = Vec::new();
        bg.push_to(&mut instances);
        logo.push_to(&mut instances);
        text.push_to(&mut instances);
        mngrs.gr_mngr.update_instances(0, &instances);

        Box::new(Self {
            total_time: 0.0,
            text,
        })
    }
}

impl SceneTrait for TitleScene {
    fn update(&mut self, mngrs: &mut Managers, duration: Duration) -> NextScene {
        // PRESS Z KE(ry の色を変える
        // 2秒間で消える→現れる
        self.text.col = self
            .text
            .col
            .with_w((f32::consts::PI * self.total_time / 2.0).cos().abs());

        // PRESS Z KE(ry のバッファを更新
        let mut instances = Vec::new();
        self.text.push_to(&mut instances);
        mngrs.gr_mngr.update_instances(2, &instances);

        // 描画
        mngrs.gr_mngr.render(&[
            RenderCommand {
                image_id: "title",
                instances_range: 0..2 as u32,
            },
            RenderCommand {
                image_id: "chars",
                instances_range: 2..(2 + self.text.len()) as u32,
            },
        ]);

        self.total_time += duration.as_secs_f32();
        None
    }
}
