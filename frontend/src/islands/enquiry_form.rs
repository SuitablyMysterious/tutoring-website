use common::EnquiryRequest;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Idle,
    Sending,
    Sent,
    Failed,
}

#[island]
pub fn EnquiryForm() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());
    // Honeypot: invisible to humans, filled by naive bots.
    let website = RwSignal::new(String::new());
    let status = RwSignal::new(Status::Idle);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if status.get() == Status::Sending {
            return;
        }
        status.set(Status::Sending);

        let req = EnquiryRequest {
            name: name.get(),
            email: email.get(),
            message: message.get(),
            website: website.get(),
        };
        spawn_local(async move {
            match submit(&req).await {
                Ok(()) => status.set(Status::Sent),
                Err(()) => status.set(Status::Failed),
            }
        });
    };

    view! {
        <form class="enquiry-form" on:submit=on_submit>
            <label>"Name" <input type="text" bind:value=name required /></label>
            <label>"Email" <input type="email" bind:value=email required /></label>
            <label>"Message" <textarea bind:value=message required></textarea></label>
            <label class="hp-field" aria-hidden="true">
                "Website" <input type="text" bind:value=website tabindex="-1" autocomplete="off" />
            </label>
            <button class="button" type="submit" disabled=move || status.get() == Status::Sending>
                {move || {
                    if status.get() == Status::Sending { "Sending…" } else { "Send enquiry" }
                }}
            </button>
            {move || match status.get() {
                Status::Sent => {
                    Some(
                        view! { <p class="form-status">"Thanks — your enquiry has been sent."</p> }
                            .into_any(),
                    )
                }
                Status::Failed => {
                    Some(
                        view! {
                            <p class="form-status error">
                                "Sorry — enquiries are currently unavailable. Please email directly instead."
                            </p>
                        }
                            .into_any(),
                    )
                }
                _ => None,
            }}
        </form>
    }
}

/// POSTs the enquiry to the backend. The body only runs in the browser; the
/// ssr build needs it to compile but never calls it.
async fn submit(req: &EnquiryRequest) -> Result<(), ()> {
    #[cfg(feature = "hydrate")]
    {
        let response = gloo_net::http::Request::post("/api/enquiry")
            .json(req)
            .map_err(|_| ())?
            .send()
            .await
            .map_err(|_| ())?;
        if response.ok() { Ok(()) } else { Err(()) }
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = req;
        unreachable!("enquiry submission only runs in the browser")
    }
}
