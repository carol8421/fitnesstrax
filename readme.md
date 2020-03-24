# Fitnesstrax

Privacy-first fitness tracking. Your health, your data, your machine.

This application is deeply inspired by the work that I did on [an older health application](https://github.com/savannidgerinel/health). It will eventually be a full replacement for that application, but with improved architecture from several years of learning.

The data files for this application are built atop [emseries](), and thus should be easily readable. Please refer to that project for the time series file format. The individual records relevant to this application will be documented one day in the future.

## Installing

This application does not have proper installers for anything other than NixOS at this time. While `cargo install` will mostly work, it is still important do get the GTK resources into hte path.

Before running the application, create a blank file that can serve as the database. The UI is not currently able to do an "open or create" type of behavior.

## A word on timezones

Only a few timezones are currently included in the application. I have put no effort yet into providing the full list of timezones that can be supported. If you have a particular timezone that you would like to see added, file an issue on the project. Ideally, please report your timezone from the [TZ Database Name](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones) column.

Alternatively, if you would like to implement the full timezone listing, please contact me and we can discuss what is involved.

## Translations

Please contact me if you believe the English or Esperanto translations could be better.

Please contact me if you are interested in adding a language translation for the application.
