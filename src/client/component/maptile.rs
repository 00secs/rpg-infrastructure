use super::*;

/// マップを構成するタイルチップ1個。
pub struct MapTile {
    sprite: Sprite,
    passable: bool,
}

impl MapTile {
    /// コンストラクタ。
    ///
    /// * i - マップ上の行番号
    /// * j - マップ上の列番号
    /// * uv - UV座標
    /// * passable - 進入可能か否か
    pub fn new(i: usize, j: usize, image_id: &'static str, uv: Vec4, passable: bool) -> Self {
        let sprite = Sprite::new(image_id)
            .with_scl(Vec2::new(MAPTILE_SIZE, MAPTILE_SIZE))
            .with_pos(Vec3::new(
                MAPTILE_SIZE * j as f32,
                -MAPTILE_SIZE * i as f32,
                95.0,
            ))
            .with_uv(uv);
        Self { sprite, passable }
    }

    /// 進入可能か否かを取得するメソッド。
    pub fn is_passable(&self) -> bool {
        self.passable
    }

    /// インスタンスバッファ更新用のデータをinstancesに追加するメソッド。
    pub fn push_to(&mut self, instances: &mut Vec<InstanceMeta>) {
        self.sprite.push_to(instances);
    }
}

/// マップタイルを二次元Vecで管理するヘルパーオブジェクト。
///
/// WARN: 必ずtilesは長方形となっていること。
pub struct MapTiles {
    pub tiles: Vec<Vec<MapTile>>,
}

impl MapTiles {
    pub fn get(&self, i: usize, j: usize) -> Option<&MapTile> {
        self.tiles.get(i)?.get(j)
    }

    pub fn push_to(&mut self, instances: &mut Vec<InstanceMeta>) {
        for n in &mut self.tiles {
            for n in n {
                n.push_to(instances);
            }
        }
    }
}
