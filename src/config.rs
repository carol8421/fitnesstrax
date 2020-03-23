use gio::{Settings, SettingsExt};
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::borrow::Cow;
use std::convert::From;
use std::convert::TryFrom;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path;
use unic_langid::LanguageIdentifier;

use crate::i18n::UnitSystem;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub series_path: Option<path::PathBuf>,
    pub language: LanguageId,
    pub timezone: chrono_tz::Tz,
    pub units: UnitSystem,
}

impl Configuration {
    pub fn load_from_yaml() -> Configuration {
        let config_path = env::var("CONFIG").unwrap_or("config.yaml".to_string());
        let config_file = File::open(config_path.clone());
        match config_file {
            Ok(file) => serde_yaml::from_reader(file).expect("config file could not be parsed"),
            Err(_) => Configuration {
                series_path: None,
                language: LanguageId("en".parse().expect("hard-coded en should always parse")),
                timezone: chrono_tz::Etc::UTC,
                units: UnitSystem::SI,
            },
        }
    }

    pub fn save_to_yaml(&self) {
        let config_path = env::var("CONFIG").unwrap_or("config.yaml".to_string());
        let s = serde_yaml::to_string(&self).unwrap();
        let mut config_file = File::create(config_path.clone())
            .expect(&format!("cannot open configuration file: {}", &config_path));
        let _ = config_file.write(s.as_bytes());
    }

    pub fn load_from_gsettings() -> Configuration {
        let s = Settings::new("com.luminescent-dreams.fitnesstrax");

        Configuration {
            series_path: s.get_string("series-path").and_then(|s| {
                if s.as_str() == "" {
                    None
                } else {
                    Some(path::PathBuf::from(s.as_str()))
                }
            }),
            language: LanguageId(
                s.get_string("language")
                    .unwrap()
                    .parse()
                    .expect("language strings"),
            ),
            timezone: s
                .get_string("timezone")
                .unwrap()
                .as_str()
                .parse::<chrono_tz::Tz>()
                .unwrap(),
            units: UnitSystem::try_from(s.get_string("units").unwrap().as_str()).unwrap(),
        }
    }

    pub fn save_to_gsettings(&self) {
        let s = Settings::new("com.luminescent-dreams.fitnesstrax");
        s.delay();
        s.set_string(
            "series-path",
            &self
                .series_path
                .clone()
                .map(|p| String::from(p.to_string_lossy()))
                .unwrap_or(String::from("")),
        );
        s.set_string("language", self.language.get_language());
        s.set_string("timezone", self.timezone.name());
        s.set_string("units", &String::from(&self.units));
        s.apply();
    }
}

#[derive(Clone, Debug)]
pub struct LanguageId(LanguageIdentifier);

impl Deref for LanguageId {
    type Target = LanguageIdentifier;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&LanguageId> for LanguageIdentifier {
    fn from(lang: &LanguageId) -> LanguageIdentifier {
        lang.0.clone()
    }
}

impl From<&LanguageIdentifier> for LanguageId {
    fn from(lang: &LanguageIdentifier) -> LanguageId {
        LanguageId(lang.clone())
    }
}

impl Serialize for LanguageId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0.get_language())
    }
}

struct LanguageIdVisitor;

impl<'de> Visitor<'de> for LanguageIdVisitor {
    type Value = LanguageId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation for a standard language identifier")
    }

    fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
        s.parse().map(LanguageId).or(Err(E::custom(format!(
            "string is not a parsable language identifier"
        ))))
    }
}

impl<'de> Deserialize<'de> for LanguageId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(LanguageIdVisitor)
    }
}
