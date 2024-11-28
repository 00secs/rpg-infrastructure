pub mod graphic;

use crate::EError;
use std::{sync::Arc, time::Instant};
use winit::{application::*, dpi::*, event::*, event_loop::*, window::*};

/// アプリケーションの基本情報。
pub struct ApplicationInfo {
    pub title: &'static str,
    pub width: f32,
    pub height: f32,
    pub is_fullscreen: bool,
}

/// マネージャオブジェクトの集合。
///
/// クライアントはこのマネージャを叩いてゲームを表現する。
pub struct Managers<'a> {
    pub gr_mngr: graphic::GraphicManager<'a>,
}

/// winitベースウィンドウアプリケーションの構造体。
struct Application<'a> {
    info: ApplicationInfo,
    window: Option<Arc<Window>>,
    mngrs: Option<Managers<'a>>,
    last: Instant,
}

impl<'a> ApplicationHandler for Application<'a> {
    /// アプリケーションが再開されたときに呼ばれるメソッド。
    ///
    /// ことWindows, macOS, Linuxにおいてはアプリケーション起動直後に一度だけ呼ばれる。
    /// そのため、アプリケーションに必要なすべてのオブジェクトを初期化する。
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
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
            .with_inner_size(LogicalSize::new(self.info.width, self.info.height))
            .with_fullscreen(fullscreen);
        let window = event_loop
            .create_window(window_attributes)
            .expect("failed to create a window.");
        window.set_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);

        let window = Arc::new(window);

        let gr_mngr = graphic::GraphicManager::new(window.clone())
            .expect("failed to create a graphic manager.");
        let mngrs = Managers { gr_mngr };

        self.window = Some(window);
        self.mngrs = Some(mngrs);
    }

    /// ウィンドウイベントを処理するメソッド。
    ///
    /// - ウィンドウ破棄イベント -> アプリケーション終了
    /// - キーボード入力イベント -> InputManager
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        if self.window.is_none() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Destroyed => event_loop.exit(),
            _ => (),
        }
    }

    /// デッドタイムに呼ばれるメソッド。
    ///
    /// つまり、アプリケーションのメインループ。
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if self.window.is_none() {
            return;
        }

        println!("[ debug ] duration: {:?}", self.last.elapsed());
        // TODO: ゲーム更新
        self.last = Instant::now();

        self.mngrs.as_ref().unwrap().gr_mngr.render();
    }
}

/// winitベースウィンドウアプリケーションを実行する関数。
///
/// ウィンドウが閉じられるまでスレッドを待機する。
pub fn run(info: ApplicationInfo) -> Result<(), EError> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut Application {
        info,
        window: None,
        mngrs: None,
        last: Instant::now(),
    })?;
    Ok(())
}
