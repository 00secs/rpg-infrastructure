use super::*;
use crate::{
    engine::graphic::{pipeline::BaseInstance, RenderCommand},
    SCENE_HEIGHT, SCENE_WIDTH,
};
use glam::*;

pub struct TitleScene;

impl TitleScene {
    pub fn new(mngrs: &mut Managers) -> Scene {
        Box::new(Self)
    }
}

impl SceneTrait for TitleScene {
    fn update(&mut self, mngrs: &mut Managers, _: Duration) -> NextScene {
        // TODO: 必要な分だけバッファを更新するようにする。
        let instance = BaseInstance {
            _world: Mat4::from_scale(Vec3::new(SCENE_WIDTH, SCENE_HEIGHT, 1.0)),
            _tex_coord: Vec4::new(0.0, 0.0, 1.0, 1.0),
        };
        mngrs.gr_mngr.update_instances(0, &[instance]);
        mngrs.gr_mngr.render(&[RenderCommand {
            image_id: "title",
            instances_range: 0..1,
        }]);

        None
    }
}
