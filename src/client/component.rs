// NOTE: 定義するだけ定義しておいて使わなくても良いので。
#![allow(dead_code)]

mod actor;
mod camera;
mod components;
mod maptile;
mod message;
mod sprite;
mod text;

pub use actor::*;
pub use camera::*;
pub use components::*;
pub use maptile::*;
pub use message::*;
pub use sprite::*;
pub use text::*;

use crate::{
    engine::{
        graphic::{pipeline::*, *},
        Managers,
    },
    SCENE_HEIGHT, SCENE_WIDTH,
};
use glam::*;
use std::time::Duration;
use uuid::Uuid;

/// マップタイル1個(正方形)の1辺のサイズ [px]。
pub const MAPTILE_SIZE: f32 = 48.0;

/// イベントの型。
///
/// イベントを維持する場合true、破棄する場合falseを返す。
pub type Event = fn(&mut Managers, &mut Components, Duration) -> bool;

/// 座標系を指定する列挙型。
#[derive(Clone, PartialEq, Eq)]
pub enum CoordinateSystem {
    /// ワールド座標系。
    /// - 画面中央原点
    /// - 左から右へ [-SCENE_WIDTH/2, SCENE_WIDTH/2]
    /// - 下から上へ [-SCENE_HEIGHT/2, SCENE_HEIGHT/2]
    World,
    /// キャンバス座標系。
    /// - 画面左上原点
    /// - 左から右へ [0, SCENE_WIDTH]
    /// - 上から下へ [0, SCENE_HEIGHT]
    Canvas,
}

/// アラインメント(コンポーネントの原点の位置)を指定する列挙型。
#[derive(Clone, PartialEq, Eq)]
pub enum Alignment {
    Center,
    TopLeft,
}
