use chrono::Timelike;
use emseries::*;
use gtk::prelude::*;
use std::convert::TryFrom;
//use std::sync::{Arc, RwLock};
use std::cell::RefCell;
use std::rc::Rc;

use crate::components::basics::{
    distance_c, distance_edit_c, dropmenu_c, duration_c, duration_edit_c, labeled_widget_c, time_c,
    time_edit_c, MenuOptions,
};
use crate::settings::Settings;
use fitnesstrax::timedistance::{activity_types, ActivityType, TimeDistanceRecord};

fn activity_c(activity: &ActivityType, settings: &Settings) -> gtk::Label {
    let activity_str = match activity {
        ActivityType::Cycling => settings.text.cycling(),
        ActivityType::Rowing => settings.text.rowing(),
        ActivityType::Running => settings.text.running(),
        ActivityType::Swimming => settings.text.swimming(),
        ActivityType::Walking => settings.text.walking(),
    };

    gtk::Label::new(Some(&activity_str))
}

pub fn time_distance_c(
    record: &fitnesstrax::timedistance::TimeDistanceRecord,
    settings: &Settings,
) -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    container.pack_start(
        &time_c(
            &record
                .timestamp()
                .0
                .with_timezone(&settings.timezone)
                .time(),
        ),
        false,
        false,
        5,
    );
    container.pack_start(&activity_c(&record.activity, &settings), false, false, 5);
    container.pack_start(
        &record
            .distance
            .map(|r| distance_c(&r, &settings.units))
            .unwrap_or(gtk::Label::new(Some("---"))),
        false,
        false,
        5,
    );
    container.pack_start(
        &record
            .duration
            .map(|r| duration_c(r))
            .unwrap_or(gtk::Label::new(Some("---"))),
        false,
        false,
        5,
    );

    return container;
}

pub fn time_distance_record_edit_c(
    id: UniqueId,
    record: TimeDistanceRecord,
    settings: Settings,
    on_update: Box<dyn Fn(UniqueId, TimeDistanceRecord)>,
) -> gtk::Box {
    let on_update = Rc::new(on_update);
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let record = Rc::new(RefCell::new(record));

    let time_entry = {
        let time = record
            .borrow()
            .timestamp()
            .0
            .with_timezone(&settings.timezone)
            .time();
        let settings = settings.clone();
        time_edit_c(
            &time,
            Box::new(enclose!(id, record, on_update => move |val| {
                let mut r = record.borrow_mut();
                r.timestamp = r.timestamp.map(|ts| {
                    ts.clone()
                        .with_hour(val.hour())
                        .unwrap()
                        .with_minute(val.minute())
                        .unwrap()
                        .with_second(val.second())
                        .unwrap()
                        .with_timezone(&settings.timezone)
                });
                on_update(id.clone(), r.clone());
            })),
        )
    };

    let activity_selection = {
        let menu: Vec<(String, std::borrow::Cow<'_, str>)> = activity_types()
            .iter()
            .map(|activity| {
                (
                    format!("{:?}", activity),
                    settings.text.time_distance_activity(activity),
                )
            })
            .collect();
        /* It's really annoying that I have to do this, but the iterators won't convert a vec of
         * (String, Cow) to (&str, &str), and trying to do it all in a single statement leads to a
         * lot of temporaries getting dropped. */
        let menu_: Vec<(&str, &str)> = menu
            .iter()
            .map(|(id, text)| (id.as_ref(), text.as_ref()))
            .collect();
        let activity_id = format!("{:?}", record.borrow().activity);
        labeled_widget_c(
            &settings.text.activity(),
            dropmenu_c(
                MenuOptions(menu_),
                &activity_id,
                Box::new(enclose!(id, record, on_update => move |val| {
                    let mut r = record.borrow_mut();
                    r.activity = ActivityType::try_from(val).unwrap();
                    on_update(id.clone(), r.clone());
                })),
            ),
        )
    };

    let distance_entry = {
        let distance = record.borrow().distance.clone();
        distance_edit_c(
            &distance,
            &settings.units,
            Box::new(enclose!(id, record, on_update => move |res| match res {
                Some(val) => {
                    let mut r = record.borrow_mut();
                    r.distance = Some(val);
                    on_update(id.clone(), r.clone());
                }
                None => (),
            })),
        )
    };

    let duration_entry = {
        let id = id.clone();
        let record = record.clone();
        let on_update = on_update.clone();
        let duration = record.borrow().duration.clone();
        duration_edit_c(
            &duration,
            Box::new(move |res| match res {
                Some(val) => {
                    let mut r = record.borrow_mut();
                    r.duration = Some(val);
                    on_update(id.clone(), r.clone());
                }
                None => (),
            }),
        )
    };

    container.pack_start(&time_entry, false, false, 5);
    container.pack_start(&activity_selection, false, false, 5);
    container.pack_start(&distance_entry, false, false, 5);
    container.pack_start(&duration_entry, false, false, 5);

    container
}
