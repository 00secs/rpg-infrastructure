use super::*;

use std::time::Duration;

/// アニメーション時のUV座標を指定する関数のポインタの型。
///
/// * ActorDirection - アクターの向き
/// * f32 - アニメーションの進行度 [0.0-1.0]
pub type UVFunc = fn(ActorDirection, f32) -> Vec4;

/// アクターの向きを示す列挙型。
#[derive(Clone)]
pub enum ActorDirection {
    Left,
    Right,
    Up,
    Down,
}

/// アクターの状態を示す列挙型。
pub enum State {
    Idle(ActorDirection),
    Moving((ActorDirection, f32)),
}

/// アクター。
pub struct Actor {
    sprite: Sprite,
    i: usize,
    j: usize,
    z: f32,
    speed: f32,
    idle_uv: UVFunc,
    moving_uv: UVFunc,
    state: State,
}

// TODO: Z座標
impl Actor {
    /// コンストラクタ。
    ///
    /// * i - マップ上の行番号
    /// * j - マップ上の列番号
    /// * z - 深度値
    /// * speed - 1秒間における移動スピード [px]
    /// * idle_uv - アイドルアニメーションのUV座標を指定する関数のポインタ
    /// * idle_uv - 移動アニメーションのUV座標を指定する関数のポインタ
    pub fn new(
        i: usize,
        j: usize,
        z: f32,
        speed: f32,
        image_id: &'static str,
        idle_uv: UVFunc,
        moving_uv: UVFunc,
    ) -> Self {
        let sprite = Sprite::new(image_id)
            .with_scl(Vec2::new(MAPTILE_SIZE, MAPTILE_SIZE))
            .with_pos(Vec3::new(
                MAPTILE_SIZE * j as f32,
                -MAPTILE_SIZE * i as f32,
                z,
            ))
            .with_uv((idle_uv)(ActorDirection::Down, 0.0));
        Self {
            sprite,
            i,
            j,
            z,
            speed,
            idle_uv,
            moving_uv,
            state: State::Idle(ActorDirection::Down),
        }
    }

    /// 移動開始可能な状態か否か取得するメソッド。
    pub fn can_start_move(&self) -> bool {
        match self.state {
            State::Idle(_) => true,
            _ => false,
        }
    }

    /// マップ上での座標(行番号, 列番号)を取得するメソッド。
    pub fn get_ij(&self) -> (usize, usize) {
        (self.i, self.j)
    }

    /// アクターの座標(Vec3)を取得するメソッド。
    pub fn get_position(&self) -> Vec3 {
        self.sprite.get_pos()
    }

    /// アイドル状態の向きを変更するメソッド。
    pub fn change_direction(&mut self, direction: ActorDirection) {
        self.state = State::Idle(direction.clone());
        self.sprite.set_uv((self.idle_uv)(direction, 0.0));
    }

    /// 指定した向きへ移動を開始するメソッド。
    pub fn start_move(&mut self, direction: ActorDirection) {
        self.state = State::Moving((direction.clone(), 0.0));
        self.sprite.set_uv((self.moving_uv)(direction, 0.0));
    }

    /// 移動を行うメソッド。
    pub fn update(&mut self, duration: Duration) {
        match &self.state {
            State::Moving((direction, offset)) => {
                // オフセットを更新
                let diff = self.speed * duration.as_secs_f32();
                let offset = offset + diff;
                // まだ移動中？
                if offset < MAPTILE_SIZE {
                    let pos = self.sprite.get_pos();
                    let pos = match direction {
                        ActorDirection::Left => pos.with_x(pos.x - diff),
                        ActorDirection::Right => pos.with_x(pos.x + diff),
                        ActorDirection::Up => pos.with_y(pos.y + diff),
                        ActorDirection::Down => pos.with_y(pos.y - diff),
                    };
                    self.sprite.set_pos(pos);
                    self.sprite
                        .set_uv((self.moving_uv)(direction.clone(), offset / MAPTILE_SIZE));
                    self.state = State::Moving((direction.clone(), offset));
                }
                // 移動終了？
                else {
                    match direction {
                        ActorDirection::Left => self.j -= 1,
                        ActorDirection::Right => self.j += 1,
                        ActorDirection::Up => self.i -= 1,
                        ActorDirection::Down => self.i += 1,
                    }
                    self.sprite.set_pos(Vec3::new(
                        MAPTILE_SIZE * self.j as f32,
                        -MAPTILE_SIZE * self.i as f32,
                        self.get_position().z,
                    ));
                    self.change_direction(direction.clone());
                }
            }
            _ => (),
        }
    }

    pub fn push_to(&mut self, instances: &mut Vec<InstanceMeta>) {
        self.sprite.push_to(instances);
    }
}
