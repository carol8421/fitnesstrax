use dimensioned::si::KG;
use emseries::{DateTimeTz, Recordable, UniqueId};
use fitnesstrax_lib::steps::StepRecord;
use fitnesstrax_lib::weight::WeightRecord;
use fitnesstrax_lib::TraxRecord;
use gtk::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread;

use crate::components::basics::date_c;
use crate::components::rep_duration::rep_duration_c;
use crate::components::set_rep::set_rep_c;
use crate::components::steps::{steps_c, steps_edit_c};
use crate::components::time_distance::TimeDistanceEdit;
use crate::components::time_distance_row::time_distance_c;
use crate::components::weight::{weight_record_c, weight_record_edit_c};
use crate::components::{Component, Container};
use crate::context::Application;
use crate::i18n::{Text, UnitSystem};

#[derive(Clone)]
pub struct Day {
    widget: gtk::Box,
    edit_button: gtk::Button,
    view: Rc<RefCell<Container>>,
    ctx: Arc<RwLock<Application>>,

    date: chrono::Date<chrono_tz::Tz>,
    records: Vec<(UniqueId, TraxRecord)>,
    timezone: chrono_tz::Tz,
    text: Text,
    units: UnitSystem,
}

impl Component for Day {
    fn widget(&self) -> gtk::Widget {
        self.widget.clone().upcast::<gtk::Widget>()
    }
}

impl Day {
    pub fn new(
        ctx: Arc<RwLock<Application>>,
        date: chrono::Date<chrono_tz::Tz>,
        records: Vec<(UniqueId, TraxRecord)>,
        timezone: chrono_tz::Tz,
        text: Text,
        units: UnitSystem,
    ) -> Day {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let header = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let edit_button = gtk::Button::new_with_label(&text.edit());
        edit_button.show();

        header.pack_start(&date_c(&date), false, false, 5);
        header.pack_start(&edit_button, false, false, 5);
        header.show();
        widget.pack_start(&header, false, false, 5);

        let view = Container::new(Some(day_c(
            &date,
            records.iter().map(|rec| &rec.1).collect(),
            &timezone,
            &text,
            &units,
        )));

        widget.pack_start(&view.widget(), true, true, 5);

        widget.show();

        let mut c = Day {
            widget,
            edit_button,
            view: Rc::new(RefCell::new(view)),
            ctx,
            date,
            records,
            timezone,
            text,
            units,
        };

        {
            let c = Rc::new(RefCell::new(c.clone()));
            c.borrow()
                .edit_button
                .connect_clicked(enclose!(c => move |_| c.borrow_mut().edit()));
        }

        c.view();
        c
    }

    fn view(&mut self) {
        self.view.borrow_mut().swap(Some(
            day_c(
                &self.date,
                self.records.iter().map(|rec| &rec.1).collect(),
                &self.timezone,
                &self.text,
                &self.units,
            )
            .widget(),
        ));
    }

    fn edit(&mut self) {
        let component = Rc::new(RefCell::new(self.clone()));
        let record_map = self.records.iter().fold(HashMap::new(), |mut acc, rec| {
            acc.insert(rec.0.clone(), rec.1.clone());
            acc
        });
        self.view.borrow_mut().swap(
            Some(DayEdit::new(
                &self.date,
                &record_map,
                self.timezone.clone(),
                self.text.clone(),
                self.units.clone(),
                Box::new(enclose!(component => move | updated_records, new_records| component.borrow_mut().save(updated_records, new_records))),
            Box::new(enclose!(component => move || component.borrow_mut().view())),
            ))
        );
    }

    fn save(&mut self, updated_records: Vec<(UniqueId, TraxRecord)>, new_records: Vec<TraxRecord>) {
        let ctx = self.ctx.clone();
        thread::spawn(move || {
            ctx.write()
                .unwrap()
                .save_records(updated_records, new_records)
                .expect("record save failed");
        });
        self.view();
    }
}

fn day_c(
    _date: &chrono::Date<chrono_tz::Tz>,
    data: Vec<&TraxRecord>,
    timezone: &chrono_tz::Tz,
    text: &Text,
    units: &UnitSystem,
) -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let first_row = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    container.pack_start(&first_row, false, false, 5);

    let mut weight_component = None;
    let mut step_component = None;
    let mut rep_duration_components: Vec<gtk::Box> = Vec::new();
    let mut set_rep_components: Vec<gtk::Box> = Vec::new();
    let mut time_distance_components: Vec<gtk::Box> = Vec::new();
    let mut records = data.clone();
    records.sort_unstable_by_key(|rec| rec.timestamp());
    for record in records {
        match record {
            TraxRecord::Comments(ref _rec) => (),
            TraxRecord::RepDuration(ref rec) => rep_duration_components.push(rep_duration_c(&rec)),
            TraxRecord::SetRep(ref rec) => set_rep_components.push(set_rep_c(&rec)),
            TraxRecord::Steps(ref rec) => step_component = Some(steps_c(&rec, text)),
            TraxRecord::TimeDistance(ref rec) => {
                time_distance_components.push(time_distance_c(&rec, timezone, text, units))
            }
            TraxRecord::Weight(ref rec) => {
                weight_component = Some(weight_record_c(&rec, text, units))
            }
        }
    }

    weight_component.map(|c| first_row.pack_start(&c, false, false, 5));
    step_component.map(|c| first_row.pack_start(&c, false, false, 5));
    for component in time_distance_components {
        container.pack_start(&component, false, false, 5);
    }
    for component in set_rep_components {
        container.pack_start(&component, false, false, 5);
    }
    for component in rep_duration_components {
        container.pack_start(&component, false, false, 5);
    }

    container.show_all();
    return container;
}

