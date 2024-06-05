use gpui::*;
use theme::*;

use crate::{
    components::header::HeaderView,
    state::{tab::Tab, State},
    views::{receive::ReceiveView, share::ShareView, tabs::TabsView},
};

pub struct Workspace {
    share_view: View<ShareView>,
    receive_view: View<ReceiveView>,
    header_view: View<HeaderView>,

    current_tab: Tab,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Workspace> {
        cx.new_view(|cx| {
            let share_view = ShareView::build(cx);
            let receive_view = ReceiveView::build(cx);
            let header_view = HeaderView::build(cx);

            let state = cx.global::<State>().clone();
            let current_tab = state.tab_state.read(cx).get_current_tab();

            cx.observe(&state.tab_state, |this: &mut Workspace, model, cx| {
                this.current_tab = model.read(cx).get_current_tab();
                cx.notify();
            })
            .detach();

            Workspace {
                share_view,
                receive_view,
                current_tab,
                header_view,
            }
        })
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let tab: AnyView = if self.current_tab == Tab::Share {
            self.share_view.clone().into()
        } else {
            self.receive_view.clone().into()
        };

        div()
            .w_full()
            .h_full()
            .bg(cx.theme().colors().background)
            .p_8()
            .pt_0()
            .flex()
            .flex_col()
            .items_center()
            .text_color(cx.theme().colors().text)
            .child(self.header_view.clone())
            .child(tab)
    }
}
