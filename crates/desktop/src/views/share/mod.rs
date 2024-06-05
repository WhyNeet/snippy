use gpui::*;
use theme::*;
use ui::FluentBuilder;

use crate::{
    components::code_view::CodeView,
    state::{code::SharedCodeChangeEvent, State},
};

pub struct ShareView {
    code: Option<String>,
}

impl ShareView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let state = cx.global::<State>();
            let share_code_state = state.share_code_state.clone();

            cx.observe(&share_code_state, |this: &mut Self, model, cx| {
                this.code = model.read(cx).get_code().map(|s| s.clone());
                cx.notify();
            })
            .detach();

            ShareView { code: None }
        })
    }
}

impl Render for ShareView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .h_full()
            .w_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w_72()
                    .py_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_lg()
                    .bg(cx.theme().colors().element_background)
                    .hover(|style| style.bg(cx.theme().colors().element_hover))
                    .on_mouse_down(MouseButton::Left, |event, cx| {
                        State::update(
                            |model, cx| {
                                let event = SharedCodeChangeEvent::from_clipboard(cx);
                                cx.update_model(&model.share_code_state, |model, cx| {
                                    cx.emit(event);
                                })
                            },
                            cx,
                        )
                    })
                    .child("Paste from clipboard"),
            )
            .when(self.code.is_some(), |this| {
                this.child(CodeView::new(self.code.clone().unwrap()))
            })
    }
}
