use chrono::Date;
use chrono_tz::Tz;
use emseries::Record;
use fitnesstrax::TraxRecord;
use gtk::prelude::*;
use std::sync::{Arc, RwLock};

use crate::components::{Component, Day, RangeSelector};
use crate::context::AppContext;
use crate::range::group_by_date;
use crate::settings::Settings;
use crate::types::DateRange;

#[derive(Clone)]
pub struct History {
    widget: gtk::Box,
    history_box: gtk::Box,

    range: DateRange,
    records: Vec<Record<TraxRecord>>,
    settings: Settings,

    ctx: Arc<RwLock<AppContext>>,
}

impl History {
    pub fn new(
        range: DateRange,
        records: Vec<Record<TraxRecord>>,
        settings: Settings,
        ctx: Arc<RwLock<AppContext>>,
    ) -> History {
        let widget = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let history_box = gtk::Box::new(gtk::Orientation::Vertical, 5);

        let range_bar = {
            let ctx = ctx.clone();
            RangeSelector::new(
                range.clone(),
                Box::new(move |new_range| ctx.write().unwrap().set_range(new_range)),
            )
        };
        let no_adjustment: Option<&gtk::Adjustment> = None;
        let scrolling_history = gtk::ScrolledWindow::new(no_adjustment, no_adjustment);
        scrolling_history.add(&history_box);

        widget.pack_start(&range_bar.widget, false, false, 25);
        widget.pack_start(&scrolling_history, true, true, 5);

        widget.show();
        history_box.show_all();
        scrolling_history.show();
        range_bar.show();

        let mut component = History {
            widget,
            history_box,
            range,
            records,
            settings,
            ctx,
        };

        component.render();

        component
    }

    pub fn set_range(&mut self, range: DateRange) {
        self.range = range;
        self.render();
    }

    pub fn set_records(&mut self, records: Vec<Record<TraxRecord>>) {
        self.records = records;
        self.render();
    }

    pub fn set_settings(&mut self, settings: Settings) {
        self.settings = settings;
        self.render();
    }

    fn render(&mut self) {
        let grouped_history = group_by_date(&self.range, self.records.clone());
        self.history_box.foreach(|child| child.destroy());
        let mut dates = grouped_history.keys().collect::<Vec<&Date<Tz>>>();
        dates.sort_unstable();
        dates.reverse();
        dates.iter().for_each(|date| {
            let ctx = self.ctx.clone();
            let day = Day::new(
                ctx,
                *date.clone(),
                grouped_history.get(date).unwrap().clone(),
                self.settings.clone(),
            );
            self.history_box.pack_start(&day.widget(), true, true, 25);
        });
    }
}

impl Component for History {
    fn widget(&self) -> gtk::Widget {
        self.widget.clone().upcast::<gtk::Widget>()
    }
}
