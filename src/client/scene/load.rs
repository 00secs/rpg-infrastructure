use super::*;

use crate::{client::component::*, *};
use glam::*;

pub struct LoadScene;

impl LoadScene {
    pub fn new(mngrs: &mut Managers) -> Scene {
        // TODO: エラー時はダイアログ出して落とす方が親切かも。
        let _ = mngrs.gr_mngr.load_image(&mngrs.rs_mngr, "load");

        // ロード画面描画
        let mut instances = Vec::new();
        Sprite::new("load")
            .with_scl(Vec2::new(SCENE_WIDTH, SCENE_HEIGHT))
            .with_uv(Vec4::new(0.0, 0.0, 1.0, 0.5625))
            .push_to(&mut instances);
        mngrs.gr_mngr.render_with_metas(instances);

        Box::new(Self)
    }
}

impl SceneTrait for LoadScene {
    fn update(&mut self, mngrs: &mut Managers, _: Duration) -> NextScene {
        Some(title::TitleScene::new(mngrs))
    }
}
