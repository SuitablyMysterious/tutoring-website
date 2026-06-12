use leptos::prelude::*;

use crate::content::shared::Cta;

#[component]
pub fn Nav(site_name: &'static str, links: &'static [Cta]) -> impl IntoView {
    view! {
        <header class="flex items-center justify-between border-b border-border p-md">
            <a href="/" class="font-heading text-lg">
                {site_name}
            </a>
            <nav>
                <ul class="flex gap-md">
                    {links
                        .iter()
                        .map(|link| {
                            view! {
                                <li>
                                    <a href=link.href>{link.label}</a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </nav>
        </header>
    }
}
