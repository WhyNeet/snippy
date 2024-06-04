use gpui::{Action, Menu, MenuItem, WindowOptions};

use crate::{state::State, workspace::Workspace};

pub fn launch(app: gpui::App) {
    app.run(|cx| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.set_window_title("snippy");

            State::init(cx);

            Workspace::build(cx)
        });
    })
}
