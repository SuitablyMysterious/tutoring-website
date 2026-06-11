use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::section::Section;
use crate::content::about::ABOUT;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text=ABOUT.page_title />
        <Section title=ABOUT.heading>
            {ABOUT
                .paragraphs
                .iter()
                .map(|paragraph| view! { <p>{*paragraph}</p> })
                .collect_view()}
        </Section>
    }
}
