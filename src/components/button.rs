use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn Button<F>(
    #[prop(default = "")] class: &'static str,
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <button
            class=format!("shadow-sm rounded-lg px-4 py-2 {class}")
            on:click=on_click
        >
            {children()}
        </button>
    }
}
