use super::*;

/// スプライトコンポーネント。
pub struct Sprite {
    uuid: Uuid,
    image_id: &'static str,
    scl: Vec2,
    rot: Quat,
    pos: Vec3,
    uv: Vec4,
    col: Vec4,
    param: Vec4,
    coords: CoordinateSystem,
    align: Alignment,
    should_push: bool,
}

impl Sprite {
    /// コンストラクタ。
    ///
    /// 初期設定として次が設定される：
    /// - サイズ：1x1
    /// - 回転：なし
    /// - 座標：(0, 0, 0)
    /// - UV座標：([0,1], [0,1])
    /// - 色：白
    /// - UIか：いいえ
    /// - 座標系：ワールド座標系
    /// - アラインメント：中央
    pub fn new(image_id: &'static str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            image_id,
            scl: Vec2::new(1.0, 1.0),
            rot: Quat::IDENTITY,
            pos: Vec3::ZERO,
            uv: Vec4::new(0.0, 0.0, 1.0, 1.0),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            param: Vec4::ZERO,
            coords: CoordinateSystem::World,
            align: Alignment::Center,
            should_push: true,
        }
    }
    pub fn get_scl(&self) -> Vec2 {
        self.scl
    }
    pub fn get_rot(&self) -> Quat {
        self.rot
    }
    pub fn get_pos(&self) -> Vec3 {
        self.pos
    }
    pub fn get_uv(&self) -> Vec4 {
        self.uv
    }
    pub fn get_col(&self) -> Vec4 {
        self.col
    }
    pub fn get_param(&self) -> Vec4 {
        self.param
    }
    pub fn get_coords(&self) -> CoordinateSystem {
        self.coords.clone()
    }
    pub fn get_align(&self) -> Alignment {
        self.align.clone()
    }
    pub fn set_scl(&mut self, scl: Vec2) {
        self.scl = scl;
        self.should_push = true;
    }
    pub fn set_rot(&mut self, rot: Quat) {
        self.rot = rot;
        self.should_push = true;
    }
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.should_push = true;
    }
    pub fn set_uv(&mut self, uv: Vec4) {
        self.uv = uv;
        self.should_push = true;
    }
    pub fn set_col(&mut self, col: Vec4) {
        self.col = col;
        self.should_push = true;
    }
    pub fn set_is_ui(&mut self, is_ui: bool) {
        if is_ui {
            self.param.x = 1.0;
        } else {
            self.param.x = 0.0;
        }
        self.should_push = true;
    }
    pub fn set_coods(&mut self, coords: CoordinateSystem) {
        self.coords = coords;
        self.should_push = true;
    }
    pub fn set_align(&mut self, align: Alignment) {
        self.align = align;
        self.should_push = true;
    }
    pub fn with_scl(mut self, scl: Vec2) -> Self {
        self.set_scl(scl);
        self
    }
    pub fn with_rot(mut self, rot: Quat) -> Self {
        self.set_rot(rot);
        self
    }
    pub fn with_pos(mut self, pos: Vec3) -> Self {
        self.set_pos(pos);
        self
    }
    pub fn with_uv(mut self, uv: Vec4) -> Self {
        self.set_uv(uv);
        self
    }
    pub fn with_col(mut self, col: Vec4) -> Self {
        self.set_col(col);
        self
    }
    pub fn with_is_ui(mut self, is_ui: bool) -> Self {
        self.set_is_ui(is_ui);
        self
    }
    pub fn with_coods(mut self, coords: CoordinateSystem) -> Self {
        self.set_coods(coords);
        self
    }
    pub fn with_align(mut self, align: Alignment) -> Self {
        self.set_align(align);
        self
    }
    pub fn push_to(&mut self, instances: &mut Vec<InstanceMeta>) {
        let pos = match self.coords {
            CoordinateSystem::World => self.pos,
            CoordinateSystem::Canvas => Vec3::new(
                -SCENE_WIDTH / 2.0 + self.pos.x,
                SCENE_HEIGHT / 2.0 - self.pos.y,
                self.pos.z,
            ),
        };

        let pos = match self.align {
            Alignment::Center => pos,
            Alignment::TopLeft => {
                Vec3::new(pos.x + self.scl.x / 2.0, pos.y - self.scl.y / 2.0, pos.z)
            }
        };

        instances.push(InstanceMeta {
            instance: BaseInstance {
                _world: Mat4::from_scale_rotation_translation(
                    Vec3::new(self.scl.x, self.scl.y, 1.0),
                    self.rot,
                    pos,
                ),
                _uv: self.uv,
                _color: self.col,
                _param: self.param,
            },
            uuid: self.uuid,
            updated: self.should_push,
            image_id: self.image_id,
            depth: pos.z,
        });

        self.should_push = false;
    }
}
