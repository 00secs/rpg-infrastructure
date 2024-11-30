use super::*;
use crate::{
    engine::graphic::{pipeline::BaseInstance, RenderCommand},
    SCENE_HEIGHT, SCENE_WIDTH,
};
use glam::*;

pub struct LoadScene;

impl LoadScene {
    pub fn new(mngrs: &mut Managers) -> Scene {
        // TODO: エラー時はダイアログ出して落とす方が親切かも。
        let _ = mngrs.gr_mngr.load_image(&mngrs.rs_mngr, "load");

        // ロード画面描画
        let instance = BaseInstance {
            _world: Mat4::from_scale(Vec3::new(SCENE_WIDTH, SCENE_HEIGHT, 1.0)),
            _tex_coord: Vec4::new(0.0, 0.0, 1.0, 1.0),
        };
        mngrs.gr_mngr.update_instances(0, &[instance]);
        mngrs.gr_mngr.render(&[RenderCommand {
            image_id: "load",
            instances_range: 0..1,
        }]);

        // リソースのロード
        let _ = mngrs.gr_mngr.load_image(&mngrs.rs_mngr, "title");

        Box::new(Self)
    }
}

impl SceneTrait for LoadScene {
    fn update(&mut self, _: &mut Managers, _: Duration) -> NextScene {
        Some(title::TitleScene::new())
    }
}
