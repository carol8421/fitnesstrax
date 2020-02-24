use gtk::prelude::*;

use crate::components::Component;

pub fn labeled_widget_c<A: Component>(label: &str, w: A) -> gtk::Widget {
    let widget = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    widget.pack_start(&gtk::Label::new(Some(label)), false, false, 5);
    widget.pack_start(&w.widget(), false, false, 5);

    widget.upcast::<gtk::Widget>()
}
