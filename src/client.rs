use crate::engine::{graphic::pipeline::BaseInstance, ClientHandler, Managers};
use glam::{Mat4, Quat, Vec3, Vec4};
use std::time::Duration;

/// ゲームを管理するオブジェクト。
pub struct GameManager;

impl ClientHandler for GameManager {
    fn update(&mut self, mngrs: &Managers, _: Duration) -> bool {
        let instances = [
            BaseInstance {
                _world: Mat4::from_scale_rotation_translation(
                    Vec3::new(100.0, 100.0, 1.0),
                    Quat::IDENTITY,
                    Vec3::new(-100.0, 100.0, 0.0),
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
        mngrs.gr_mngr.render(&instances);

        true
    }
}
