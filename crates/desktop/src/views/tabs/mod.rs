use gpui::*;
use ui::*;

use crate::state::{
    tab::{SwitchTabEvent, Tab},
    State,
};

pub struct TabsView {
    active_tab: Tab,
}

impl TabsView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let tab_state = cx.global::<State>().tab_state.clone();
            let active_tab = tab_state.read(cx).get_current_tab();

            cx.observe(&tab_state, |this: &mut TabsView, model, cx| {
                this.active_tab = model.read(cx).get_current_tab();
                cx.notify();
            })
            .detach();

            TabsView { active_tab }
        })
    }
}

impl Render for TabsView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .rounded_lg()
            .flex()
            .w_72()
            .content_stretch()
            .h_8()
            .child(
                div()
                    .child("Share")
                    .w_full()
                    .bg(if self.active_tab == Tab::Share {
                        cx.theme().colors().tab_active_background
                    } else {
                        cx.theme().colors().tab_inactive_background
                    })
                    .flex()
                    .justify_center()
                    .items_center()
                    .rounded_l_lg()
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, |event, cx| {
                        State::update(
                            |model, cx| {
                                cx.update_model(&model.tab_state, |model, cx| {
                                    cx.emit(SwitchTabEvent(Tab::Share))
                                })
                            },
                            cx,
                        )
                    }),
            )
            .child(
                div()
                    .child("Receive")
                    .w_full()
                    .rounded_r_lg()
                    .bg(if self.active_tab == Tab::Receive {
                        cx.theme().colors().tab_active_background
                    } else {
                        cx.theme().colors().tab_inactive_background
                    })
                    .flex()
                    .justify_center()
                    .items_center()
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, |event, cx| {
                        State::update(
                            |model, cx| {
                                cx.update_model(&model.tab_state, |model, cx| {
                                    cx.emit(SwitchTabEvent(Tab::Receive))
                                })
                            },
                            cx,
                        )
                    }),
            )
    }
}
