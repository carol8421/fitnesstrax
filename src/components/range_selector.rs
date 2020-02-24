use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::components::date_selector_c;
use crate::types::DateRange;

pub struct RangeSelector {
    pub widget: gtk::Box,
    start_selector: gtk::Box,
    end_selector: gtk::Box,
}

impl RangeSelector {
    pub fn new(range: DateRange, on_change: Box<dyn Fn(DateRange)>) -> RangeSelector {
        let start_date = range.start.clone();
        let end_date = range.end.clone();
        let on_change = Rc::new(on_change);
        let timezone = Rc::new(range.start.timezone());

        let range = Rc::new(RefCell::new(range));
        let start_selector = {
            date_selector_c(
                start_date,
                Box::new(enclose!(range, on_change, timezone => move |new_date| {
                    let mut r = range.borrow_mut();
                    let new_range = DateRange {
                        start: new_date.with_timezone(&*timezone),
                        end: r.end.clone(),
                    };
                    *r = new_range;
                    on_change(r.clone());
                })),
            )
        };

        let end_selector = {
            date_selector_c(
                end_date,
                Box::new(enclose!(range, on_change, timezone => move |new_date| {
                    let mut r = range.borrow_mut();
                    let new_range = DateRange {
                        start: r.start.clone(),
                        end: new_date.with_timezone(&*timezone),
                    };
                    *r = new_range;
                    on_change(r.clone());
                })),
            )
        };

        let w = RangeSelector {
            widget: gtk::Box::new(gtk::Orientation::Vertical, 5),
            start_selector,
            end_selector,
        };

        w.widget.pack_start(&w.start_selector, false, false, 5);
        w.widget.pack_start(&w.end_selector, false, false, 5);

        {}

        w
    }

    pub fn show(&self) {
        self.widget.show();
    }
}
