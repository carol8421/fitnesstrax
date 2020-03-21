use chrono::Utc;
use glib::Sender;
use std::path::PathBuf;

use crate::config::Configuration;
use crate::errors::{Error, Result};
use crate::i18n::{Text, UnitSystem};
use crate::range::Range;
use crate::settings::Settings;
use crate::types::DateRange;
use emseries::{DateTimeTz, UniqueId};
use fitnesstrax::{Trax, TraxRecord};

#[derive(Clone, Debug)]
pub enum Message {
    ChangeRange {
        range: DateRange,
        records: Vec<(UniqueId, TraxRecord)>,
    },
    ChangeLanguage(Text),
    ChangeTimezone(chrono_tz::Tz),
    ChangeUnits(UnitSystem),
    RecordsUpdated(Vec<(UniqueId, TraxRecord)>),
}

pub struct Application {
    channel: Sender<Message>,
    state: State,
}

pub enum State {
    Unconfigured(Unconfigured),
    Configured(Configured),
}

pub struct Unconfigured {
    settings: Settings,
    series_path: Option<PathBuf>,
}

pub struct Configured {
    settings: Settings,
    series_path: PathBuf,
    trax: Trax,
    range: DateRange,
}

impl State {
    pub fn series_path(&self) -> Option<&PathBuf> {
        match self {
            State::Unconfigured(Unconfigured { series_path, .. }) => series_path.as_ref(),
            State::Configured(Configured { series_path, .. }) => Some(&series_path),
        }
    }

    pub fn settings(&self) -> &Settings {
        match self {
            State::Unconfigured(Unconfigured { settings, .. }) => &settings,
            State::Configured(Configured { settings, .. }) => &settings,
        }
    }

    pub fn text(&self) -> &Text {
        match self {
            State::Unconfigured(Unconfigured { settings, .. }) => &settings.text,
            State::Configured(Configured { settings, .. }) => &settings.text,
        }
    }

    fn set_language(&mut self, language_str: &str) {
        match self {
            State::Unconfigured(Unconfigured { settings, .. }) => {
                settings.set_language(language_str)
            }
            State::Configured(Configured { settings, .. }) => settings.set_language(language_str),
        }
    }

    fn set_timezone(&mut self, timezone: chrono_tz::Tz) {
        match self {
            State::Unconfigured(Unconfigured {
                ref mut settings, ..
            }) => settings.set_timezone(timezone),
            State::Configured(Configured {
                ref mut settings, ..
            }) => settings.set_timezone(timezone),
        }
    }

    fn set_units(&mut self, units_str: &str) {
        match self {
            State::Unconfigured(Unconfigured {
                ref mut settings, ..
            }) => settings.set_units(units_str),
            State::Configured(Configured {
                ref mut settings, ..
            }) => settings.set_units(units_str),
        }
    }
}

impl Configured {
    pub fn range(&self) -> DateRange {
        self.range.clone()
    }

    pub fn get_history(&self) -> Result<Vec<(UniqueId, TraxRecord)>> {
        let start_time = DateTimeTz(
            self.range
                .start
                .and_hms(0, 0, 0)
                .with_timezone(&self.settings.timezone),
        );
        let end_time = DateTimeTz(
            (self.range.end + chrono::Duration::days(1))
                .and_hms(0, 0, 0)
                .with_timezone(&self.settings.timezone),
        );
        self.trax
            .get_history(start_time, end_time)
            .map(|v| {
                v.iter()
                    .map(|(ref id, ref record)| ((*id).clone(), (*record).clone()))
                    .collect()
            })
            .map_err(|err| Error::TraxError(err))
    }

    pub fn save_records(
        &mut self,
        updated_records: Vec<(UniqueId, TraxRecord)>,
        new_records: Vec<TraxRecord>,
    ) {
        for (id, record) in updated_records {
            let _ = self.trax.replace_record(id, record);
        }
        for record in new_records {
            let _ = self.trax.add_record(record);
        }
    }

    pub fn set_range(&mut self, range: DateRange) {
        self.range = range;
    }

    pub fn text(&self) -> &Text {
        &self.settings.text
    }

