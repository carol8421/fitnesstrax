use gtk::prelude::*;

pub struct MenuOptions<'a, Val>(pub Vec<(&'a str, Val)>);

pub fn dropmenu_c(
    label: &str,
    MenuOptions(options): MenuOptions<&str>,
    current: &str,
    on_changed: Box<dyn Fn(&str)>,
) -> gtk::Box {
    let widget = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    widget.pack_start(&gtk::Label::new(Some(label)), true, true, 5);

    let combo = gtk::ComboBoxText::new();
    for (id, option) in options.iter() {
        combo.append(Some(id), option);
    }
    combo.set_active_id(Some(current));
    combo.connect_changed(move |s| match s.get_active_id() {
        Some(val) => on_changed(val.as_str()),
        None => (),
    });

    widget.pack_start(&combo, true, true, 5);

    widget
}
