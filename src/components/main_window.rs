use crate::context::{Application, Message, State};
use gtk::prelude::*;
use std::sync::{Arc, RwLock};

use crate::components::*;

pub struct MainWindow {
    notebook: gtk::Notebook,
    history_idx: Option<u32>,
    history_page: Option<Page<History>>,
    //settings_idx: u32,
    settings_page: Page<Settings>,
    ctx: Arc<RwLock<Application>>,
}

impl MainWindow {
    pub fn new(ctx: Arc<RwLock<Application>>, app: &gtk::Application) -> MainWindow {
        let ctx_ = ctx.read().unwrap();
        let state = ctx_.get_state();

        let widget = gtk::ApplicationWindow::new(app);
        widget.set_title("Fitnesstrax");
        widget.set_default_size(350, 70);
        let notebook = gtk::Notebook::new();

        let settings_page = Page::new(&state.text().preferences(), Settings::new(ctx.clone()));

        let history_page = match state {
            State::Unconfigured(_) => None,
            State::Configured(state) => {
                let history = History::new(
                    state.range(),
                    state.get_history().unwrap(),
                    state.text(),
                    state.timezone(),
                    state.units(),
                    ctx.clone(),
                );
                Some(Page::new(&state.text().history(), history))
            }
        };

        notebook.show();
        widget.add(&notebook);
        widget.show();

        /*let settings_idx =*/
        notebook.append_page(
            &settings_page.component.widget(),
            Some(&settings_page.label),
        );
        let history_idx = match history_page {
            Some(ref page) => {
                Some(notebook.prepend_page(&page.component.widget(), Some(&page.label)))
            }
            None => None,
        };

        let self_ = MainWindow {
            notebook,
            history_idx,
            history_page: history_page,
            //settings_idx,
            settings_page,
            ctx: ctx.clone(),
        };
        self_.select_history_page();

        self_
    }

    /* Move this into the Application, similar to the architecture shown here:
     * https://gitlab.gnome.org/World/Shortwave/-/blob/master/src/app.rs
     */
    pub fn update_from(&mut self, message: Message) {
        match message {
            Message::ChangeSeriesFile { range, records } => match self.history_page {
                None => {
                    let ctx_ = self.ctx.read().unwrap();
                    let state = ctx_.get_state();
                    let history = History::new(
                        range,
                        records,
                        state.text(),
                        state.timezone(),
                        state.units(),
                        self.ctx.clone(),
                    );
                    let history_page = Page::new(&state.text().history(), history);
                    self.history_idx =
                        Some(self.notebook.prepend_page(
                            &history_page.component.widget(),
                            Some(&history_page.label),
                        ));
                    self.history_page = Some(history_page);
                }
                Some(ref mut page) => {
                    page.component.set_range(range);
                    page.component.set_records(records);
                }
            },
            Message::ChangeRange { range, records } => {
                if let Some(ref mut page) = self.history_page {
                    page.component.set_range(range);
                    page.component.set_records(records);
                };
            }
            Message::ChangeLanguage(ref text) => {
                if let Some(ref mut page) = self.history_page {
                    page.set_label(&text.history());
                    page.component.set_language(text.clone());
                }
                self.settings_page.set_label(&text.preferences());
            }
            Message::ChangeTimezone(timezone) => {
                self.history_page
                    .as_mut()
                    .map(|page| page.component.set_timezone(timezone));
            }
            Message::ChangeUnits(units) => {
                self.history_page
                    .as_mut()
                    .map(|page| page.component.set_units(units));
            }
            Message::RecordsUpdated(records) => {
                self.history_page
                    .as_mut()
                    .map(|page| page.component.set_records(records));
            }
        }
    }

    /*
    fn set_settings_page(&self) {
        self.notebook.set_current_page(Some(self.settings_idx));
    }
    */

    fn select_history_page(&self) {
        self.notebook.set_current_page(self.history_idx);
    }
}

struct Page<T: Component> {
    pub label: gtk::Label,
    pub component: T,
}

impl<T: Component> Page<T> {
    pub fn new(label_text: &str, component: T) -> Page<T> {
        Page {
            label: gtk::Label::new(Some(label_text)),
            component,
        }
    }

    pub fn set_label(&self, label_text: &str) {
        self.label.set_markup(label_text);
    }
}
