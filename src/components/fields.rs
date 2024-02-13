use leptos::*;
use web_sys::Event;

const LABEL_CLASSES: &str = "block font-bold mb-2 text-gray-800 dark:text-slate-200";
const INPUT_CLASSES: &str =
    "block appearance-none w-full px-3 py-2 mr-8 rounded-lg smadow-sm bg-gray-100 text-gray-800 h-10 dark:text-slate-200 dark:bg-slate-900";

#[component]
pub fn SelectField<F>(
    label: &'static str,
    id: &'static str,
    on_input: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(Event) + 'static,
{
    view! {
        <label class=LABEL_CLASSES for=id>{label}</label>
        <select
            name=id
            id=id
            class=INPUT_CLASSES
            on:input=on_input
        >
            {children()}
        </select>
    }
}

#[component]
pub fn NumberField<F>(
    label: &'static str,
    id: &'static str,
    min: u32,
    max: u32,
    channel_id: ReadSignal<String>,
    on_change: F,
) -> impl IntoView
where
    F: Fn(Event) + 'static,
{
    view! {
        <label class=LABEL_CLASSES for=id>{label}</label>
        <input
            type="number"
            min=format!("{min}")
            max=format!("{max}")
            name=id
            id=id
            class=format!("{INPUT_CLASSES} no-spinner")
            on:change=on_change
            prop:value=channel_id.get()
        >
        </input>
    }
}
