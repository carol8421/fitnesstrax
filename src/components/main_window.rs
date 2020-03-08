use crate::context::{AppContext, Message};
use gtk::prelude::*;
use std::sync::{Arc, RwLock};

use crate::components::*;

pub struct MainWindow {
    history_label: gtk::Label,
    settings_label: gtk::Label,
    history: History,
}

impl MainWindow {
    pub fn new(ctx: Arc<RwLock<AppContext>>, app: &gtk::Application) -> MainWindow {
        let (settings, range, records) = {
            let ctx = ctx.read().unwrap();
            (
                ctx.get_settings(),
                ctx.get_range(),
                ctx.get_history().unwrap(),
            )
        };

        let widget = gtk::ApplicationWindow::new(app);
        widget.set_title("Fitnesstrax");
        widget.set_default_size(350, 70);

        let notebook = gtk::Notebook::new();
        let history = History::new(range, records, settings.clone(), ctx.clone());
        let settings_ui = Settings::new(ctx.clone());

        let history_label = gtk::Label::new(Some(&settings.text.history()));
        let settings_label = gtk::Label::new(Some(&settings.text.preferences()));

        notebook.append_page(&history.widget(), Some(&history_label));
        notebook.append_page(&settings_ui.widget(), Some(&settings_label));

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
                self.history.set_range(range);
                self.history.set_records(records);
            }
            Message::ChangeSettings { settings } => {
                self.history_label.set_markup(&settings.text.history());
                self.settings_label.set_markup(&settings.text.preferences());
                self.history.set_settings(settings);
            }
            Message::RecordsUpdated { records } => {
                self.history.set_records(records);
            }
        }
    }
}