#[derive(Clone)]
struct DayEdit {
    widget: gtk::Box,
}

impl Component for DayEdit {
    fn widget(&self) -> gtk::Widget {
        self.widget.clone().upcast::<gtk::Widget>()
    }
}

impl DayEdit {
    fn new(
        date: &chrono::Date<chrono_tz::Tz>,
        data: &HashMap<UniqueId, TraxRecord>,
        timezone: chrono_tz::Tz,
        text: Text,
        units: UnitSystem,
        on_save: Box<dyn Fn(Vec<(UniqueId, TraxRecord)>, Vec<TraxRecord>)>,
        on_cancel: Box<dyn Fn()>,
    ) -> DayEdit {
        let updates = Rc::new(RefCell::new(HashMap::new()));
        let new_records = Rc::new(RefCell::new(HashMap::new()));

        let widget = gtk::Box::new(gtk::Orientation::Vertical, 5);

        let first_row = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        widget.pack_start(&first_row, false, false, 5);

        let mut weight_component = {
            weight_record_edit_c(
                UniqueId::new(),
                WeightRecord::new(DateTimeTz(date.clone().and_hms(0, 0, 0)), 0.0 * KG),
                &text,
                units.clone(),
                Box::new(enclose!(new_records => move |id, rec| {
                    new_records.borrow_mut().insert(id, TraxRecord::from(rec));
                })),
            )
        };

        let mut step_component = {
            steps_edit_c(
                UniqueId::new(),
                StepRecord::new(DateTimeTz(date.clone().and_hms(0, 0, 0)), 0),
                &text,
                Box::new(enclose!(new_records => move |id, rec| {
                    new_records.borrow_mut().insert(id, TraxRecord::from(rec));
                })),
            )
        };

        let mut time_distance_records = Vec::new();

        for (id, data) in data {
            match data {
                TraxRecord::Weight(ref rec) => {
                    weight_component = weight_record_edit_c(
                        id.clone(),
                        rec.clone(),
                        &text,
                        units.clone(),
                        Box::new(enclose!(updates => move |id, rec| {
                            updates.borrow_mut().insert(id, TraxRecord::from(rec));
                        })),
                    )
                }
                TraxRecord::Steps(ref rec) => {
                    step_component = steps_edit_c(
                        id.clone(),
                        rec.clone(),
                        &text,
                        Box::new(enclose!(updates => move |id_, rec| {
                            updates.borrow_mut().insert(id_.clone(), TraxRecord::from(rec));
                        })),
                    )
                }
                TraxRecord::TimeDistance(ref rec) => {
                    time_distance_records.push((id, rec));
                }
                _ => (),
            }
        }

        let time_distance_edit = TimeDistanceEdit::new(
            date.clone(),
            time_distance_records,
            timezone.clone(),
            text.clone(),
            units.clone(),
        );

        first_row.pack_start(&weight_component, false, false, 5);
        first_row.pack_start(&step_component, false, false, 5);
        widget.pack_start(&time_distance_edit.widget, false, false, 5);

        let buttons_row = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let save_button = gtk::Button::new_with_label(&text.save());
        let cancel_button = gtk::Button::new_with_label(&text.cancel());
        buttons_row.pack_start(&save_button, false, false, 5);
        buttons_row.pack_start(&cancel_button, false, false, 5);
        widget.pack_start(&buttons_row, false, false, 5);

        save_button.connect_clicked(enclose!(updates, new_records => move |_| {
            let mut updated_records: Vec<(UniqueId, TraxRecord)> = updates
                .borrow()
                .iter()
                .map(|(id, record)| (id.clone(), record.clone()))
                .collect();
            let mut new_records: Vec<TraxRecord> = new_records
                .borrow()
                .values()
                .map(|v| v.clone())
                .collect();

            updated_records.append(
                &mut time_distance_edit
                    .updated_records()
                    .into_iter()
                    .map(|(id, rec)| (id, TraxRecord::from(rec)))
                    .collect::<Vec<(UniqueId, TraxRecord)>>(),
            );

            new_records.append(
                &mut time_distance_edit
                    .new_records()
                    .into_iter()
                    .map(|(_, rec)| TraxRecord::from(rec))
                    .collect::<Vec<TraxRecord>>(),
            );

            on_save(updated_records, new_records);
        }));
        cancel_button.connect_clicked(move |_| on_cancel());

        widget.show_all();

        DayEdit { widget }
    }
}
