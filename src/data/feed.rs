// src/data/feed.rs
use std::time::Duration;
use gpui_kit::AsyncApp;
use super::model::*;

/// 启动模拟行情流。真实接入时，把定时器替换为交易所 WebSocket / REST 轮询即可。
pub fn start_mock_feed(cx: &mut AsyncApp, update: impl Fn(&mut AsyncApp) + 'static + Clone) {
    cx.spawn(async move |cx| {
        loop {
            cx.background_executor().timer(Duration::from_millis(500)).await;
            update(cx);
        }
    })
        .detach();
}