use super::shared::ServiceSummary;

pub struct ServicesContent {
    pub page_title: &'static str,
    pub heading: &'static str,
    pub intro: &'static str,
}

pub const SERVICES_PAGE: ServicesContent = ServicesContent {
    page_title: "[Site name] — Services",
    heading: "[Services heading]",
    intro: "[Short intro to the services offered]",
};

pub const SERVICES: &[ServiceSummary] = &[
    ServiceSummary {
        title: "[Service 1]",
        blurb: "[One-line description]",
        href: "/services",
    },
    ServiceSummary {
        title: "[Service 2]",
        blurb: "[One-line description]",
        href: "/services",
    },
    ServiceSummary {
        title: "[Service 3]",
        blurb: "[One-line description]",
        href: "/services",
    },
];
