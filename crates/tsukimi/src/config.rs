pub const GETTEXT_PACKAGE: &str = "tsukimi";

pub const VERSION: &str = compile_version();

use std::sync::LazyLock;

/// On Windows the app is installed under a prefix that is not known at compile
/// time, so we resolve the `share` directory relative to the executable. This
/// supports both an installed layout (`<prefix>/bin/rdplayer.exe` with
/// `<prefix>/share/...`) and a portable layout with `share/` next to the exe.
/// Returns `None` when it cannot be determined (e.g. not on Windows, or the
/// resources are not where we expect them).
#[cfg(target_os = "windows")]
fn windows_share_root() -> Option<std::path::PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let exedir = exe.parent()?;
    let candidates = [
        exedir.join("share"),
        exedir.parent().map(|p| p.join("share")).unwrap_or_default(),
    ];
    for base in candidates.into_iter().filter(|p| !p.as_os_str().is_empty()) {
        if base.join("rdplayer").join("tsukimi.gresource").exists() {
            return Some(base);
        }
    }
    None
}

static PKGDATADIR: LazyLock<String> = LazyLock::new(|| {
    if let Ok(env) = std::env::var("TSUKIMI_PKGDATADIR") {
        return env;
    }
    #[cfg(target_os = "windows")]
    {
        if let Some(share) = windows_share_root() {
            return share.join("rdplayer").to_string_lossy().into_owned();
        }
    }
    "/usr/share/rdplayer".to_string()
});

static LOCALEDIR: LazyLock<String> = LazyLock::new(|| {
    if let Ok(env) = std::env::var("TSUKIMI_LOCALEDIR") {
        return env;
    }
    #[cfg(target_os = "windows")]
    {
        if let Some(share) = windows_share_root() {
            return share.join("locale").to_string_lossy().into_owned();
        }
    }
    "/usr/share/locale".to_string()
});

/// Directory containing the compiled GSettings schemas (`gschemas.compiled`).
/// Only meaningful on Windows, where the schema dir lives next to the exe.
#[cfg(target_os = "windows")]
pub fn gsettings_schema_dir() -> Option<String> {
    windows_share_root().map(|share| {
        share
            .join("glib-2.0")
            .join("schemas")
            .to_string_lossy()
            .into_owned()
    })
}

/// Directory containing the GStreamer plugin DLLs (`lib/gstreamer-1.0`).
/// Only meaningful on Windows; consumed via `GST_PLUGIN_PATH`.
#[cfg(target_os = "windows")]
pub fn gstreamer_plugin_dir() -> Option<String> {
    windows_share_root()
        .and_then(|share| share.parent().map(|p| p.to_path_buf()))
        .map(|prefix| {
            prefix
                .join("lib")
                .join("gstreamer-1.0")
                .to_string_lossy()
                .into_owned()
        })
}

/// Path to the compiled GResource bundle (`tsukimi.gresource`).
pub fn pkgdatadir() -> &'static str {
    PKGDATADIR.as_str()
}

/// Path to the gettext translation catalog directory.
pub fn localedir() -> &'static str {
    LOCALEDIR.as_str()
}

pub const fn compile_version() -> &'static str {
    const fn fallback(version: Option<&'static str>) -> &'static str {
        match version {
            Some(version) => version,
            None => env!("CARGO_PKG_VERSION"),
        }
    }

    if option_env!("TSUKIMI_RELEASE").is_none() {
        return fallback(option_env!("TSUKIMI_GIT_VERSION"));
    };

    fallback(option_env!("TSUKIMI_VERSION"))
}

pub const fn version() -> &'static str {
    VERSION
}
