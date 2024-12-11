use super::*;

use crate::client::component::*;
use glam::*;
use winit::keyboard::KeyCode;

pub struct MapScene {
    camera: Camera,
    coms: Components,
    events: Vec<Event>,
}

impl MapScene {
    pub fn new(mngrs: &mut Managers) -> Self {
        let _ = mngrs.gr_mngr.load_image(&mut mngrs.rs_mngr, "tiles");
        let _ = mngrs.gr_mngr.load_image(&mut mngrs.rs_mngr, "actors");
        let _ = mngrs.gr_mngr.load_image(&mut mngrs.rs_mngr, "uis");

        let camera = Camera::new();

        let mut tiles = Vec::new();
        for i in 0..10 {
            let mut v = Vec::new();
            for j in 0..10 {
                let tile = MapTile::new(i, j, "tiles", Vec4::new(0.0, 0.0, 1.0, 1.0), true);
                v.push(tile);
            }
            tiles.push(v);
        }
        let map_tiles = MapTiles { tiles };
        let player = Actor::new(
            0,
            0,
            80.0,
            240.0,
            "actors",
            player_idle_uv,
            player_moving_uv,
        );

        let coms = Components {
            map_tiles,
            player,
            actors: Vec::new(),
            message_box: None,
        };

        let mut events: Vec<Event> = Vec::new();
        events.push(message_event);
        events.push(move_player_event);

        Self {
            camera,
            coms,
            events,
        }
    }
}

impl SceneTrait for MapScene {
    fn update(&mut self, mngrs: &mut Managers, duration: Duration) -> NextScene {
        // イベントを実行
        let mut events = Vec::new();
        for event in &self.events {
            if (event)(mngrs, &mut self.coms, duration) {
                events.push(*event);
            }
        }
        self.events = events;

        // カメラバッファを更新
        self.camera.chase(self.coms.player.get_position());
        mngrs.gr_mngr.update_camera(&self.camera.buffer);

        // 描画
        let mut instances = Vec::new();
        self.coms.push_to(&mut instances);
        mngrs.gr_mngr.render_with_metas(instances);

        // 終了
        None
    }
}

// DEBUG:
fn message_event(mngrs: &mut Managers, coms: &mut Components, _: Duration) -> bool {
    if coms.message_box.is_none() {
        let mut message_box = MessageBox::new("uis", Vec4::new(0.0, 0.0, 1.0, 1.0));
        message_box.set_message(
            mngrs,
            "UtsukushiFONT.otf",
            "メッセージです".to_string(),
            24.0,
        );
        coms.message_box = Some(message_box);
    }

    if mngrs.in_mngr.get(&KeyCode::KeyZ) == 1 {
        coms.message_box = None;
        return false;
    }

    return true;
}

// TODO: 引っ越す？
fn move_player_event(mngrs: &mut Managers, coms: &mut Components, duration: Duration) -> bool {
    // プレイヤーが移動不可であれば早期リターン
    if coms.message_box.is_some() {
        return true;
    }

    // プレイヤーが移動開始不可であれば移動して早期リターン
    if !coms.player.can_start_move() {
        coms.player.update(duration);
        return true;
    }

    // 新しく入力された方向を取得
    const DIRECTIONS: [(KeyCode, (i32, i32, ActorDirection)); 4] = [
        (KeyCode::ArrowLeft, (0, -1, ActorDirection::Left)),
        (KeyCode::ArrowRight, (0, 1, ActorDirection::Right)),
        (KeyCode::ArrowUp, (-1, 0, ActorDirection::Up)),
        (KeyCode::ArrowDown, (1, 0, ActorDirection::Down)),
    ];
    let mut recent_input = None;
    let mut min_state = std::u32::MAX;
    for (kc, n) in &DIRECTIONS {
        let state = mngrs.in_mngr.get(kc);
        if state > 0 && state < min_state {
            recent_input = Some(n.clone());
            min_state = state;
        }
    }

    // 新しく入力された方向がないなら移動して早期リターン
    let Some((di, dj, direction)) = recent_input else {
        coms.player.update(duration);
        return true;
    };

    // 取り敢えず向きを変更
    coms.player.change_direction(direction.clone());

    // 旧座標から新座標取得
    let (oi, oj) = coms.player.get_ij();
    let (ni, nj) = (oi as i32 + di, oj as i32 + dj);

    // 移動先が進入可能ならば移動開始
    // TODO: アクター同士がぶつからないようにする。
    if ni >= 0 && nj >= 0 {
        if let Some(tile) = coms.map_tiles.get(ni as usize, nj as usize) {
            if tile.is_passable() {
                coms.player.start_move(direction);
            }
        }
    }

    // 移動
    coms.player.update(duration);

    return true;
}

// TODO: 引っ越す
fn player_idle_uv(_: ActorDirection, _: f32) -> Vec4 {
    Vec4::new(0.0, 0.0, 1.0, 1.0)
}
// TODO: 引っ越す
fn player_moving_uv(_: ActorDirection, _: f32) -> Vec4 {
    Vec4::new(0.0, 0.0, 1.0, 1.0)
}
