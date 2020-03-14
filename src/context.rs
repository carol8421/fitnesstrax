use chrono::Utc;
use glib::Sender;
use std::path::PathBuf;

use crate::config::Configuration;
use crate::errors::{Error, Result};
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
    ChangeSettings {
        settings: Settings,
    },
    RecordsUpdated {
        records: Vec<(UniqueId, TraxRecord)>,
    },
}

pub struct AppContext {
    settings: Settings,
    series_path: Option<PathBuf>,
    trax: Option<Trax>,
    range: DateRange,
    channel: Sender<Message>,
}

impl AppContext {
    pub fn new(channel: Sender<Message>) -> Result<AppContext> {
        let config = Configuration::load_from_yaml();

        let trax = if let Some(ref path) = config.series_path {
            fitnesstrax::Trax::new(fitnesstrax::Params {
                series_path: path.clone(),
            })
            .map(Some)
        } else {
            Ok(None)
        }?;

        let range = Range::new(
            Utc::now().with_timezone(&config.timezone).date() - chrono::Duration::days(7),
            Utc::now().with_timezone(&config.timezone).date(),
        );

        let settings = Settings::from_config(&config);

        Ok(AppContext {
            series_path: config.series_path,
            settings,
            trax,
            range,
            channel,
        })
    }

    pub fn get_series_path(&self) -> Option<&PathBuf> {
        self.series_path.as_ref()
    }

    pub fn set_series_path(&mut self, path: &str) {
        self.series_path = Some(PathBuf::from(path));
    }

    pub fn get_settings(&self) -> Settings {
        self.settings.clone()
    }

    pub fn set_language(&mut self, language_str: &str) {
        self.settings.set_language(language_str);
        self.send_notifications(Message::ChangeSettings {
            settings: self.settings.clone(),
        });
    }

    pub fn set_timezone(&mut self, timezone: chrono_tz::Tz) {
        self.settings.set_timezone(timezone);
        self.send_notifications(Message::ChangeSettings {
            settings: self.settings.clone(),
        });
    }

    pub fn set_units(&mut self, units_str: &str) {
        self.settings.set_units(units_str);
        self.send_notifications(Message::ChangeSettings {
            settings: self.settings.clone(),
        });
    }

    /*
    pub fn set_settings(&mut self, settings: Settings) {
        {
            if settings.text != self.settings.language {
                self.translations = Translations::new(&settings.language);
            }
            self.settings = settings;

            let config = Configuration {
                series_path: self.series_path.clone(),
                language: self.settings.language.clone(),
                timezone: self.settings.timezone,
                units: String::from(self.settings.units.clone()),
            };
            config.save_to_yaml();
        }
        self.send_notifications(Message::ChangePreferences {
            settings: self.settings.clone(),
            range: self.range.clone(),
            records: self.get_history().unwrap(),
        });
    }
    */

    pub fn get_range(&self) -> DateRange {
        self.range.clone()
    }

    pub fn get_history(&self) -> Result<Vec<(UniqueId, TraxRecord)>> {
        match self.trax {
            None => Err(Error::SeriesNotOpen),
            Some(ref trax) => {
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
                trax.get_history(start_time, end_time)
                    .map(|v| {
                        v.iter()
                            .map(|(ref id, ref record)| ((*id).clone(), (*record).clone()))
                            .collect()
                    })
                    .map_err(|err| Error::TraxError(err))
            }
        }
    }

    pub fn save_records(
        &mut self,
        updated_records: Vec<(UniqueId, TraxRecord)>,
        new_records: Vec<TraxRecord>,
    ) -> Result<()> {
        match self.trax {
            None => Err(Error::SeriesNotOpen),
            Some(ref mut trax) => {
                for (id, record) in updated_records {
                    let _ = trax.replace_record(id, record);
                }
                for record in new_records {
                    let _ = trax.add_record(record);
                }
                let history = self.get_history().unwrap();
                self.send_notifications(Message::RecordsUpdated { records: history });
                Ok(())
            }
        }
    }

    pub fn set_range(&mut self, range: DateRange) {
        self.range = range.clone();
        let history = self.get_history().unwrap();
        self.send_notifications(Message::ChangeRange {
            range,
            records: history,
        });
    }

    fn send_notifications(&self, msg: Message) {
        //println!("dispatching message: {:?}", msg);
        self.channel.send(msg).unwrap();
    }
}
