// src/assets.rs
use std::borrow::Cow;
use gpui_kit::{
    AssetSource, Result, SharedString,
};

pub struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path == "icons/app-icon.svg" {
            return Ok(Some(Cow::Borrowed(include_bytes!(
                "../assets/icons/app-icon.svg"
            ))));
        }
        // 回退到 gpui-kit 默认资产
        gpui_kit::assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        // 直接委托给默认资产源即可
        gpui_kit::assets::Assets.list(path)
    }
}