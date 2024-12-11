use super::*;

use crate::{client::component::*, *};
use glam::*;
use map::MapScene;
use std::f32;
use winit::keyboard::KeyCode;

pub struct TitleScene {
    total_time: f32,
    bg: Sprite,
    logo: Sprite,
    text: Text,
}

impl TitleScene {
    pub fn new(mngrs: &mut Managers) -> Scene {
        let _ = mngrs.gr_mngr.load_image(&mngrs.rs_mngr, "title");

        let bg = Sprite::new("title")
            .with_scl(Vec2::new(SCENE_WIDTH, SCENE_HEIGHT))
            .with_pos(Vec3::new(0.0, 0.0, 90.0))
            .with_uv(Vec4::new(0.0, 0.0, 0.625, 0.703125))
            .with_coods(CoordinateSystem::Canvas)
            .with_align(Alignment::TopLeft);
        let logo = Sprite::new("title")
            .with_scl(Vec2::new(500.0, 250.0))
            .with_pos(Vec3::new(100.0, 100.0, 80.0))
            .with_uv(Vec4::new(0.0, 0.75, 0.25, 0.25))
            .with_coods(CoordinateSystem::Canvas)
            .with_align(Alignment::TopLeft);
        let text = Text::new(
            mngrs,
            "UtsukushiFONT.otf",
            "PRESS Z KEY TO START".to_string(),
            24.0,
        )
        .with_pos(Vec3::new(SCENE_WIDTH * 0.5, SCENE_HEIGHT * 0.75, 0.0))
        .with_align(Alignment::Center);

        Box::new(Self {
            total_time: 0.0,
            bg,
            logo,
            text,
        })
    }
}

impl SceneTrait for TitleScene {
    fn update(&mut self, mngrs: &mut Managers, duration: Duration) -> NextScene {
        // Zボタンで次のシーン
        if mngrs.in_mngr.get(&KeyCode::KeyZ) > 0 {
            return Some(Box::new(MapScene::new(mngrs)));
        }

        // PRESS Z KE(ry の色を変える
        // 2秒間で消える→現れる
        self.text.set_col(Vec4::new(
            1.0,
            1.0,
            1.0,
            (f32::consts::PI * self.total_time / 2.0).cos().abs(),
        ));

        // 描画
        let mut instances = Vec::new();
        self.bg.push_to(&mut instances);
        self.logo.push_to(&mut instances);
        self.text.push_to(&mut instances);
        mngrs.gr_mngr.render_with_metas(instances);

        self.total_time += duration.as_secs_f32();
        None
    }
}
