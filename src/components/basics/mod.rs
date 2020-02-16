use dimensioned::si;
use gtk::prelude::*;
use std::convert::TryFrom;

mod dropmenu;
mod time;

pub use dropmenu::{dropmenu_c, MenuOptions};
pub use time::{time_c, time_edit_c};

use crate::components::validated_text_entry::validated_text_entry_c;
use crate::formats::{Duration, HoursMinutes};
use crate::i18n::UnitSystem;

pub fn date_c(date: &chrono::Date<chrono_tz::Tz>) -> gtk::Label {
    let lbl = gtk::Label::new(Some(&format!("{}", date.format("%B %e, %Y"))));
    lbl.show_all();
    lbl
}

pub fn distance_c(distance: &si::Meter<f64>, units: &UnitSystem) -> gtk::Label {
    gtk::Label::new(Some(&format!(
        "{}",
        units.render_distance(distance.clone())
    )))
}

pub fn distance_edit_c(
    distance: &Option<si::Meter<f64>>,
    units: &UnitSystem,
    on_update: Box<dyn Fn(Option<si::Meter<f64>>)>,
) -> gtk::Entry {
    let u1 = units.clone();
    let u2 = units.clone();
    validated_text_entry_c(
        distance.clone(),
        Box::new(move |s| {
            let u1 = u1.clone();
            s.map(move |s_| u1.render_distance(s_.clone()))
                .unwrap_or(String::from(""))
        }),
        Box::new(move |s| {
            if s.len() == 0 {
                Ok(None)
            } else {
                u2.parse_distance(s).map(|v| Some(v))
            }
        }),
        on_update,
    )
}

pub fn duration_c(duration: si::Second<f64>) -> gtk::Label {
    gtk::Label::new(Some(&format!("{}", Duration::new(duration))))
}

pub fn duration_edit_c(
    duration: &Option<si::Second<f64>>,
    on_update: Box<dyn Fn(Option<si::Second<f64>>)>,
) -> gtk::Entry {
    validated_text_entry_c(
        duration.clone(),
        Box::new(|s| {
            s.map(|s_| format!("{}", Duration::new(s_)))
                .unwrap_or(String::from(""))
        }),
        Box::new(|s| {
            if s.len() > 0 {
                s.parse::<Duration>().map(|v| Some(v.extract()))
            } else {
                Ok(None)
            }
        }),
        on_update,
    )
}

pub fn setting_with_label_c<A: IsA<gtk::Widget>>(label: &str, selector: A) -> gtk::Box {
    let widget = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    widget.pack_start(&gtk::Label::new(Some(label)), false, false, 5);
    widget.pack_start(&selector, false, false, 5);

    widget
}

pub fn text_entry_c(label: &str, current: &str, on_changed: Box<dyn Fn(&str)>) -> gtk::Box {
    let entry = gtk::Entry::new();
    entry.set_text(current);
    entry.connect_changed(move |v| match v.get_text() {
        Some(ref s) => on_changed(s),
        None => (),
    });

    setting_with_label_c(label, entry)
}
