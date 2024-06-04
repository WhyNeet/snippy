use gpui::*;

use crate::state::{
    tab::{SwitchTabEvent, Tab},
    State,
};

pub struct TabsView;

impl TabsView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| TabsView)
    }
}

impl Render for TabsView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .child(
                div()
                    .child("share")
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
                    .child("receive")
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
