use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::components::{footer::Footer, nav::Nav};
use crate::content::site;
use crate::pages::{
    about::AboutPage, contact::ContactPage, home::HomePage, not_found::NotFoundPage,
    services::ServicesPage,
};

/// The HTML document shell, rendered by the backend around `App`.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    use leptos::hydration::{AutoReload, HydrationScripts};

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tutoring-website.css" />
        <Title text=site::SITE_NAME />
        <Router>
            <Nav site_name=site::SITE_NAME links=site::NAV_LINKS />
            <main>
                <Routes fallback=NotFoundPage>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("about") view=AboutPage />
                    <Route path=StaticSegment("services") view=ServicesPage />
                    <Route path=StaticSegment("contact") view=ContactPage />
                </Routes>
            </main>
            <Footer note=site::FOOTER_NOTE />
        </Router>
    }
}
