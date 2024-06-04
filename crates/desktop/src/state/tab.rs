use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Share,
    Receive,
}

pub struct TabState {
    current_tab: Tab,
}

#[derive(Debug)]
pub struct SwitchTabEvent(pub Tab);

impl EventEmitter<SwitchTabEvent> for TabState {}

impl TabState {
    pub fn build(cx: &mut WindowContext) -> Model<Self> {
        cx.new_model(|cx| {
            let current_tab = Tab::Share;

            Self { current_tab }
        })
    }

    pub fn get_current_tab(&self) -> Tab {
        self.current_tab
    }

    pub fn set_current_tab(&mut self, tab: Tab) {
        self.current_tab = tab;
    }
}
