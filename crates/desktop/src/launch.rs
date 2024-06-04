use gpui::WindowOptions;

use crate::{state::State, workspace::Workspace};

pub fn launch(app: gpui::App) {
    app.run(|cx| {
        cx.open_window(WindowOptions::default(), |cx| {
            State::init(cx);

            Workspace::build(cx)
        });
    })
}
