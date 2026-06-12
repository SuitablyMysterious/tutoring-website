use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="Not found" />
        <section class="py-lg">
            <div class="mx-auto max-w-5xl space-y-md px-md">
                <h2 class="text-2xl">"Page not found"</h2>
                <p>
                    "That page doesn't exist. " <a href="/">"Return to the home page."</a>
                </p>
            </div>
        </section>
    }
}
