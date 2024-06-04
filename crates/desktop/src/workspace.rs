use gpui::*;

use crate::{
    state::{tab::Tab, State},
    views::{receive::ReceiveView, share::ShareView, tabs::TabsView},
};

pub struct Workspace {
    share_view: View<ShareView>,
    receive_view: View<ReceiveView>,
    tabs_view: View<TabsView>,

    current_tab: Tab,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Workspace> {
        cx.new_view(|cx| {
            let tabs_view = TabsView::build(cx);
            let share_view = ShareView::build(cx);
            let receive_view = ReceiveView::build(cx);

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
                tabs_view,
                current_tab,
            }
        })
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut children: Vec<AnyView> = vec![self.tabs_view.clone().into()];

        if self.current_tab == Tab::Share {
            children.push(self.share_view.clone().into());
        } else {
            children.push(self.receive_view.clone().into());
        }

        div().text_color(rgb(0xfff)).children(children)
    }
}
