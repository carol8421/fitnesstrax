use dimensioned::si;

use super::validated_text_entry_c;
use crate::formats::Duration;

pub fn duration_c(duration: si::Second<f64>) -> gtk::Label {
    gtk::Label::new(Some(&format!("{}", Duration::new(duration))))
}

pub fn duration_edit_c(
    duration: &Option<si::Second<f64>>,
    on_update: Box<dyn Fn(Option<si::Second<f64>>)>,
) -> gtk::Entry {
    validated_text_entry_c(
        duration.clone(),
        Box::new(|s| {
            s.map(|s_| format!("{}", Duration::new(s_)))
                .unwrap_or(String::from(""))
        }),
        Box::new(|s| {
            if s.len() > 0 {
                s.parse::<Duration>().map(|v| Some(v.extract()))
            } else {
                Ok(None)
            }
        }),
        on_update,
    )
}
