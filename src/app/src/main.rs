use leptos::component;
use leptos::create_signal;
use leptos::document;
use leptos::event_target_value;
use leptos::logging::log;
use leptos::view;
use leptos::IntoView;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::SignalUpdate;
use leptos::WriteSignal;
use leptos_router::{
    Route, RouteProps, Router, RouterProps, Routes, RoutesProps,
};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

fn main() {
    //leptos::mount_to_body(App)

    _ = console_error_panic_hook::set_once(); // For better error handling in Wasm
    leptos::mount_to_body(|cx| {
        view! { cx, <FetchComponent /> }
    });
}

#[component]
pub fn App() -> impl IntoView {
    view! {
      my app
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ApiResponse {
    title: String,
    body: String,
}

#[component]
fn FetchComponent(cx: Scope) -> impl IntoView {
    // Create a resource to fetch data asynchronously
    let data = create_resource(cx, || (), |_| fetch_data());

    view! { cx,
        <div>
            <h1>"Fetch API Data"</h1>
            {move || {
                if let Some(fetch_data) = data.read(cx) {
                    match fetch_data {
                        Ok(api_response) => view! { cx,
                            <div>
                                <h2>{api_response.title}</h2>
                                <p>{api_response.body}</p>
                            </div>
                        }.into_view(cx),
                        Err(_) => view! { cx, <p>"Error fetching data."</p> }.into_view(cx),
                    }
                } else {
                    view! { cx, <p>"Loading..."</p> }.into_view(cx)
                }
            }}
        </div>
    }
}

async fn fetch_data() -> Result<ApiResponse, Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = reqwest::get(url).await?.json::<ApiResponse>().await?;
    Ok(response)
}
