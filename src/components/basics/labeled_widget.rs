use gtk::prelude::*;

pub fn labeled_widget_c<A: IsA<gtk::Widget>>(label: &str, widget: A) -> gtk::Box {
    let widget = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    widget.pack_start(&gtk::Label::new(Some(label)), false, false, 5);
    widget.pack_start(&widget, false, false, 5);

    widget
}
