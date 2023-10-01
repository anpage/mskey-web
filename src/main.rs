use std::rc::Rc;

use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{Url, UrlSearchParams};

use crate::pages::{activate::Activate, generate::Generate, validate::Validate};

mod components;
mod gen;
mod pages;

fn main() {
    mount_to_body(|| view! { <App /> })
}

#[derive(Debug, Clone, Copy)]
enum SelectedTab {
    Generate,
    Validate,
    Activate,
}

#[component]
fn App() -> impl IntoView {
    // Initialize the KeyGen struct into an Rc, which can be cheaply passed around to components.
    let keygen = Rc::new(gen::KeyGen::new().unwrap());

    let search = web_sys::window().unwrap().location().search().unwrap();
    let search = UrlSearchParams::new_with_str(&search).unwrap();
    let tab = if search.get("activate").is_some() {
        SelectedTab::Activate
    } else if search.get("validate").is_some() {
        SelectedTab::Validate
    } else {
        SelectedTab::Generate
    };

    let (selected_tab, set_selected_tab) = create_signal(tab);
    let is_generate_active = move || matches!(selected_tab.get(), SelectedTab::Generate);
    let is_validate_active = move || matches!(selected_tab.get(), SelectedTab::Validate);
    let is_activate_active = move || matches!(selected_tab.get(), SelectedTab::Activate);

    let update_selected_tab = move |tab| {
        let tab_name = match tab {
            SelectedTab::Generate => "generate",
            SelectedTab::Validate => "validate",
            SelectedTab::Activate => "activate",
        };
        let url = web_sys::window().unwrap().location().href().unwrap();
        let url = Url::new(&url).unwrap();
        url.set_search(&format!("?{}", tab_name));
        let url = url.href();
        let _ = web_sys::window()
            .unwrap()
            .history()
            .unwrap()
            .replace_state_with_url(&JsValue::NULL, "", Some(&url));
        set_selected_tab.set(tab);
    };

    view! {
        <div class="section">
            <div class="columns is-centered">
                <div class="column is-narrow is-full-tablet is-8-desktop is-6-widescreen is-5-fullhd">
                    <div class="columns is-mobile is-centered">
                        <div class="column is-narrow">
                            <div class="tabs is-toggle is-toggle-rounded">
                                <ul>
                                    <li class={move || if is_generate_active() {"is-active"} else {""} }>
                                        <a on:click=move |_| {
                                            if is_generate_active() {
                                                return;
                                            }
                                            update_selected_tab(SelectedTab::Generate);
                                        }>
                                            "Generate"
                                        </a>
                                    </li>
                                    <li class={move || if is_validate_active() {"is-active"} else {""} }>
                                        <a on:click=move |_| {
                                            if is_validate_active() {
                                                return;
                                            }
                                            update_selected_tab(SelectedTab::Validate);
                                        }>
                                            "Validate"
                                        </a>
                                    </li>
                                    <li class={move || if is_activate_active() {"is-active"} else {""} }>
                                        <a on:click=move |_| {
                                            if is_activate_active() {
                                                return;
                                            }
                                            update_selected_tab(SelectedTab::Activate);
                                        }>
                                            "Activate"
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                    {move || {
                        match selected_tab.get() {
                            SelectedTab::Generate => view! {  <div><Generate keygen=keygen.clone() /></div> },
                            SelectedTab::Validate => view! {  <div><Validate keygen=keygen.clone() /></div> },
                            SelectedTab::Activate => view! {  <div><Activate /></div> },
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
