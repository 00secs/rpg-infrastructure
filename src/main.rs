mod client;
mod engine;

type EError = Box<dyn std::error::Error>;

fn anything_to_u8slice<T>(a: &T) -> &[u8] {
    use std::{mem, slice};
    unsafe { slice::from_raw_parts((a as *const T).cast::<u8>(), mem::size_of::<T>()) }
}

fn slice_to_u8slice<T>(a: &[T]) -> &[u8] {
    use std::{mem, slice};
    unsafe { slice::from_raw_parts(a.as_ptr().cast::<u8>(), mem::size_of::<T>() * a.len()) }
}

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
