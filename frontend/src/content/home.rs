use super::shared::{Cta, Stat};

pub struct HomeContent {
    pub page_title: &'static str,
    pub hero_eyebrow: &'static str,
    pub hero_title: &'static str,
    pub hero_title_emphasis: &'static str,
    pub hero_subtitle: &'static str,
    pub stats: &'static [Stat],
    pub primary_cta: Cta,
    pub secondary_cta: Cta,
    pub services_heading: &'static str,
}

pub const HOME: HomeContent = HomeContent {
    page_title: "[Site name] — Home",
    hero_eyebrow: "[Subjects taught]",
    hero_title: "[Hero headline]",
    hero_title_emphasis: "[emphasised part]",
    hero_subtitle: "[One-or-two sentence pitch]",
    stats: &[
        Stat {
            number: "[n]",
            label: "[stat label]",
        },
        Stat {
            number: "[n]",
            label: "[stat label]",
        },
        Stat {
            number: "[n]",
            label: "[stat label]",
        },
    ],
    primary_cta: Cta {
        label: "[Primary CTA]",
        href: "/contact",
    },
    secondary_cta: Cta {
        label: "[Secondary CTA]",
        href: "/services",
    },
    services_heading: "[Services section heading]",
};
