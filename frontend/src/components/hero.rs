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
        <section class="bg-surface-alt py-xl text-center">
            <div class="mx-auto max-w-5xl space-y-md px-md">
                <p class="text-sm tracking-widest text-text-muted uppercase">{eyebrow}</p>
                <h1 class="text-4xl">{title} " " <em class="text-accent not-italic">{emphasis}</em></h1>
                <p>{subtitle}</p>
                <div class="flex justify-center gap-lg pt-md">
                    {stats
                        .iter()
                        .map(|stat| {
                            view! {
                                <div>
                                    <strong>{stat.number}</strong>
                                    " "
                                    <span>{stat.label}</span>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
                <p class="flex justify-center gap-md pt-md">
                    <a
                        href=primary_cta.href
                        class="inline-block rounded-md bg-primary px-lg py-sm text-primary-contrast no-underline"
                    >
                        {primary_cta.label}
                    </a>
                    <a
                        href=secondary_cta.href
                        class="inline-block rounded-md border border-primary px-lg py-sm text-primary no-underline"
                    >
                        {secondary_cta.label}
                    </a>
                </p>
            </div>
        </section>
    }
}
