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
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

fn main() {
    //leptos::mount_to_body(InputBox)
    let f1 = getElement("f1");
    leptos::mount_to(f1, App);

    let f2: HtmlElement = getElement("f2");
    leptos::mount_to(f2, InputBox);

    let f3 = getElement("f3");
    leptos::mount_to(f3, TogglBox);
}

fn getElement(id: &str) -> HtmlElement {
    document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into()
        .unwrap()
}

#[component]
fn App() -> impl IntoView {
    view! {
        hello
    }
}

#[component]
fn HelloComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        hello
    }
}

#[component]
fn InputBox() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());
    view! {
        <input type="text"
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name.set(event_target_value(&ev));

            }
            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            //prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn TogglBox() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled? " {toggled}</p>
        <ButtonA setter=set_toggled/>
    }
}

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}
