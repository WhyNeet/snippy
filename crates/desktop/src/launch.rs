use assets::Assets;
use gpui::*;
use settings::Settings;
use theme::{ThemeRegistry, ThemeSettings};

use crate::{state::State, workspace::Workspace};

pub fn launch(app: gpui::App) {
    app.with_assets(Assets).run(|cx| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.set_window_title("snippy");

            load_embedded_fonts(cx).unwrap();

            settings::init(cx);
            theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);

            let theme_registry = ThemeRegistry::global(cx);
            let mut theme_settings = ThemeSettings::get_global(cx).clone();
            theme_settings.active_theme = theme_registry.get("Gruvbox Dark Hard").unwrap();
            ThemeSettings::override_global(theme_settings, cx);

            State::init(cx);

            Workspace::build(cx)
        });
    })
}

fn load_embedded_fonts(cx: &AppContext) -> gpui::Result<()> {
    let font_paths = cx.asset_source().list("fonts")?;
    let mut embedded_fonts = Vec::new();
    for font_path in font_paths {
        if font_path.ends_with(".ttf") {
            let font_bytes = cx
                .asset_source()
                .load(&font_path)?
                .expect("Should never be None in the storybook");
            embedded_fonts.push(font_bytes);
        }
    }

    cx.text_system().add_fonts(embedded_fonts)
}
