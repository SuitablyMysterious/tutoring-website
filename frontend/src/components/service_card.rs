use leptos::prelude::*;

use crate::content::shared::ServiceSummary;

#[component]
pub fn ServiceCard(service: &'static ServiceSummary) -> impl IntoView {
    view! {
        <article class="card">
            <h3>
                <a href=service.href>{service.title}</a>
            </h3>
            <p>{service.blurb}</p>
        </article>
    }
}
