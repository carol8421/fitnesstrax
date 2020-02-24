use gtk::prelude::*;

pub struct MenuOptions<'a, Val>(pub Vec<(&'a str, Val)>);

pub fn dropmenu_c(
    MenuOptions(options): MenuOptions<&str>,
    current: &str,
    on_changed: Box<dyn Fn(&str)>,
) -> gtk::Widget {
    let combo = gtk::ComboBoxText::new();
    for (id, option) in options.iter() {
        combo.append(Some(id), option);
    }
    combo.set_active_id(Some(current));
    combo.connect_changed(move |s| {
        if let Some(val) = s.get_active_id() {
            on_changed(val.as_str());
        }
    });

    combo.upcast::<gtk::Widget>()
}
