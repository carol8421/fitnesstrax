use gtk::prelude::*;

use crate::components::Component;
use crate::i18n::Text;

#[derive(Clone)]
pub struct About {
    widget: gtk::Box,
}

impl About {
    pub fn new(text: &Text) -> About {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 5);

        widget.pack_start(
            &gtk::Label::new(Some(&text.name_with_version())),
            false,
            false,
            0,
        );

        let no_adjustment: Option<&gtk::Adjustment> = None;
        let scrolling_info = gtk::ScrolledWindow::new(no_adjustment, no_adjustment);

        let tag_table: Option<&gtk::TextTagTable> = None;

        let dependencies_buffer = gtk::TextBuffer::new(tag_table);
        dependencies_buffer.set_text(&text.dependencies());
        let dependencies_view = gtk::TextView::new_with_buffer(&dependencies_buffer);

        let license_buffer = gtk::TextBuffer::new(tag_table);
        license_buffer.set_text(&text.license());
        let license_view = gtk::TextView::new_with_buffer(&license_buffer);

        let info_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        info_box.pack_start(&dependencies_view, false, false, 0);
        info_box.pack_start(&license_view, false, false, 0);

        scrolling_info.add(&info_box);
        scrolling_info.show();

        widget.pack_start(&scrolling_info, true, true, 0);

        widget.show_all();

        About { widget }
    }

    pub fn set_language(&mut self, text: &Text) {}
}

impl Component for About {
    fn widget(&self) -> gtk::Widget {
        self.widget.clone().upcast::<gtk::Widget>()
    }
}
