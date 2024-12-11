use super::*;

/// マップシーンで使われるコンポーネントの集合体。
pub struct Components {
    pub map_tiles: MapTiles,
    pub player: Actor,
    pub actors: Vec<Actor>,
    pub message_box: Option<MessageBox>,
}

impl Components {
    pub fn push_to(&mut self, instances: &mut Vec<InstanceMeta>) {
        self.map_tiles.push_to(instances);
        self.player.push_to(instances);
        for n in &mut self.actors {
            n.push_to(instances);
        }
        if let Some(n) = &mut self.message_box {
            n.push_to(instances);
        }
    }
}
