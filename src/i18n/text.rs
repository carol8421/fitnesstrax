use dimensioned::si::Kilogram;
use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use std::fmt;
use std::sync::Arc;
use unic_langid::LanguageIdentifier;

use crate::i18n::UnitSystem;
use fitnesstrax_lib::timedistance;

const ENGLISH_STRINGS: &str = "
about = About
activity = Activity
add-time-distance-workout = Add Time/Distance Workout
cancel = Cancel
cycling = Cycling
database-path = Database Path
dependencies = Dependencies
  chrono-tz 0.4, MIT/Apache-2.0, Djzin
  chrono 0.4, MIT/Apache-2.0, Brandon W. Maister, Kang Seonghoon
  dimensioned 0.7.0, MIT/Apache-2.0, Paho Lurie-Gregg
  emseries 0.5.0, BSD-3-Clause, Savanni D'Gerine
  fluent 0.9.1, Apache-2.0/MIT, Staś Małolepszy, Zibi Braniecki
  gio, MIT, The Gtk-rs Project Developers
  glib 0.9.0, MIT, The Gtk-rs Project Developers
  gtk 0.8.0, MIT, The Gtk-rs Project Developers
  serde 1, MIT/Apache-2.0, David Tolnay, Erick Tryzelaar
  serde_yaml, MIT/Apache-2.0, David Tolnay
  tzdata 0.4.1, MIT, Maxime Lenoir
  unic-langid 0.7.1, MIT/Apache-2.0, Zibi Braniecki
edit = Edit
enter-distance = Enter distance
enter-duration = Enter duration
enter-time = Enter time
health-tracker = Health Tracker
history = History
language = Language
license = Copyright Savanni D'Gerinel (c) 2018-2020
  All rights reserved.

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions are met:

  - Redistributions of source code must retain the above copyright
    notice, this list of conditions and the following disclaimer.

  - Redistributions in binary form must reproduce the above
    copyright notice, this list of conditions and the following
    disclaimer in the documentation and/or other materials provided
    with the distribution.

  - Neither the name of Savanni D'Gerinel nor the names of other
    contributors may be used to endorse or promote products derived
    from this software without specific prior written permission.

  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
  \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
  LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
  A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
  OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
  LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
  DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
  THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
  (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
  OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
mass = {$units ->
    *[SI] {$value} kilograms
    [USA] {$value} pounds
}
mass-label = {$units ->
    *[SI] kilograms
    [USA] pounds
}
name-with-version = FitnessTrax, version 0.1
preferences = Preferences
pushups = Pushups
rowing = Rowing
running = Running
save = Save
situps = Situps
steps = Steps
step-count = {$count ->
    [one] 1 step
    *[other] {$count} steps
}
swimming = Swimming
timezone = Timezone
units = Units
walking = Walking
weight = Weight
";

const ESPERANTO_STRINGS: &str = "
about = Pri
add-time-distance-workout = Aldonu Trejnadon de Daŭro/Distanco
cancel = Nuligi
cycling = Biciklado
database-path = Vojo al Datumbazo
edit = Redaktu
enter-distance = Eniru distanco
enter-duration = Eniru daŭro
enter-time = Eniru tempon
health-tracker = Sana Supuristo
history = Historio
language = Lingvo
mass = {$units ->
    *[SI] {$value} kilogramoj
    [USA] {$value} funtoj
}
mass-label = {$units ->
    *[SI] kilogramoj
    [USA] funtoj
}
preferences = Agdoroj
pushups = Supraj Puŝoj
rowing = Remado
running = Kurado
save = Ŝpari
situps = Sidiĝoj
steps = Paŝoj
step-count = {$count ->
    [one] 1 paŝo
    *[other] {$count} paŝoj
}
swimming = Naĝado
timezone = Horzono
units = Unuoj
walking = Promenadi
weight = Pezo
";

#[derive(Clone)]
pub struct Text {
    language: LanguageIdentifier,
    bundle: Arc<FluentBundle<FluentResource>>,
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Text {{ language: {}, units: {} }}",
            self.language, "whatever, for the moment"
        )
    }
}

fn add_language(bundle: &mut FluentBundle<FluentResource>, langid: &LanguageIdentifier) {
    let lang_resource = match langid.get_language() {
        "en" => FluentResource::try_new(String::from(ENGLISH_STRINGS)),
        "eo" => FluentResource::try_new(String::from(ESPERANTO_STRINGS)),
        _ => panic!("unsupported language"),
    };
    match lang_resource {
        Ok(res) => {
            let _ = bundle.add_resource(res);
        }
        Err(err) => panic!("{:?}", err),
    }
}

