use leptos::prelude::*;

#[component]
pub fn Section(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="py-lg">
            <div class="mx-auto max-w-5xl space-y-md px-md">
                <h2 class="text-2xl">{title}</h2>
                {children()}
            </div>
        </section>
    }
}
