use gtk::prelude::*;
use gtk::{EditableSignals, EntryExt, StyleContextExt, WidgetExt};

use crate::errors::Error;

pub fn text_entry_c(current: &str, on_changed: Box<dyn Fn(&str)>) -> gtk::Widget {
    let entry = gtk::Entry::new();
    entry.set_text(current);
    entry.connect_changed(move |v| match v.get_text() {
        Some(ref s) => on_changed(s),
        None => (),
    });
    entry.upcast::<gtk::Widget>()
}

pub fn validated_text_entry_c<A: 'static + Clone>(
    value: A,
    render: Box<dyn Fn(&A) -> String>,
    parse: Box<dyn Fn(&str) -> Result<A, Error>>,
    on_update: Box<dyn Fn(A)>,
) -> gtk::Widget {
    let widget = gtk::Entry::new();
    widget.set_text(&render(&value));

    let w = widget.clone();
    widget.connect_changed(move |v| match v.get_text() {
        Some(ref s) => match parse(s.as_str()) {
            Ok(val) => {
                let context = w.get_style_context();
                context.remove_class(&gtk::STYLE_CLASS_WARNING);
                on_update(val);
            }
            Err(_err) => {
                let context = w.get_style_context();
                context.add_class(&gtk::STYLE_CLASS_WARNING);
            }
        },
        None => (),
    });

    widget.upcast::<gtk::Widget>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::{Arc, RwLock};

    #[test]
    fn it_calls_callbacks_on_change() {
        gtk::init();
        let change_param = Arc::new(RwLock::new(String::new()));
        let entry = {
            let mut change_param = change_param.clone();
            text_entry_c(
                "",
                Box::new(move |s| {
                    let mut c = change_param.write().unwrap();
                    c.clear();
                    c.push_str(s);
                }),
            )
        };
        entry
            .downcast::<gtk::Entry>()
            .unwrap()
            .get_buffer()
            .set_text("abcdefg");
        assert_eq!(change_param.read().unwrap().as_str(), "abcdefg");
    }
}
