pub mod graphic;
pub mod input;
pub mod resource;

use std::{
    mem, slice,
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{application::*, dpi::*, event::*, event_loop::*, window::*};

type EError = Box<dyn std::error::Error>;

fn anything_to_u8slice<T>(a: &T) -> &[u8] {
    unsafe { slice::from_raw_parts((a as *const T).cast::<u8>(), mem::size_of::<T>()) }
}

fn slice_to_u8slice<T>(a: &[T]) -> &[u8] {
    unsafe { slice::from_raw_parts(a.as_ptr().cast::<u8>(), mem::size_of::<T>() * a.len()) }
}

/// アプリケーションの基本情報。
pub struct ApplicationInfo {
    pub title: &'static str,
    pub scene_width: u32,
    pub scene_height: u32,
    pub window_width: f32,
    pub window_height: f32,
    pub is_fullscreen: bool,
}

/// マネージャオブジェクトの集合。
///
/// クライアントはこのマネージャを叩いてゲームを表現する。
pub struct Managers<'a> {
    pub gr_mngr: graphic::GraphicManager<'a>,
    pub in_mngr: input::InputManager,
    pub rs_mngr: resource::ResourceManager,
}

/// クライアントが実装すべきトレイト。
pub trait ClientHandler {
    /// クライアントコンストラクタ。
    fn new(mngrs: &mut Managers) -> Self;
    /// クライアント更新メソッド。
    ///
    /// アプリケーションを続行する場合true、終了する場合falseを返す。
    fn update(&mut self, mngrs: &mut Managers, duration: Duration) -> bool;
}

/// アプリケーションのコアとなるオブジェクトの集合。
struct ApplicationCore<'a, T> {
    _window: Arc<Window>,
    mngrs: Managers<'a>,
    client: T,
}

/// winitベースウィンドウアプリケーションの構造体。
struct Application<'a, T>
where
    T: ClientHandler,
{
    info: ApplicationInfo,
    core: Option<ApplicationCore<'a, T>>,
    last: Instant,
}

impl<'a, T> ApplicationHandler for Application<'a, T>
where
    T: ClientHandler,
{
    /// アプリケーションが再開されたときに呼ばれるメソッド。
    ///
    /// ことWindows, macOS, Linuxにおいてはアプリケーション起動直後に一度だけ呼ばれる。
    /// そのため、アプリケーションに必要なすべてのオブジェクトを初期化する。
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.core.is_some() {
            return;
        }

        let primary_monitor = event_loop
            .primary_monitor()
            .expect("no primary monitor is found.");

        let fullscreen = if self.info.is_fullscreen {
            Some(Fullscreen::Borderless(Some(primary_monitor)))
        } else {
            None
        };

        let window_attributes = Window::default_attributes()
            .with_title(self.info.title)
            .with_resizable(false)
            .with_inner_size(LogicalSize::new(
                self.info.window_width,
                self.info.window_height,
            ))
            .with_fullscreen(fullscreen);
        let window = event_loop
            .create_window(window_attributes)
            .expect("failed to create a window.");
        window.set_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);

        let window = Arc::new(window);

        let gr_mngr = graphic::GraphicManager::new(
            window.clone(),
            self.info.scene_width,
            self.info.scene_height,
        )
        .expect("failed to create a graphic manager.");
        let in_mngr = input::InputManager::new();
        let rs_mngr = resource::ResourceManager::new();
        let mut mngrs = Managers {
            gr_mngr,
            in_mngr,
            rs_mngr,
        };

        let client = T::new(&mut mngrs);

        self.core = Some(ApplicationCore {
            _window: window,
            mngrs,
            client,
        });
    }

    /// ウィンドウイベントを処理するメソッド。
    ///
    /// - ウィンドウ破棄イベント -> アプリケーション終了
    /// - キーボード入力イベント -> InputManager
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Some(core) = &mut self.core else {
            return;
        };

        match event {
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                core.mngrs.in_mngr.on_key_event_happened(event);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Destroyed => event_loop.exit(),
            _ => (),
        }
    }

    /// デッドタイムに呼ばれるメソッド。
    ///
    /// つまり、アプリケーションのメインループ。
    /// クライアントの更新メソッドを呼ぶ。
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let Some(core) = &mut self.core else {
            return;
        };

        let duration = self.last.elapsed();
        self.last = Instant::now();

        #[cfg(debug_assertions)]
        {
            use std::io::Write;
            let mut stdout = std::io::stdout();
            write!(stdout, "\x1B[2J\x1B[H").unwrap();
            stdout.flush().unwrap();
            println!(
                "duration: {:.1} ms ({:.1} fps)",
                duration.as_secs_f32() * 1000.0,
                1.0 / duration.as_secs_f32(),
            );
        }

        if !core.client.update(&mut core.mngrs, duration) {
            event_loop.exit();
        }

        core.mngrs.in_mngr.go_next();
    }
}

/// winitベースウィンドウアプリケーションを実行する関数。
///
/// ウィンドウが閉じられるまでスレッドを待機する。
pub fn run<T>(info: ApplicationInfo) -> Result<(), EError>
where
    T: ClientHandler,
{
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut Application::<T> {
        info,
        core: None,
        last: Instant::now(),
    })?;
    Ok(())
}
