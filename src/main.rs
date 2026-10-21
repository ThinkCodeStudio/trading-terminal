// src/main.rs
mod app;
mod data;
mod panels;
pub mod assets;

use app::TradingApp;
use gpui_kit::assets::AllAssets;
use gpui_kit::component::TitleBar;
use gpui_kit::{App, AppContext, Bounds, WindowBounds, WindowOptions, application, px, size};
use crate::assets::AppAssets;

fn main() {
    application()
        .with_assets(AllAssets)
        .with_assets(AppAssets)
        .run(|cx: &mut App| {
        gpui_kit::init(cx);

        let bounds = Bounds::centered(None, size(px(1440.0), px(900.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..TitleBar::window_options()
            },
            |window, cx| cx.new(|cx| TradingApp::new(window, cx)),
        )
            .expect("failed to open main window");
    });
}