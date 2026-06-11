use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="Not found" />
        <section class="section">
            <div class="container">
                <h2>"Page not found"</h2>
                <p>
                    "That page doesn't exist. " <a href="/">"Return to the home page."</a>
                </p>
            </div>
        </section>
    }
}
