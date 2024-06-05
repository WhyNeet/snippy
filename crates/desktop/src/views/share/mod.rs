use gpui::*;
use theme::*;
use ui::*;

pub struct ShareView;

impl ShareView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| ShareView)
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
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event, cx| {
                            println!(
                                "use clipboard:\n{:?}",
                                cx.read_from_clipboard()
                                    .map(|item| item.text().clone())
                                    .unwrap_or("no content".to_string())
                            );
                        }),
                    )
                    .child("Use clipboard"),
            )
    }
}
