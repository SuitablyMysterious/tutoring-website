pub struct ContactContent {
    pub page_title: &'static str,
    pub heading: &'static str,
    pub intro: &'static str,
    /// Shown when the enquiry API is unavailable, so the page still works
    /// if the backend is down or wasm fails to load.
    pub fallback_email: &'static str,
}

pub const CONTACT: ContactContent = ContactContent {
    page_title: "[Site name] — Contact",
    heading: "[Contact heading]",
    intro: "[Short invitation to get in touch]",
    fallback_email: "[contact email]",
};
