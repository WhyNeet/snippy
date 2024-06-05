use gpui::*;
use theme::*;

#[derive(IntoElement)]
pub struct CodeView {
    code: String,
}

impl CodeView {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

impl RenderOnce for CodeView {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .absolute()
            .inset_0()
            .bg(cx.theme().colors().panel_background)
            .rounded_lg()
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(
                div()
                    .w_full()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .px_4()
                    .py_1()
                    .child("code"),
            )
            .child(self.code)
    }
}
