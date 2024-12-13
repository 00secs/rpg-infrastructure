use super::*;

use std::collections::HashSet;

/// マップシーンで使われるコンポーネントの集合体。
pub struct Components {
    pub camera: Camera,
    pub map_tiles: MapTiles,
    pub player: Actor,
    pub actors: Vec<Actor>,
    pub message_box: Option<MessageBox>,
}

impl Components {
    pub fn collect_characters(&self, chars: &mut HashSet<(&'static str, char)>) {
        if let Some(n) = &self.message_box {
            n.collect_characters(chars);
        }
    }

    pub fn push_to(
        &mut self,
        instances: &mut Vec<InstanceMeta>,
        mngrs: &Managers,
        should_push_text: bool,
    ) {
        self.map_tiles.push_to(instances);
        self.player.push_to(instances);
        for n in &mut self.actors {
            n.push_to(instances);
        }
        if let Some(n) = &mut self.message_box {
            n.push_to(instances, mngrs, should_push_text);
        }
    }
}
