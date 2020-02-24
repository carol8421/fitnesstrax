use std::convert::TryFrom;

use super::validated_text_entry_c;
use crate::formats::HoursMinutes;

pub fn time_c(time: &chrono::NaiveTime) -> gtk::Label {
    gtk::Label::new(Some(&format!("{}", HoursMinutes::new(time))))
}

pub fn time_edit_c(
    time: &chrono::NaiveTime,
    on_update: Box<dyn Fn(chrono::NaiveTime)>,
) -> gtk::Widget {
    validated_text_entry_c(
        time.clone(),
        Box::new(|s| format!("{}", HoursMinutes::new(s))),
        Box::new(|s| HoursMinutes::try_from(s).map(|v| v.extract())),
        on_update,
    )
}
