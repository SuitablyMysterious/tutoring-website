//! Server-render smoke tests: each page renders without panicking and
//! contains its content. Run with `cargo test -p tutoring-website-frontend
//! --features ssr`. These deliberately assert on content/structure, not
//! exact markup, so styling and layout changes don't break them.

use leptos::prelude::*;

use crate::content::{about::ABOUT, contact::CONTACT, home::HOME, services::SERVICES_PAGE};
use crate::islands::enquiry_form::EnquiryForm;
use crate::pages::{
    about::AboutPage, contact::ContactPage, home::HomePage, not_found::NotFoundPage,
    services::ServicesPage,
};

fn render<V: IntoView + 'static>(f: impl FnOnce() -> V) -> String {
    let owner = Owner::new();
    owner.with(|| {
        leptos_meta::provide_meta_context();
        f().to_html()
    })
}

#[test]
fn home_page_renders_hero_and_services() {
    let html = render(|| view! { <HomePage /> });
    assert!(html.contains(HOME.hero_title));
    assert!(html.contains(HOME.hero_title_emphasis));
    assert!(html.contains(HOME.primary_cta.label));
    assert!(html.contains(HOME.services_heading));
}

#[test]
fn about_page_renders_all_paragraphs() {
    let html = render(|| view! { <AboutPage /> });
    for paragraph in ABOUT.paragraphs {
        assert!(html.contains(paragraph));
    }
}

#[test]
fn services_page_renders_every_service_card() {
    let html = render(|| view! { <ServicesPage /> });
    assert!(html.contains(SERVICES_PAGE.intro));
    for service in crate::content::services::SERVICES {
        assert!(html.contains(service.title));
        assert!(html.contains(service.blurb));
    }
}

#[test]
fn contact_page_renders_form_and_email_fallback() {
    let html = render(|| view! { <ContactPage /> });
    // The non-wasm fallback must always be in the server HTML.
    assert!(html.contains(&format!("mailto:{}", CONTACT.fallback_email)));
    assert!(html.contains("enquiry-form"));
}

#[test]
fn enquiry_form_keeps_its_honeypot() {
    let html = render(|| view! { <EnquiryForm /> });
    assert!(html.contains("enquiry-form"));
    // The spam honeypot must stay in the markup and stay hidden.
    assert!(html.contains("hp-field"));
    assert!(html.contains("aria-hidden"));
}

#[test]
fn not_found_page_links_home() {
    let html = render(|| view! { <NotFoundPage /> });
    assert!(html.contains(r#"href="/""#));
}
