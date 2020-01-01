use chrono::{Date, Datelike, TimeZone, Utc};
use chrono_tz::Etc::UTC;
use chrono_tz::Tz;
use gtk::prelude::*;
use std::sync::{Arc, RwLock};

pub struct DateSelector {
    pub widget: gtk::Box,
    button: gtk::Button,
    calendar: gtk::Calendar,
    date: Arc<RwLock<Date<Tz>>>,
    on_change: Arc<Box<dyn Fn(Date<Tz>)>>,
}

impl DateSelector {
    pub fn new(date: Date<Tz>, on_change: Box<dyn Fn(Date<Tz>)>) -> DateSelector {
        let w = DateSelector {
            widget: gtk::Box::new(gtk::Orientation::Vertical, 5),
            button: gtk::Button::new_with_label(&format!("{}", date.format("%B %e, %Y"))),
            calendar: gtk::Calendar::new(),
            date: Arc::new(RwLock::new(date)),
            on_change: Arc::new(on_change),
        };

        w.calendar.select_month(date.month0(), date.year() as u32);
        w.calendar.select_day(date.day());

        {
            let cal = w.calendar.clone();
            w.button.connect_clicked(move |_| {
                if !cal.get_visible() {
                    cal.show()
                } else {
                    cal.hide()
                }
            });
        }

        {
            let change_handler = w.on_change.clone();
            let button = w.button.clone();
            let date = w.date.clone();

            w.calendar.connect_day_selected(move |cal| {
                let (year, month0, day) = cal.get_date();
                *date.write().unwrap() = Utc.ymd(year as i32, month0 + 1, day).with_timezone(&UTC);
                button.set_label(&format!("{}", date.read().unwrap().format("%B %e, %Y")));
                change_handler(*date.read().unwrap());
            });
        }

        w.widget.pack_start(&w.button, false, false, 5);
        w.widget.pack_start(&w.calendar, false, false, 5);
        w.calendar.hide();

        w.show();

        w
    }

    pub fn show(&self) {
        self.widget.show();
        self.button.show();
    }

    /*
    pub fn update_from(&mut self, date: Date<Tz>) {
        if *self.date.read().unwrap() != date {
            *self.date.write().unwrap() = date;
            self.calendar
                .select_month(date.month0(), date.year() as u32);
            self.calendar.select_day(date.day());
            self.button
                .set_label(&format!("{}", date.format("%B %e, %Y")));
        }
    }
    */
}