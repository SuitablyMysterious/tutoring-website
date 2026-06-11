//! Guards the theme contract: base.css may only reference CSS custom
//! properties that build.rs will actually generate from theme/theme.toml.
//! Catches token typos at test time instead of as silently-broken styles.

use std::collections::HashSet;

fn defined_tokens() -> HashSet<String> {
    let theme: toml::Table = include_str!("../theme/theme.toml")
        .parse()
        .expect("theme/theme.toml is not valid TOML");

    let mut tokens = HashSet::new();
    for (section, values) in &theme {
        let table = values
            .as_table()
            .unwrap_or_else(|| panic!("[{section}] must be a table"));
        // Mirrors build.rs: section name is the variable prefix verbatim.
        for (key, value) in table {
            assert!(
                value.is_str(),
                "{section}.{key} must be a string, build.rs would panic on it"
            );
            tokens.insert(format!("--{section}-{key}"));
        }
    }
    tokens
}

fn strip_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        match rest[start..].find("*/") {
            Some(end) => rest = &rest[start + end + 2..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

#[test]
fn base_css_only_references_defined_theme_tokens() {
    let tokens = defined_tokens();
    assert!(!tokens.is_empty(), "theme.toml defines no tokens");

    let css = strip_comments(include_str!("../style/base.css"));
    let mut rest = css.as_str();
    let mut found_any = false;
    while let Some(pos) = rest.find("var(") {
        rest = &rest[pos + "var(".len()..];
        let end = rest
            .find([')', ','])
            .expect("unterminated var() in style/base.css");
        let name = rest[..end].trim();
        assert!(
            tokens.contains(name),
            "style/base.css references {name}, which theme/theme.toml does not define"
        );
        found_any = true;
    }
    assert!(
        found_any,
        "base.css uses no theme tokens at all — suspicious"
    );
}
