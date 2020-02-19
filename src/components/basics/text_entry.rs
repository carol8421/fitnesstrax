use gtk::prelude::*;

pub fn text_entry_c(current: &str, on_changed: Box<dyn Fn(&str)>) -> gtk::Entry {
    let entry = gtk::Entry::new();
    entry.set_text(current);
    entry.connect_changed(move |v| match v.get_text() {
        Some(ref s) => on_changed(s),
        None => (),
    });
    entry
}
