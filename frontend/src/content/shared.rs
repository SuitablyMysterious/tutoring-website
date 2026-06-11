//! Content shapes reused across pages.

/// A call-to-action link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cta {
    pub label: &'static str,
    pub href: &'static str,
}

/// A headline statistic ("8+ Years Experience").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stat {
    pub number: &'static str,
    pub label: &'static str,
}

/// A service shown as a card on the home and services pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceSummary {
    pub title: &'static str,
    pub blurb: &'static str,
    pub href: &'static str,
}