impl Text {
    pub fn new(langid: LanguageIdentifier) -> Text {
        let english_id: LanguageIdentifier = "en".parse().unwrap();
        let mut bundle = FluentBundle::new(&[langid.clone(), english_id.clone()]);

        add_language(&mut bundle, &langid);
        add_language(&mut bundle, &english_id);

        Text {
            language: langid.clone(),
            bundle: Arc::new(bundle),
        }
    }

    pub fn language_id(&self) -> &LanguageIdentifier {
        &self.language
    }

    pub fn about<'s>(&self) -> String {
        self.tr("about", None).unwrap()
    }

    pub fn activity<'s>(&self) -> String {
        self.tr("activity", None).unwrap()
    }

    pub fn add_time_distance_workout(&self) -> String {
        self.tr("add-time-distance-workout", None).unwrap()
    }

    pub fn cancel(&self) -> String {
        self.tr("cancel", None).unwrap()
    }

    pub fn cycling(&self) -> String {
        self.tr("cycling", None).unwrap()
    }

    pub fn database_path(&self) -> String {
        self.tr("database-path", None).unwrap()
    }

    pub fn dependencies(&self) -> String {
        self.tr("dependencies", None).unwrap()
    }

    pub fn edit(&self) -> String {
        self.tr("edit", None).unwrap()
    }

    pub fn history(&self) -> String {
        self.tr("history", None).unwrap()
    }

    pub fn language(&self) -> String {
        self.tr("language", None).unwrap()
    }

    pub fn license(&self) -> String {
        self.tr("license", None).unwrap()
    }

    pub fn mass(&self, value: Kilogram<f64>, units: &UnitSystem) -> String {
        let mut args = FluentArgs::new();
        args.insert("value", FluentValue::from(units.render_mass(value)));
        args.insert("units", FluentValue::from(String::from(units)));

        self.tr("mass", Some(&args)).unwrap()
    }

    pub fn mass_label(&self) -> String {
        self.tr("mass-label", None).unwrap()
    }

    pub fn name_with_version(&self) -> String {
        self.tr("name-with-version", None).unwrap()
    }

    pub fn preferences(&self) -> String {
        self.tr("preferences", None).unwrap()
    }

    pub fn rowing(&self) -> String {
        self.tr("rowing", None).unwrap()
    }

    pub fn running(&self) -> String {
        self.tr("running", None).unwrap()
    }

    pub fn save(&self) -> String {
        self.tr("save", None).unwrap()
    }

    pub fn step_count(&self, count: u32) -> String {
        let mut _errors = vec![];

        let mut args = FluentArgs::new();
        args.insert("count", FluentValue::from(count));

        self.bundle
            .get_message("step-count")
            .and_then(|msg| msg.value)
            .map(move |pattern| {
                String::from(
                    self.bundle
                        .format_pattern(&pattern, Some(&args), &mut _errors),
                )
            })
            .unwrap()
    }

    pub fn steps_label(&self) -> String {
        self.tr("steps", None).unwrap()
    }

    pub fn swimming(&self) -> String {
        self.tr("swimming", None).unwrap()
    }

    pub fn timezone<'s>(&'s self) -> String {
        self.tr("timezone", None).unwrap()
    }

    pub fn time_distance_activity<'s>(&'s self, activity: &timedistance::ActivityType) -> String {
        match activity {
            timedistance::ActivityType::Cycling => self.tr("cycling", None),
            timedistance::ActivityType::Rowing => self.tr("rowing", None),
            timedistance::ActivityType::Running => self.tr("running", None),
            timedistance::ActivityType::Swimming => self.tr("swimming", None),
            timedistance::ActivityType::Walking => self.tr("walking", None),
        }
        .unwrap()
    }

    pub fn units(&self) -> String {
        self.tr("units", None).unwrap()
    }

    pub fn walking(&self) -> String {
        self.tr("walking", None).unwrap()
    }

    pub fn tr(&self, id: &str, args: Option<&FluentArgs>) -> Option<String> {
        let mut _errors = vec![];

        self.bundle
            .get_message(id)
            .and_then(|msg| msg.value)
            .map(|pattern| String::from(self.bundle.format_pattern(&pattern, args, &mut _errors)))
    }
}

#[cfg(test)]
mod test {
    use super::Text;

    use crate::i18n::UnitSystem;

    #[test]
    fn translations_work() {
        let en = Text::new("en-US".parse().unwrap());
        assert_eq!(en.preferences(), "Preferences");

        let eo = Text::new("eo".parse().unwrap());
        assert_eq!(eo.preferences(), "Agdoroj");
        assert_eq!(eo.history(), "Historio");
    }
}