    pub fn timezone(&self) -> &chrono_tz::Tz {
        &self.settings.timezone
    }

    pub fn units(&self) -> &UnitSystem {
        &self.settings.units
    }
}

impl Application {
    pub fn new(channel: Sender<Message>) -> Result<Application> {
        let config = Configuration::load_from_yaml();

        let range = Range::new(
            Utc::now().with_timezone(&config.timezone).date() - chrono::Duration::days(7),
            Utc::now().with_timezone(&config.timezone).date(),
        );

        let settings = Settings::from_config(&config);

        let state = if let Some(ref path) = config.series_path {
            State::Configured(Configured {
                trax: fitnesstrax::Trax::new(fitnesstrax::Params {
                    series_path: path.clone(),
                })
                .unwrap(),
                series_path: path.clone(),
                range,
                settings,
            })
        } else {
            State::Unconfigured(Unconfigured {
                series_path: None,
                settings,
            })
        };

        Ok(Application { channel, state })
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn set_series_path(&mut self, path: &str) {
        let trax = fitnesstrax::Trax::new(fitnesstrax::Params {
            series_path: PathBuf::from(path),
        })
        .unwrap();

        let range = Range::new(
            Utc::now()
                .with_timezone(&self.state.settings().timezone)
                .date()
                - chrono::Duration::days(7),
            Utc::now()
                .with_timezone(&self.state.settings().timezone)
                .date(),
        );

        self.state = match self.state {
            State::Unconfigured(Unconfigured { ref settings, .. }) => {
                State::Configured(Configured {
                    trax,
                    series_path: PathBuf::from(path),
                    range,
                    settings: settings.clone(),
                })
            }
            State::Configured(Configured {
                ref range,
                ref settings,
                ..
            }) => State::Configured(Configured {
                trax,
                series_path: PathBuf::from(path),
                range: range.clone(),
                settings: settings.clone(),
            }),
        }
    }

    pub fn set_language(&mut self, language_str: &str) {
        self.state.set_language(language_str);
        if let State::Configured(ref state) = self.state {
            self.send_notifications(Message::ChangeLanguage(state.settings.text.clone()));
        }
    }

    pub fn set_timezone(&mut self, timezone: chrono_tz::Tz) {
        self.state.set_timezone(timezone);
        if let State::Configured(ref state) = self.state {
            self.send_notifications(Message::ChangeTimezone(state.settings.timezone.clone()));
        }
    }

    pub fn set_units(&mut self, units_str: &str) {
        self.state.set_units(units_str);
        if let State::Configured(ref state) = self.state {
            self.send_notifications(Message::ChangeUnits(state.settings.units.clone()));
        }
    }

    pub fn save_records(
        &mut self,
        updated_records: Vec<(UniqueId, TraxRecord)>,
        new_records: Vec<TraxRecord>,
    ) -> Result<()> {
        match self.state {
            State::Unconfigured(_) => Err(Error::SeriesNotOpen),
            State::Configured(ref mut state) => {
                state.save_records(updated_records, new_records);
                Ok(())
            }
        }?;
        match self.state {
            State::Unconfigured(_) => Err(Error::SeriesNotOpen),
            State::Configured(ref state) => {
                self.send_notifications(Message::RecordsUpdated(
                    state.get_history().unwrap().clone(),
                ));
                Ok(())
            }
        }
    }

    pub fn set_range(&mut self, range: DateRange) -> Result<()> {
        match self.state {
            State::Unconfigured(_) => Err(Error::SeriesNotOpen),
            State::Configured(ref mut state) => {
                state.set_range(range);
                Ok(())
            }
        }?;
        match self.state {
            State::Unconfigured(_) => Err(Error::SeriesNotOpen),
            State::Configured(ref state) => {
                self.send_notifications(Message::ChangeRange {
                    range: state.range().clone(),
                    records: state.get_history().unwrap().clone(),
                });
                Ok(())
            }
        }
    }

    fn send_notifications(&self, msg: Message) {
        //println!("dispatching message: {:?}", msg);
        self.channel.send(msg).unwrap();
    }
}
