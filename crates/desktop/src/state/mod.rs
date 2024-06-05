pub mod code;
pub mod tab;

use code::{ShareCodeState, SharedCodeChangeEvent};
use gpui::*;
use tab::TabState;

#[derive(Clone)]
pub struct State {
    pub tab_state: Model<TabState>,
    pub share_code_state: Model<ShareCodeState>,
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

        let share_code_state = ShareCodeState::build(cx);

        cx.subscribe(
            &share_code_state,
            |share_code_state: Model<ShareCodeState>, event: &SharedCodeChangeEvent, cx| {
                share_code_state.update(cx, move |model, cx| {
                    let value = unsafe {
                        let event =
                            event as *const SharedCodeChangeEvent as *mut SharedCodeChangeEvent;

                        &mut (*event).0
                    };

                    model.set_code(value.take().map(|c| Some(c)).unwrap_or(None));
                    cx.notify();
                });
            },
        )
        .detach();

        let this = State {
            tab_state,
            share_code_state,
        };

        cx.set_global(this);
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut WindowContext), cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|mut this, cx| {
            f(&mut this, cx);
        });
    }
}

impl Global for State {}
