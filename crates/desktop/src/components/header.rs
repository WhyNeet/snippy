use gpui::*;

use crate::views::tabs::TabsView;

use super::logo::Logo;

pub struct HeaderView {
    tabs_view: View<TabsView>,
}

impl HeaderView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let tabs_view = TabsView::build(cx);

            Self { tabs_view }
        })
    }
}

impl Render for HeaderView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .h_16()
            .w_full()
            .flex()
            .items_center()
            .child(div().flex_1().child(Logo))
            .child(div().child(self.tabs_view.clone()))
            .child(div().flex_1())
    }
}
