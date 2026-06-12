//! Guards the theme contract: custom CSS in style/main.css may only
//! reference tokens that the @theme block actually defines (plus Tailwind's
//! own --default-* aliases). Catches token typos at test time instead of as
//! silently-broken styles.
//!
//! Note: token-derived *utility* classes used in components (bg-primary,
//! gap-md, ...) can't be cheaply verified here — Tailwind silently skips
//! unknown utilities — so renaming a token still warrants a visual check.

use std::collections::HashSet;

const MAIN_CSS: &str = include_str!("../style/main.css");

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

/// Extracts the body of the `@theme { ... }` block.
fn theme_block(css: &str) -> &str {
    let start = css
        .find("@theme")
        .expect("main.css must have a @theme block");
    let open = css[start..].find('{').expect("malformed @theme block") + start + 1;
    let mut depth = 1;
    for (i, c) in css[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &css[open..open + i];
                }
            }
            _ => {}
        }
    }
    panic!("unterminated @theme block");
}

fn defined_tokens(theme: &str) -> HashSet<&str> {
    theme
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            line.starts_with("--")
                .then(|| line.split(':').next().unwrap().trim())
        })
        .collect()
}

#[test]
fn custom_css_only_references_defined_theme_tokens() {
    let css = strip_comments(MAIN_CSS);
    let theme = theme_block(&css);
    let tokens = defined_tokens(theme);
    assert!(!tokens.is_empty(), "@theme block defines no tokens");

    // Everything after the @theme block is our custom CSS (base/components
    // layers); it must only var() into defined tokens.
    let custom = &css[css.find(theme).unwrap() + theme.len()..];
    let mut rest = custom;
    let mut found_any = false;
    while let Some(pos) = rest.find("var(") {
        rest = &rest[pos + "var(".len()..];
        let end = rest
            .find([')', ','])
            .expect("unterminated var() in style/main.css");
        let name = rest[..end].trim();
        assert!(
            tokens.contains(name),
            "style/main.css references {name}, which @theme does not define"
        );
        found_any = true;
    }
    assert!(
        found_any,
        "custom CSS uses no theme tokens at all — suspicious"
    );
}

#[test]
fn honeypot_styling_survives() {
    // The .hp-field off-screen rule is what keeps the spam honeypot
    // invisible to humans; losing it would show a junk field to users.
    let css = strip_comments(MAIN_CSS);
    let rule_start = css.find(".hp-field").expect(".hp-field rule missing");
    let rule = &css[rule_start..];
    assert!(rule.contains("absolute"), ".hp-field must stay off-screen");
}
