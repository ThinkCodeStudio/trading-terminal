// src/app.rs
use std::rc::Rc;
use gpui_kit::component::dock::{DockArea, DockSkin, DockLayout, panel_handle};
use gpui_kit::{px, AppContext, Context, Entity, Window, Render, div, IntoElement, Styled, ParentElement, img};
use gpui_kit::component::{v_flex, TitleBar, h_flex};
use crate::data::model::Ticker;
use crate::panels::chart_panel::ChartPanel;
use crate::panels::orderbook_panel::OrderBookPanel;
use crate::panels::watchlist_panel::WatchlistPanel;

pub struct TradingApp {
    dock_area: Entity<DockArea>,
    dock_skin: Rc<DockSkin>,
    watchlist: Entity<WatchlistPanel>,
    chart: Entity<ChartPanel>,
    order_book: Entity<OrderBookPanel>,
}

impl TradingApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (dock_area, dock_skin) =
            DockSkin::dock_area("main-dock", Some(1), window, cx);

        let watchlist = cx.new(|cx| WatchlistPanel::new(cx));
        let chart = cx.new(|cx| ChartPanel::new(cx));
        let order_book = cx.new(|cx| OrderBookPanel::new(cx));

        let layout = DockLayout::h_split()
            .child(
                DockLayout::tabs().panel_view(panel_handle(watchlist.clone()), cx),
                Some(px(280.)),
            )
            .child(
                DockLayout::v_split()
                    .child(
                        DockLayout::tabs().panel_view(panel_handle(chart.clone()), cx),
                        Some(px(520.)),
                    )
                    .child(
                        DockLayout::tabs().panel_view(panel_handle(order_book.clone()), cx),
                        None,
                    ),
                None,
            );

        dock_area.update(cx, |area, cx| {
            area.set_center(layout, window, cx);
        });

        Self { dock_area, dock_skin, watchlist, chart, order_book }
    }

    /// 行情推送入口：feed 收到数据后调用此方法即可驱动 UI 重绘
    pub fn push_ticker(&mut self, t: Ticker, cx: &mut Context<Self>) {
        self.watchlist.update(cx, |panel, cx| {
            panel.tickers.push(t);
            cx.notify();
        });
    }
}

impl Render for TradingApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(
                TitleBar::new().child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            img("icons/app-icon.svg")
                                .w(px(18.))
                                .h(px(18.))
                        )
                )
            )
            .child(self.dock_area.clone()) // 下方是原有的 Dock 工作区
    }
}