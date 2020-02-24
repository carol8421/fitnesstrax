use gtk::prelude::*;

use crate::components::Component;

pub struct SwappableComponent {
    widget: gtk::Box,
    child: Box<dyn Component>,
}

impl SwappableComponent {
    pub fn new(initial_component: Box<dyn Component>) -> SwappableComponent {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 5);
        widget.pack_start(&initial_component.widget(), true, true, 5);
        widget.show();

        SwappableComponent {
            widget,
            child: initial_component,
        }
    }

    pub fn swap(&mut self, new_component: Box<dyn Component>) {
        self.widget.remove(&self.child.widget());
        self.child = new_component;
        self.widget.pack_start(&self.child.widget(), true, true, 5);
    }
}

impl Component for SwappableComponent {
    fn widget(&self) -> gtk::Widget {
        self.widget.clone().upcast::<gtk::Widget>()
    }
}

impl Component for gtk::Box {
    fn widget(&self) -> gtk::Widget {
        self.clone().upcast::<gtk::Widget>()
    }
}
