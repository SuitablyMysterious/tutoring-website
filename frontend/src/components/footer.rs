use leptos::prelude::*;

#[component]
pub fn Footer(note: &'static str) -> impl IntoView {
    view! {
        <footer class="mt-xl border-t border-border text-text-muted">
            <div class="mx-auto max-w-5xl px-md py-lg">
                <p>{note}</p>
            </div>
        </footer>
    }
}
