use gtk::prelude::*;

use crate::components::Component;

pub enum LabelPosition {
    Before,
    After,
}

pub fn labeled_widget_c<A: Component>(label: &str, w: A, position: LabelPosition) -> gtk::Widget {
    let widget = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    match position {
        LabelPosition::Before => {
            widget.pack_start(&gtk::Label::new(Some(label)), false, false, 5);
            widget.pack_start(&w.widget(), false, false, 5);
        }
        LabelPosition::After => {
            widget.pack_start(&w.widget(), false, false, 5);
            widget.pack_start(&gtk::Label::new(Some(label)), false, false, 5);
        }
    }

    widget.upcast::<gtk::Widget>()
}
