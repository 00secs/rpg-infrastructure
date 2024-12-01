mod component;
mod scene;

use crate::engine::{ClientHandler, Managers};
use std::time::Duration;

/// ゲームを管理するオブジェクト。
///
/// エンジンに対するクライアント。
/// ClientHandlerを実装することでエンジンのライフサイクルに組み込める。
pub struct GameManager {
    scene: scene::Scene,
}

impl ClientHandler for GameManager {
    fn new(mngrs: &mut Managers) -> Self {
        let scene = scene::load::LoadScene::new(mngrs);
        Self { scene }
    }

    fn update(&mut self, mngrs: &mut Managers, duration: Duration) -> bool {
        if let Some(next) = self.scene.update(mngrs, duration) {
            self.scene = next;
        }

        // TODO: どうやって終了を検知しようか。
        //       どうせグローバルデータ領域を作るので、そこに終了フラグを設けるか。
        true
    }
}
