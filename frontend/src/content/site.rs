//! Site-wide strings: name, navigation, footer.

use super::shared::Cta;

pub const SITE_NAME: &str = "[Site name]";

pub const NAV_LINKS: &[Cta] = &[
    Cta {
        label: "Home",
        href: "/",
    },
    Cta {
        label: "About",
        href: "/about",
    },
    Cta {
        label: "Services",
        href: "/services",
    },
    Cta {
        label: "Contact",
        href: "/contact",
    },
];

pub const FOOTER_NOTE: &str = "[Footer note — e.g. copyright and DBS statement]";
