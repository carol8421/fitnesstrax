use gtk::prelude::*;

use crate::components::Component;

#[derive(Clone)]
pub struct Container {
    pub widget: gtk::Box,
}

impl Container {
    pub fn new<A: Component>(w: Option<A>) -> Container {
        let widget = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        if let Some(w) = w {
            widget.pack_start(&w.widget(), true, true, 0);
        }
        Container { widget }
    }

    pub fn swap<A: Component>(&mut self, w: Option<A>) {
        self.widget.foreach(|child| child.destroy());
        if let Some(w) = w {
            self.widget.pack_start(&w.widget(), true, true, 0);
            self.widget.show_all();
        }
    }
}

impl Component for Container {
    fn widget(&self) -> gtk::Widget {
        self.widget.clone().upcast::<gtk::Widget>()
    }
}
