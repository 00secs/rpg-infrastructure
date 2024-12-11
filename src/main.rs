mod client;
mod engine;

const SCENE_WIDTH: f32 = 1280.0;
const SCENE_HEIGHT: f32 = 720.0;

fn main() {
    engine::run::<client::GameManager>(engine::ApplicationInfo {
        title: "タイトル",
        width: SCENE_WIDTH,
        height: SCENE_HEIGHT,
        is_fullscreen: false,
    })
    .unwrap();
}
