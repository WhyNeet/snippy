use gpui::*;

pub struct ShareCodeState {
    code: Option<String>,
}

pub struct SharedCodeChangeEvent(pub Option<String>);

impl SharedCodeChangeEvent {
    pub fn from_clipboard(cx: &mut WindowContext) -> Self {
        Self(cx.read_from_clipboard().map(|c| c.text().clone()))
    }
}

impl EventEmitter<SharedCodeChangeEvent> for ShareCodeState {}

impl ShareCodeState {
    pub fn build(cx: &mut WindowContext) -> Model<Self> {
        cx.new_model(|cx| Self { code: None })
    }

    pub fn get_code(&self) -> Option<&String> {
        self.code.as_ref()
    }

    pub fn set_code(&mut self, code: Option<String>) {
        self.code = code;
    }
}
