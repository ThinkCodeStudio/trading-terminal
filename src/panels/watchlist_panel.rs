// src/panels/watchlist_panel.rs
use gpui_kit::component::dock::{BasePanel, Panel, PanelEvent};
use gpui_kit::{App, Context, EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled, Window};
use gpui_kit::component::{h_flex, v_flex};
use crate::data::model::Ticker;

pub struct WatchlistPanel {
    focus_handle: FocusHandle,
    pub(crate) tickers: Vec<Ticker>,
}

impl WatchlistPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            tickers: Vec::new(),
        }
    }
}

impl EventEmitter<PanelEvent> for WatchlistPanel {}

impl Focusable for WatchlistPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl BasePanel for WatchlistPanel {
    fn panel_name(&self) -> &'static str {
        "WatchlistPanel"
    }
}

impl Panel for WatchlistPanel {
    fn title(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        "Watchlist"
    }
}

impl Render for WatchlistPanel {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_1()
            .p_2()
            .children(self.tickers.iter().map(|t| {
                h_flex()
                    .justify_between()
                    .child(t.symbol.clone())
                    .child(format!("{:.2}  {:+.2}%", t.last, t.change_pct))
            }))
    }
}