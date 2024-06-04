pub mod tab;

use gpui::*;
use tab::TabState;

#[derive(Clone)]
pub struct State {
    pub tab_state: Model<TabState>,
}

impl State {
    pub fn init(cx: &mut WindowContext) {
        let tab_state = TabState::build(cx);

        cx.subscribe(&tab_state, |tab_state, event, cx| {
            tab_state.update(cx, |model, cx| {
                model.set_current_tab(event.0);
                cx.notify();
            });
        })
        .detach();

        let this = State { tab_state };

        cx.set_global(this);
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut WindowContext), cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|mut this, cx| {
            f(&mut this, cx);
        });
    }
}

impl Global for State {}
