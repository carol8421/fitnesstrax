use dimensioned::si::Second;
use gtk::prelude::*;

use crate::settings::Settings;
use fitnesstrax_lib;

fn activity_c(activity: &fitnesstrax_lib::repduration::ActivityType) -> gtk::Label {
    gtk::Label::new(match activity {
        fitnesstrax_lib::repduration::ActivityType::MartialArts => Some("MartialArts"),
        fitnesstrax_lib::repduration::ActivityType::Planks => Some("Planks"),
        fitnesstrax_lib::repduration::ActivityType::Yoga => Some("Yoga"),
    })
}

fn sets_c(sets: &Vec<Second<f64>>) -> gtk::Label {
    let set_strs: Vec<String> = sets.iter().map(|r| format!("{}", r)).collect();
    gtk::Label::new(Some(&set_strs.join(" ")))
}

pub fn rep_duration_c(record: &fitnesstrax_lib::repduration::RepDurationRecord) -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    container.add(&activity_c(&record.activity));
    container.add(&sets_c(&record.sets));

    container
}
