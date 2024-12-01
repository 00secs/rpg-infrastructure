use std::collections::HashMap;
use winit::{
    event::KeyEvent,
    keyboard::{KeyCode, PhysicalKey},
};

/// キーボードの入力状態を管理するオブジェクト。
/// 
/// 入力状態は次の通り：
/// - 0: 押されていない
/// - n: nフレーム押されている
pub struct InputManager {
    /// キーの入力状態を管理するマップ。
    /// 押されているキーのみ存在する。
    states: HashMap<KeyCode, u32>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    pub fn get(&self, key_code: &KeyCode) -> u32 {
        *self.states.get(key_code).unwrap_or(&0)
    }

    pub fn on_key_event_happened(&mut self, event: KeyEvent) {
        let key_code = match event.physical_key {
            PhysicalKey::Code(n) => n,
            _ => return,
        };
        if event.state.is_pressed() {
            if !self.states.contains_key(&key_code) {
                self.states.insert(key_code, 1);
            }
        } else {
            self.states.remove(&key_code);
        }
    }

    pub fn go_next(&mut self) {
        for n in self.states.values_mut() {
            *n += 1;
        }
    }
}
