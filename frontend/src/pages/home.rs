use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::{hero::Hero, section::Section, service_card::ServiceCard};
use crate::content::{home::HOME, services::SERVICES};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text=HOME.page_title />
        <Hero
            eyebrow=HOME.hero_eyebrow
            title=HOME.hero_title
            emphasis=HOME.hero_title_emphasis
            subtitle=HOME.hero_subtitle
            stats=HOME.stats
            primary_cta=HOME.primary_cta
            secondary_cta=HOME.secondary_cta
        />
        <Section title=HOME.services_heading>
            <div class="card-grid">
                {SERVICES
                    .iter()
                    .map(|service| view! { <ServiceCard service=service /> })
                    .collect_view()}
            </div>
        </Section>
    }
}
