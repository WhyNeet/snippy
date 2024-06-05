use gpui::*;
use theme::*;

#[derive(IntoElement)]
pub struct Logo;

impl RenderOnce for Logo {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .child("snippy")
            .text_color(cx.theme().colors().text_accent)
    }
}
