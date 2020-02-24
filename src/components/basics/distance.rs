use dimensioned::si;

use super::validated_text_entry_c;
use crate::i18n::UnitSystem;

pub fn distance_c(distance: &si::Meter<f64>, units: &UnitSystem) -> gtk::Label {
    gtk::Label::new(Some(&format!(
        "{}",
        units.render_distance(distance.clone())
    )))
}

pub fn distance_edit_c(
    distance: &Option<si::Meter<f64>>,
    units: &UnitSystem,
    on_update: Box<dyn Fn(Option<si::Meter<f64>>)>,
) -> gtk::Widget {
    let u1 = units.clone();
    let u2 = units.clone();
    validated_text_entry_c(
        distance.clone(),
        Box::new(move |s| {
            let u1 = u1.clone();
            s.map(move |s_| u1.render_distance(s_.clone()))
                .unwrap_or(String::from(""))
        }),
        Box::new(move |s| {
            if s.len() == 0 {
                Ok(None)
            } else {
                u2.parse_distance(s).map(|v| Some(v))
            }
        }),
        on_update,
    )
}
