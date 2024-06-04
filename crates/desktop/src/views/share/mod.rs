use gpui::*;

pub struct ShareView;

impl ShareView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| ShareView)
    }
}

impl Render for ShareView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child("ShareView")
    }
}
