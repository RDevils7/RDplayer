use std::{
    env,
    sync::LazyLock,
};

mod app;
mod arg;
mod config;
mod gstl;
mod macros;

mod mpris_common;
mod ui;
mod utils;

pub mod client;

pub use arg::Args;
use clap::Parser;
pub use config::*;
use gettextrs::*;
use gtk::prelude::*;

pub use ui::Window;

pub use app::TsukimiApplication as Application;

use crate::client::runtime::runtime;

pub static USER_AGENT: LazyLock<String> =
    LazyLock::new(|| format!("{}/{} - {}", CLIENT_ID, version(), env::consts::OS));

pub const APP_ID: &str = "moe.tsuna.tsukimi";
pub const CLIENT_ID: &str = "RDplayer";
const APP_RESOURCE_PATH: &str = "/moe/tsuna/tsukimi";
const GRESOURCE_FILE: &str = "tsukimi.gresource";

pub fn run() -> gtk::glib::ExitCode {
    Args::parse().init();

    // Initialize gettext
    unsafe { setlocale(LocaleCategory::LcAll, String::new()) };
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8").expect("Failed to set textdomain codeset");
    bindtextdomain(GETTEXT_PACKAGE, localedir())
        .expect("Invalid argument passed to bindtextdomain");

    textdomain(GETTEXT_PACKAGE).expect("Invalid string passed to textdomain");

    // On Windows the GSettings schema compiler output lives next to the exe, so
    // point GSettings (and GStreamer) at the bundled runtime dirs before they
    // are needed.
    #[cfg(target_os = "windows")]
    {
        if std::env::var("GSETTINGS_SCHEMA_DIR").is_err() {
            if let Some(dir) = crate::config::gsettings_schema_dir() {
                std::env::set_var("GSETTINGS_SCHEMA_DIR", dir);
            }
        }
        if std::env::var("GST_PLUGIN_PATH").is_err() {
            if let Some(dir) = crate::config::gstreamer_plugin_dir() {
                std::env::set_var("GST_PLUGIN_PATH", dir);
            }
        }
    }

    adw::init().expect("Failed to initialize Adwaita");
    mutsumi::init();

    register_gio_resources();

    ui::init();

    gtk::glib::set_application_name(CLIENT_ID);

    let _tokio_guard = runtime().enter();
    Application::new().run_with_args::<&str>(&[])
}

fn register_gio_resources() {
    let path = std::path::Path::new(pkgdatadir()).join(GRESOURCE_FILE);
    let resources = gtk::gio::Resource::load(path).expect("Failed to load resources.");
    gtk::gio::resources_register(&resources);
}
