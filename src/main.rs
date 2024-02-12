use std::rc::Rc;

use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{Url, UrlSearchParams};

use crate::pages::{activate::Activate, generate::Generate, validate::Validate};

mod components;
mod gen;
mod icons;
mod pages;

fn main() {
    mount_to_body(|| view! { <App /> })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Generate,
    Validate,
    Activate,
}

impl Tab {
    pub fn class(tab: Tab, selected: Tab) -> String {
        let class = if tab == selected {
            "inline-block w-full p-3 text-gray-900 bg-gray-100 active focus:outline-none dark:bg-slate-700 dark:text-white"
        } else {
            "inline-block w-full p-3 bg-gray-50 hover:text-gray-700 hover:bg-gray-50 focus:outline-none dark:hover:text-white dark:bg-slate-900 dark:hover:bg-slate-700"
        };
        match tab {
            Tab::Generate => format!("{} rounded-t-lg sm:rounded-none sm:rounded-s-lg", class),
            Tab::Validate => class.to_string(),
            Tab::Activate => format!("{} rounded-b-lg sm:rounded-none sm:rounded-e-lg", class),
        }
    }
}

impl ToString for Tab {
    fn to_string(&self) -> String {
        match self {
            Tab::Generate => "generate",
            Tab::Validate => "validate",
            Tab::Activate => "activate",
        }
        .to_string()
    }
}

impl From<UrlSearchParams> for Tab {
    fn from(search: UrlSearchParams) -> Self {
        if search.get("activate").is_some() {
            Tab::Activate
        } else if search.get("validate").is_some() {
            Tab::Validate
        } else {
            Tab::Generate
        }
    }
}

#[component]
fn App() -> impl IntoView {
    // Initialize the KeyGen struct into an Rc, which can be cheaply passed around to components.
    let keygen = Rc::new(gen::KeyGen::new().unwrap());

    let search = web_sys::window().unwrap().location().search().unwrap();
    let search = UrlSearchParams::new_with_str(&search).unwrap();
    let tab: Tab = search.into();

    let (selected_tab, set_selected_tab) = create_signal(tab);

    let select_tab = move |tab: Tab| {
        if selected_tab.get() == tab {
            return;
        }
        let url = web_sys::window().unwrap().location().href().unwrap();
        let url = Url::new(&url).unwrap();
        url.set_search(&format!("?{}", tab.to_string()));
        let url = url.href();
        let _ = web_sys::window()
            .unwrap()
            .history()
            .unwrap()
            .replace_state_with_url(&JsValue::NULL, "", Some(&url));
        set_selected_tab.set(tab);
    };

    view! {
        <div class="max-w-screen-md mx-auto sm:my-8 px-8 pt-6 pb-8 bg-white dark:bg-slate-800 sm:shadow-xl rounded">
            <ul class="max-w-md mx-auto mb-8 font-medium text-center text-gray-500 rounded-lg shadow sm:flex dark:divide-gray-700 dark:text-gray-400">
                <li class="w-full">
                    <a href="" on:click=move |_| {
                        select_tab(Tab::Generate);
                    } class={move || Tab::class(Tab::Generate, selected_tab.get())}>
                        "Generate"
                    </a>
                </li>
                <li class="w-full">
                    <a href="" on:click=move |_| {
                        select_tab(Tab::Validate);
                    } class={move || Tab::class(Tab::Validate, selected_tab.get())}>
                        "Validate"
                    </a>
                </li>
                <li class="w-full">
                    <a href="" on:click=move |_| {
                        select_tab(Tab::Activate);
                    } class={move || Tab::class(Tab::Activate, selected_tab.get())}>
                        "Activate"
                    </a>
                </li>
            </ul>
            <div class="dark:text-white">
                {move || {
                    match selected_tab.get() {
                        Tab::Generate => view! { <Generate keygen=keygen.clone() /> },
                        Tab::Validate => view! { <Validate keygen=keygen.clone() /> },
                        Tab::Activate => view! { <Activate /> },
                    }
                }}
            </div>
        </div>
    }
}
