pub struct AboutContent {
    pub page_title: &'static str,
    pub heading: &'static str,
    pub paragraphs: &'static [&'static str],
}

pub const ABOUT: AboutContent = AboutContent {
    page_title: "[Site name] — About",
    heading: "[About heading]",
    paragraphs: &["[About paragraph 1]", "[About paragraph 2]"],
};
