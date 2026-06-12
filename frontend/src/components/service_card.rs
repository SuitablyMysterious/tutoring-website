use leptos::prelude::*;

use crate::content::shared::ServiceSummary;

#[component]
pub fn ServiceCard(service: &'static ServiceSummary) -> impl IntoView {
    view! {
        <article class="space-y-sm rounded-lg border border-border p-md">
            <h3 class="text-lg">
                <a href=service.href>{service.title}</a>
            </h3>
            <p>{service.blurb}</p>
        </article>
    }
}
