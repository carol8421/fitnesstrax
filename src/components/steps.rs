use emseries::{Recordable, UniqueId};
use fitnesstrax_lib::steps::StepRecord;
use gtk::prelude::*;

use crate::components::validated_text_entry_c;
use crate::errors::Error;
use crate::i18n::Text;

pub fn steps_c(record: &fitnesstrax_lib::steps::StepRecord, text: &Text) -> gtk::Label {
    gtk::Label::new(Some(&text.step_count(record.steps)))
}

pub fn steps_edit_c(
    id: UniqueId,
    record: StepRecord,
    text: &Text,
    on_update: Box<dyn Fn(UniqueId, StepRecord)>,
) -> gtk::Box {
    let b = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    let entry = validated_text_entry_c(
        record.steps,
        Box::new(|s| format!("{}", s)),
        Box::new(|s| s.parse::<u32>().map_err(|_err| Error::ParseStepsError)),
        Box::new(move |val| on_update(id.clone(), StepRecord::new(record.timestamp(), val))),
    );
    let label = gtk::Label::new(Some(&text.steps_label()));

    b.pack_start(&entry, false, false, 5);
    b.pack_start(&label, false, false, 5);
    b
}
