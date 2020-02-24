use gtk::prelude::*;

pub fn date_c(date: &chrono::Date<chrono_tz::Tz>) -> gtk::Label {
    let lbl = gtk::Label::new(Some(&format!("{}", date.format("%B %e, %Y"))));
    lbl.show_all();
    lbl
}
