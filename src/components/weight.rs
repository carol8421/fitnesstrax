use emseries::{Recordable, UniqueId};
use fitnesstrax::weight::WeightRecord;

use crate::components::basics::{labeled_widget_c, validated_text_entry_c, LabelPosition};
use crate::errors::Error;
use crate::settings::Settings;

pub fn weight_record_c(record: &WeightRecord, settings: &Settings) -> gtk::Label {
    gtk::Label::new(Some(&settings.text.mass(record.weight.clone())))
}

pub fn weight_record_edit_c(
    id: UniqueId,
    record: WeightRecord,
    settings: &Settings,
    on_update: Box<dyn Fn(UniqueId, WeightRecord)>,
) -> gtk::Widget {
    //let b = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let u1 = settings.units.clone();
    let u2 = settings.units.clone();
    let entry = validated_text_entry_c(
        record.weight,
        Box::new(move |w| u1.render_mass(w.clone())),
        Box::new(move |s| {
            if s.len() == 0 {
                Err(Error::ParseMassError)
            } else {
                match u2.parse_mass(s) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(Error::ParseMassError),
                }
            }
        }),
        Box::new(move |val| on_update(id.clone(), WeightRecord::new(record.timestamp(), val))),
    );

    labeled_widget_c(
        &settings.text.weight(),
        labeled_widget_c(&settings.text.mass_label(), entry, LabelPosition::After),
        LabelPosition::Before,
    )

    //b.pack_start(&entry, false, false, 5);
    //b.pack_start(&units_label, false, false, 5);
    //b
}
