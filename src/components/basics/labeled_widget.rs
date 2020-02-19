use gtk::prelude::*;

pub fn labeled_widget_c<A: IsA<gtk::Widget>>(label: &str, widget: A) -> gtk::Box {
    let w = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    w.pack_start(&gtk::Label::new(Some(label)), false, false, 5);
    w.pack_start(&widget, false, false, 5);

    w.get_style_context().add_class("labeled-widget");

    w
}
