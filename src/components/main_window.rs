use crate::context::{Application, Message, State};
use gtk::prelude::*;
use std::sync::{Arc, RwLock};

use crate::components::*;

pub struct MainWindow {
    settings_label: gtk::Label,
    history_label: Option<gtk::Label>,
    history: Option<History>,
}

impl MainWindow {
    pub fn new(ctx: Arc<RwLock<Application>>, app: &gtk::Application) -> MainWindow {
        let ctx_ = ctx.read().unwrap();
        let state = ctx_.get_state();

        let widget = gtk::ApplicationWindow::new(app);
        widget.set_title("Fitnesstrax");
        widget.set_default_size(350, 70);
        let notebook = gtk::Notebook::new();

        let settings_label = gtk::Label::new(Some(&state.text().preferences()));
        let settings_ui = Settings::new(ctx.clone());
        notebook.append_page(&settings_ui.widget(), Some(&settings_label));

        let (history_label, history) = match state {
            State::Unconfigured(_) => (None, None),
            State::Configured(state) => {
                let history = History::new(
                    state.range(),
                    state.get_history().unwrap(),
                    state.text(),
                    state.timezone(),
                    state.units(),
                    ctx.clone(),
                );
                let history_label = gtk::Label::new(Some(&state.text().history()));
                notebook.prepend_page(&history.widget(), Some(&history_label));
                (Some(history_label), Some(history))
            }
        };

        notebook.show();
        widget.add(&notebook);
        widget.show();

        MainWindow {
            history_label,
            settings_label,
            history,
        }
    }

    pub fn update_from(&mut self, message: Message) {
        match message {
            Message::ChangeRange { range, records } => {
                if let Some(ref mut history) = self.history {
                    history.set_range(range);
                    history.set_records(records);
                };
            }
            Message::ChangeLanguage(text) => {
                self.history_label
                    .as_ref()
                    .map(|label| label.set_markup(&text.history()));
                self.settings_label.set_markup(&text.preferences());
                // self.history.set_language(state);
            }
            Message::ChangeTimezone(timezone) => (), // self.history.set_timezone(state),
            Message::ChangeUnits(units) => (),       // self.history.set_units(state),
            Message::RecordsUpdated(records) => {
                self.history.as_mut().map(|h| h.set_records(records));
            }
        }
    }
}
