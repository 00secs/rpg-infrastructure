mod engine;

type EError = Box<dyn std::error::Error>;

fn main() {
    const SCENE_WIDTH: u32 = 1280;
    const SCENE_HEIGHT: u32 = 720;

    engine::run(engine::ApplicationInfo {
        title: "タイトル",
        width: SCENE_WIDTH as f32,
        height: SCENE_HEIGHT as f32,
        is_fullscreen: false,
    })
    .unwrap();
}
