mod client;
mod engine;

const SCENE_WIDTH: f32 = 1280.0;
const SCENE_HEIGHT: f32 = 720.0;

fn main() {
    engine::run::<client::GameManager>(engine::ApplicationInfo {
        title: "タイトル",
        scene_width: SCENE_WIDTH as u32,
        scene_height: SCENE_HEIGHT as u32,
        window_width: SCENE_WIDTH,
        window_height: SCENE_HEIGHT,
        is_fullscreen: false,
    })
    .unwrap();
}
