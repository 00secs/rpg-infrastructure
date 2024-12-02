use super::*;
use crate::{client::component::*, engine::graphic::RenderCommand, *};
use glam::*;

pub struct LoadScene;

impl LoadScene {
    pub fn new(mngrs: &mut Managers) -> Scene {
        // TODO: エラー時はダイアログ出して落とす方が親切かも。
        let _ = mngrs.gr_mngr.load_image(&mngrs.rs_mngr, "load");

        // ロード画面描画
        let bg = Sprite::new()
            .set_scl(Vec2::new(SCENE_WIDTH, SCENE_HEIGHT))
            .set_uv(Vec4::new(0.0, 0.0, 1.0, 0.5625));
        let mut instances = Vec::new();
        bg.push_to(&mut instances);
        mngrs.gr_mngr.update_instances(0, &instances);
        mngrs.gr_mngr.render(&[RenderCommand {
            image_id: "load",
            instances_range: 0..1,
        }]);

        Box::new(Self)
    }
}

impl SceneTrait for LoadScene {
    fn update(&mut self, mngrs: &mut Managers, _: Duration) -> NextScene {
        Some(title::TitleScene::new(mngrs))
    }
}
