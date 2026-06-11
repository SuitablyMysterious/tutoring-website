use leptos::prelude::*;

use crate::content::shared::{Cta, Stat};

#[component]
pub fn Hero(
    eyebrow: &'static str,
    title: &'static str,
    emphasis: &'static str,
    subtitle: &'static str,
    stats: &'static [Stat],
    primary_cta: Cta,
    secondary_cta: Cta,
) -> impl IntoView {
    view! {
        <section class="hero">
            <div class="container">
                <p class="eyebrow">{eyebrow}</p>
                <h1>{title} " " <em>{emphasis}</em></h1>
                <p>{subtitle}</p>
                <div class="stats">
                    {stats
                        .iter()
                        .map(|stat| {
                            view! {
                                <div class="stat">
                                    <strong>{stat.number}</strong>
                                    " "
                                    <span>{stat.label}</span>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
                <p>
                    <a class="button" href=primary_cta.href>
                        {primary_cta.label}
                    </a>
                    " "
                    <a class="button secondary" href=secondary_cta.href>
                        {secondary_cta.label}
                    </a>
                </p>
            </div>
        </section>
    }
}
