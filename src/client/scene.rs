pub mod load;
pub mod map;
pub mod title;

use crate::engine::Managers;
use std::time::Duration;

pub type Scene = Box<dyn SceneTrait>;
pub type NextScene = Option<Scene>;

pub trait SceneTrait {
    fn update(&mut self, mngrs: &mut Managers, duration: Duration) -> NextScene;
}
