use super::*;

/// カメラ。
///
/// width:  [-SCENE_WIDTH/2, SCENE_WIDTH/2]
/// height: [-SCENE_HEIGHT/2, SCENE_HEIGHT/2] (上向き正)
/// depth:  [0, 100]
///
/// 非描画コンポーネント。
pub struct Camera {
    buffer: BaseCamera,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            buffer: BaseCamera {
                _projection: Mat4::orthographic_lh(
                    -SCENE_WIDTH / 2.0,
                    SCENE_WIDTH / 2.0,
                    -SCENE_HEIGHT / 2.0,
                    SCENE_HEIGHT / 2.0,
                    0.0,
                    100.0,
                ),
                _view: Mat4::IDENTITY,
            },
        }
    }

    pub fn get(&self) -> BaseCamera {
        self.buffer.clone()
    }

    /// 座標posにある物体の真上に移動するメソッド。
    ///
    /// WARN: posのz座標は無視される。
    pub fn chase(&mut self, mut pos: Vec3) {
        pos.x *= -1.0;
        pos.y *= -1.0;
        pos.z = 0.0;
        self.buffer._view = Mat4::from_translation(pos);
    }
}
