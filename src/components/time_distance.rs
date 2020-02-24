use emseries::{DateTimeTz, Recordable, UniqueId};
use fitnesstrax::timedistance::{ActivityType, TimeDistanceRecord};
use gtk::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::components::time_distance_row::time_distance_record_edit_c;
use crate::settings::Settings;

#[derive(Clone)]
pub struct TimeDistanceEdit {
    pub widget: gtk::Box,
    record_box: gtk::Box,

    records: HashMap<UniqueId, TimeDistanceRecord>,
    settings: Settings,
    updated_records: Rc<RefCell<HashMap<UniqueId, TimeDistanceRecord>>>,
    new_records: Rc<RefCell<HashMap<UniqueId, TimeDistanceRecord>>>,
}

impl TimeDistanceEdit {
    pub fn new(
        date: chrono::Date<chrono_tz::Tz>,
        records: Vec<(&UniqueId, &TimeDistanceRecord)>,
        settings: Settings,
    ) -> TimeDistanceEdit {
        let mut record_hash: HashMap<UniqueId, TimeDistanceRecord> = HashMap::new();
        for (id, rec) in records.iter() {
            record_hash.insert((*id).clone(), (*rec).clone());
        }

        let updated_records = Rc::new(RefCell::new(HashMap::new()));
        let new_records = Rc::new(RefCell::new(HashMap::new()));

        let widget = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let record_box = gtk::Box::new(gtk::Orientation::Vertical, 5);

        let w = TimeDistanceEdit {
            widget,
            record_box,

            records: record_hash,
            settings: settings.clone(),
            updated_records,
            new_records: new_records.clone(),
        };

        let button_box = {
            let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
            let new_button =
                gtk::Button::new_with_label(&settings.text.add_time_distance_workout());
            new_button.show();
            button_box.pack_start(&new_button, false, false, 5);
            new_button.connect_clicked(enclose!(w, new_records => move |_| {
                new_records.borrow_mut().insert(
                    UniqueId::new(),
                    TimeDistanceRecord::new(
                        DateTimeTz(date.and_hms(0, 0, 0)),
                        ActivityType::Cycling,
                        None,
                        None,
                        None,
                    ),
                );
                w.render();
            }));
            button_box
        };

        w.widget.pack_start(&w.record_box, false, false, 5);
        w.widget.pack_start(&button_box, false, false, 5);

        w.render();

        w
    }

    pub fn render(&self) {
        self.record_box.foreach(|child| child.destroy());

        let mut sorted_records: Vec<(&UniqueId, &TimeDistanceRecord)> = self
            .records
            .iter()
            .map(|(id, record)| (id, record))
            .collect();
        sorted_records.sort_unstable_by_key(|(_, rec)| rec.timestamp());

        for (id, record) in sorted_records {
            let updated_records = self.updated_records.clone();
            match self.updated_records.borrow().get(id) {
                Some(rec) => {
                    self.record_box.pack_start(
                        &time_distance_record_edit_c(
                            id.clone(),
                            rec.clone(),
                            self.settings.clone(),
                            Box::new(move |id, rec| {
                                updated_records.borrow_mut().insert(id, rec);
                            }),
                        ),
                        false,
                        false,
                        5,
                    );
                }
                None => {
                    self.record_box.pack_start(
                        &time_distance_record_edit_c(
                            id.clone(),
                            record.clone(),
                            self.settings.clone(),
                            Box::new(move |id, rec| {
                                updated_records.borrow_mut().insert(id, rec);
                            }),
                        ),
                        false,
                        false,
                        5,
                    );
                }
            }
        }

        for (id, record) in self.new_records.borrow().iter() {
            let new_records = self.new_records.clone();
            self.record_box.pack_start(
                &time_distance_record_edit_c(
                    id.clone(),
                    record.clone(),
                    self.settings.clone(),
                    Box::new(move |id, rec| {
                        new_records.borrow_mut().insert(id, rec);
                    }),
                ),
                false,
                false,
                5,
            );
        }

        self.record_box.show_all();
    }

    pub fn updated_records(&self) -> Vec<(UniqueId, TimeDistanceRecord)> {
        self.updated_records
            .borrow()
            .iter()
            .map(|(id, record)| (id.clone(), record.clone()))
            .collect()
    }

    pub fn new_records(&self) -> Vec<(UniqueId, TimeDistanceRecord)> {
        self.new_records
            .borrow()
            .iter()
            .map(|(id, record)| (id.clone(), record.clone()))
            .collect()
    }
}
