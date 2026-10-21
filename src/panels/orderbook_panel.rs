// src/panels/orderbook_panel.rs
use crate::data::model::OrderBookSnapshot;
use gpui_kit::component::dock::{BasePanel, Panel, PanelEvent};
use gpui_kit::component::{ActiveTheme, h_flex, v_flex};
use gpui_kit::{
    App, Context, EventEmitter, FocusHandle, Focusable, Hsla, IntoElement, ParentElement, Render,
    Styled, Window, div,
};

pub struct OrderBookPanel {
    focus_handle: FocusHandle,
    pub(crate) book: OrderBookSnapshot,
}

impl OrderBookPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            book: OrderBookSnapshot {
                bids: Vec::new(),
                asks: Vec::new(),
            },
        }
    }
}

impl EventEmitter<PanelEvent> for OrderBookPanel {}

impl Focusable for OrderBookPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl BasePanel for OrderBookPanel {
    fn panel_name(&self) -> &'static str {
        "OrderBookPanel"
    }
}

impl Panel for OrderBookPanel {
    fn title(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        "Order Book"
    }
}

impl Render for OrderBookPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ask_color = cx.theme().danger;
        let bid_color = cx.theme().success;

        let asks = self
            .book
            .asks
            .iter()
            .rev()
            .map(|l| book_row(l.price, l.size, ask_color));
        let bids = self
            .book
            .bids
            .iter()
            .map(|l| book_row(l.price, l.size, bid_color));

        v_flex()
            .size_full()
            .gap_1()
            .p_2()
            .children(asks)
            .children(bids)
    }
}

fn book_row(price: f64, size: f64, color: Hsla) -> impl IntoElement {
    h_flex()
        .justify_between()
        .child(div().text_color(color).child(format!("{price:.2}")))
        .child(div().text_color(color).child(format!("{size:.4}")))
}
