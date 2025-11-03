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
    leptos::mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    view! {
     hello
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        Home
    }
}

#[component]
fn Users() -> impl IntoView {
    view! {
        Users
    }
}

#[component]
fn UserProfile() -> impl IntoView {
    view! {
        UserProfile
    }
}
