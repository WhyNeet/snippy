use gpui::*;

pub struct ReceiveView;

impl ReceiveView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| ReceiveView)
    }
}

impl Render for ReceiveView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child("ReceiveView")
    }
}
