use leptos::prelude::*;

#[component]
pub fn Section(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="section">
            <div class="container">
                <h2>{title}</h2>
                {children()}
            </div>
        </section>
    }
}
