// src/panels/chart_panel.rs
use gpui_kit::{div, App, Context, EventEmitter, FocusHandle, Focusable, IntoElement, Render, Window, Styled};
use gpui_kit::component::dock::{BasePanel, Panel, PanelEvent};


pub struct ChartPanel {
    focus_handle: FocusHandle,
}

impl ChartPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self { focus_handle: cx.focus_handle() }
    }
}

impl EventEmitter<PanelEvent> for ChartPanel {}

impl Focusable for ChartPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl BasePanel for ChartPanel {
    fn panel_name(&self) -> &'static str {
        "ChartPanel"
    }
}

impl Panel for ChartPanel {
    fn title(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        "Chart"
    }
}

impl Render for ChartPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 这里放 CandlestickChart
        div().size_full()
    }
}