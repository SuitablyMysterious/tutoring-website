use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::{section::Section, service_card::ServiceCard};
use crate::content::services::{SERVICES, SERVICES_PAGE};

#[component]
pub fn ServicesPage() -> impl IntoView {
    view! {
        <Title text=SERVICES_PAGE.page_title />
        <Section title=SERVICES_PAGE.heading>
            <p>{SERVICES_PAGE.intro}</p>
            <div class="card-grid">
                {SERVICES
                    .iter()
                    .map(|service| view! { <ServiceCard service=service /> })
                    .collect_view()}
            </div>
        </Section>
    }
}
