//! Generates `style/main.css` from the theme tokens + handwritten base styles.
//!
//! `theme/theme.toml` -> `:root { --<section>-<key>: <value>; ... }`
//! concatenated with `style/base.css`.
//!
//! main.css is generated but COMMITTED: cargo-leptos reads it independently
//! of the cargo builds, so it must exist on a fresh checkout before any
//! build script has run. Edit theme.toml or base.css, rebuild, and commit
//! the regenerated main.css (CI fails if it's stale). Never edit it by hand.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=theme/theme.toml");
    println!("cargo:rerun-if-changed=style/base.css");
    // Also rerun if the output is deleted; without this, cargo considers the
    // script fresh and would never recreate the file.
    println!("cargo:rerun-if-changed=style/main.css");

    let theme_src = fs::read_to_string("theme/theme.toml").expect("theme/theme.toml missing");
    let theme: toml::Table = theme_src
        .parse()
        .expect("theme/theme.toml is not valid TOML");

    let mut css = String::from(
        "/* GENERATED FILE - do not edit. Edit theme/theme.toml or style/base.css. */\n:root {\n",
    );

    for (section, values) in &theme {
        let table = values
            .as_table()
            .unwrap_or_else(|| panic!("[{section}] must be a table of string values"));
        for (key, value) in table {
            let value = value
                .as_str()
                .unwrap_or_else(|| panic!("{section}.{key} must be a string"));
            writeln!(css, "  --{section}-{key}: {value};").unwrap();
        }
    }
    css.push_str("}\n\n");

    css.push_str(&fs::read_to_string("style/base.css").expect("style/base.css missing"));

    let out = Path::new("style/main.css");
    // Avoid rewriting (and re-triggering cargo-leptos' style watcher) when
    // nothing changed.
    if fs::read_to_string(out).ok().as_deref() != Some(&css) {
        fs::write(out, css).expect("failed to write style/main.css");
    }
}
