use crate::engine::{
    graphic::{pipeline::BaseInstance, RenderCommand},
    ClientHandler, Managers,
};
use glam::{Mat4, Quat, Vec3, Vec4};
use std::time::Duration;

/// ゲームを管理するオブジェクト。
pub struct GameManager;

impl ClientHandler for GameManager {
    fn new(mngrs: &mut Managers) -> Self {
        let _ = mngrs.gr_mngr.load_image(&mngrs.rs_mngr, "load");
        Self
    }

    fn update(&mut self, mngrs: &mut Managers, _: Duration) -> bool {
        let instances = [
            BaseInstance {
                _world: Mat4::from_scale_rotation_translation(
                    Vec3::new(1200.0, 1200.0, 1.0),
                    Quat::IDENTITY,
                    Vec3::new(0.0, 0.0, 0.0),
                ),
                _tex_coord: Vec4::new(0.0, 0.0, 1.0, 1.0),
            },
            BaseInstance {
                _world: Mat4::from_scale_rotation_translation(
                    Vec3::new(100.0, 200.0, 1.0),
                    Quat::IDENTITY,
                    Vec3::new(100.0, 200.0, 0.0),
                ),
                _tex_coord: Vec4::new(0.0, 0.0, 1.0, 1.0),
            },
        ];
        mngrs.gr_mngr.update_instances(0, &instances);
        mngrs.gr_mngr.render(&[RenderCommand {
            image_id: "load",
            instances_range: 0..instances.len() as u32,
        }]);

        true
    }
}
