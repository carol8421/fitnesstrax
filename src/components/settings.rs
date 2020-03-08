use chrono_tz::Tz;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::components::{
    dropmenu_c, labeled_widget_c, text_entry_c, Component, Container, MenuOptions,
};
use crate::context::AppContext;
use crate::i18n::{Text, UnitSystem};
use crate::settings;

#[derive(Clone)]
pub struct Settings {
    widget: gtk::Box,

    database_path_widget: Container,
    language_widget: Container,
    timezone_widget: Container,
    units_widget: Container,

    ctx: Arc<RwLock<AppContext>>,
}

impl Settings {
    pub fn new(ctx: Arc<RwLock<AppContext>>) -> Settings {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 5);

        let no_widget: Option<gtk::Widget> = None;

        let mut component = Settings {
            widget: widget.clone(),
            database_path_widget: Container::new(no_widget.clone()),
            language_widget: Container::new(no_widget.clone()),
            timezone_widget: Container::new(no_widget.clone()),
            units_widget: Container::new(no_widget.clone()),
            ctx: ctx.clone(),
        };

        component
            .widget
            .pack_start(&component.database_path_widget.widget, false, false, 0);
        component
            .widget
            .pack_start(&component.language_widget.widget, false, false, 0);
        component
            .widget
            .pack_start(&component.timezone_widget.widget, false, false, 0);
        component
            .widget
            .pack_start(&component.units_widget.widget, false, false, 0);

        let series_path = ctx
            .read()
            .unwrap()
            .get_series_path()
            .and_then(|s| s.to_str())
            .map(String::from);
        let settings::Settings {
            timezone,
            units,
            text,
        } = ctx.read().unwrap().get_settings();

        {
            component.database_path_widget.swap(Some(labeled_widget_c(
                &text.database_path(),
                text_entry_c(
                    &series_path.unwrap_or(String::from("")),
                    Box::new(enclose!(ctx => move |s| ctx.write().unwrap().set_series_path(s))),
                ),
            )));
        }

        {
            let component = Rc::new(RefCell::new(component.clone()));

            component
                .borrow_mut()
                .language_widget
                .swap(Some(language_menu(&text, component.clone())));
            component
                .borrow_mut()
                .timezone_widget
                .swap(Some(timezone_menu(&text, &timezone, component.clone())));
            component.borrow_mut().units_widget.swap(Some(units_menu(
                &text,
                &units,
                component.clone(),
            )));
        }

        component.widget.show_all();

        component
    }

    fn set_language(&mut self, language: &str) {
        let (
            series_path,
            settings::Settings {
                text,
                timezone,
                units,
            },
        ) = {
            let mut ctx = self.ctx.write().unwrap();
            ctx.set_language(language);
            (
                ctx.get_series_path()
                    .and_then(|s| s.to_str())
                    .map(String::from),
                ctx.get_settings(),
            )
        };

        let ctx = self.ctx.clone();
        self.database_path_widget.swap(Some(labeled_widget_c(
            &text.database_path(),
            text_entry_c(
                &series_path.unwrap_or(String::from("")),
                Box::new(move |s| ctx.write().unwrap().set_series_path(s)),
            ),
        )));

        {
            let component = Rc::new(RefCell::new(self.clone()));
            self.language_widget
                .swap(Some(language_menu(&text, component.clone())));
            self.timezone_widget
                .swap(Some(timezone_menu(&text, &timezone, component.clone())));
            self.units_widget
                .swap(Some(units_menu(&text, &units, component.clone())));
        }
    }

    fn set_timezone(&self, timezone_str: &str) {
        let mut ctx = self.ctx.write().unwrap();
        ctx.set_timezone(timezone_str.parse().unwrap());
    }

    fn set_units(&self, units: &str) {
        let mut ctx = self.ctx.write().unwrap();
        ctx.set_units(units);
    }
}

impl Component for Settings {
    fn widget(&self) -> gtk::Widget {
        self.widget.clone().upcast::<gtk::Widget>()
    }
}

fn language_menu(text: &Text, component: Rc<RefCell<Settings>>) -> gtk::Widget {
    labeled_widget_c(
        text.language().as_str(),
        dropmenu_c(
            MenuOptions(vec![("en", "English"), ("eo", "Esperanto")]),
            text.language_id(),
            Box::new(move |s| component.borrow_mut().set_language(s)),
        ),
    )
}

fn timezone_menu(text: &Text, timezone: &Tz, component: Rc<RefCell<Settings>>) -> gtk::Widget {
    labeled_widget_c(
        &text.timezone(),
        dropmenu_c(
            MenuOptions(vec![
                ("America/Chicago", "United States: Chicago"),
                ("America/New_York", "United States: New York"),
            ]),
            timezone.name(),
            Box::new(enclose!(component => move |s| component.borrow_mut().set_timezone(s))),
        ),
    )
}

fn units_menu(text: &Text, units: &UnitSystem, component: Rc<RefCell<Settings>>) -> gtk::Widget {
    labeled_widget_c(
        &text.units(),
        dropmenu_c(
            MenuOptions(vec![
                ("SI", "SI (kg, km, m/s)"),
                ("USA", "USA (lbs, mi, mph)"),
            ]),
            &String::from(units),
            Box::new(enclose!(component => move |s| component.borrow_mut().set_units(s))),
        ),
    )
}
