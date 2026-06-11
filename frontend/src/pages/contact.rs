use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::section::Section;
use crate::content::contact::CONTACT;
use crate::islands::enquiry_form::EnquiryForm;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <Title text=CONTACT.page_title />
        <Section title=CONTACT.heading>
            <p>{CONTACT.intro}</p>
            <EnquiryForm />
            // noscript / wasm-failure fallback so the page degrades gracefully
            <p>"Or email " <a href=format!("mailto:{}", CONTACT.fallback_email)>{CONTACT.fallback_email}</a></p>
        </Section>
    }
}
