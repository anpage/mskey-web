use leptos::*;
use web_sys::Event;

const INPUT_CLASSES: &str =
    "block appearance-none w-full px-3 py-2 mr-8 rounded-lg smadow-sm bg-slate-100 h-10 dark:bg-slate-900";

#[component]
pub fn Label(#[prop(optional)] for_id: &'static str, children: Children) -> impl IntoView {
    view! {
        <label
            class="block font-bold mb-2 text-lg"
            for=for_id
        >
            {children()}
        </label>
    }
}

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
        <Label for_id=id>{label}</Label>
        <select
            name=id
            id=id
            class=format!("{INPUT_CLASSES} cursor-pointer")
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
        <Label for_id=id>{label}</Label>
        <input
            type="number"
            min=format!("{min}")
            max=format!("{max}")
            name=id
            id=id
            class=format!("{INPUT_CLASSES} no-spinner")
            on:change=on_change
            prop:value=channel_id.get()
        />
    }
}

#[component]
pub fn TextField<F, G>(
    label: &'static str,
    id: &'static str,
    on_change: F,
    on_input: G,
    value: ReadSignal<String>,
) -> impl IntoView
where
    F: Fn(Event) + 'static,
    G: Fn(Event) + 'static,
{
    view! {
        <Label for_id=id>{label}</Label>
        <input
            type="text"
            name=id
            id=id
            class=INPUT_CLASSES
            on:change=on_change
            on:input=on_input
            prop:value=value.get()
        />
    }
}

#[component]
pub fn CheckBox<F>(
    label: &'static str,
    id: &'static str,
    on_change: F,
    checked: ReadSignal<bool>,
) -> impl IntoView
where
    F: Fn(Event) + 'static,
{
    view! {
        <Label for_id=id>{label}</Label>
        <label class="pt-2 inline-flex items-center cursor-pointer">
            <input
                type="checkbox"
                name=id
                id=id
                class="mr-2 sr-only peer"
                on:change=on_change
                prop:checked=checked.get()
            />
            <div class="relative w-11 h-6 bg-gray-200
            peer-focus:outline-none
            peer-focus:ring-4
            peer-focus:ring-indigo-300
            dark:peer-focus:ring-indigo-500
            rounded-full peer dark:bg-gray-700
            peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full
            peer-checked:after:border-white after:content-['']
            after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all
            dark:border-gray-600 peer-checked:bg-indigo-600"></div>
        </label>
    }
}
