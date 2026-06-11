use leptos::prelude::*;

#[component]
pub fn Footer(note: &'static str) -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="container">
                <p>{note}</p>
            </div>
        </footer>
    }
}
