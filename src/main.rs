extern crate chrono;
extern crate chrono_tz;
extern crate fitnesstrax_lib;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate serde;

use gio::prelude::*;
use std::sync::{Arc, RwLock};

mod components;
mod config;
mod context;
mod errors;
mod formats;
mod i18n;
mod range;
mod settings;
mod types;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.luminescent-dreams.fitnesstrax"),
        Default::default(),
    )
    .expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let ctx = Arc::new(RwLock::new(context::Application::new(tx).unwrap()));
        let gui = Arc::new(RwLock::new(components::MainWindow::new(ctx.clone(), app)));

        let gui_clone = gui.clone();
        rx.attach(None, move |msg| {
            gui_clone.write().unwrap().update_from(msg);
            glib::Continue(true)
        });
    });

    application.run(&[]);
}
