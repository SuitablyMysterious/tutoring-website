use leptos::prelude::*;

use crate::content::shared::Cta;

#[component]
pub fn Nav(site_name: &'static str, links: &'static [Cta]) -> impl IntoView {
    view! {
        <header class="site-nav">
            <a href="/">{site_name}</a>
            <nav>
                <ul>
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
