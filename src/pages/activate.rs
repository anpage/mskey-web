use leptos::*;

use crate::{
    components::{
        button::Button,
        fields::{Label, TextField},
    },
    gen::KeyGen,
    icons::CopyIcon,
};

#[cfg(web_sys_unstable_apis)]
#[component]
pub fn Activate() -> impl IntoView {
    use wasm_bindgen_futures::JsFuture;

    let (installation_id, set_installation_id) = create_signal("".to_string());

    let update_installation_id = move |ev| {
        set_installation_id.set(event_target_value(&ev));
    };

    let confirmation_id = create_memo(move |_| {
        let installation_id = installation_id.get();
        KeyGen::get_confirmation_id(&installation_id)
            .unwrap_or_else(|_| "INVALID INSTALLATION ID".to_string())
    });

    let copy_confirmation_id_action = create_action(|input: &String| {
        let input = input.clone();
        async move {
            JsFuture::from(
                web_sys::window()
                    .unwrap()
                    .navigator()
                    .clipboard()
                    .unwrap()
                    .write_text(&input),
            )
            .await
            .unwrap();
        }
    });

    view! {
        <div class="mb-4">
            <TextField
                label="Installation ID"
                id="installationid"
                on_change=|_| {}
                on_input=update_installation_id
                value=installation_id
            />
        </div>
        <Label>"Confirmation ID"</Label>
        <div class="overflow-scroll sm:overflow-clip mb-8 mx-auto font-mono text-center text-2xl text-slate-800 rounded-lg p-6 bg-slate-200 dark:bg-slate-700 dark:text-slate-200">
                {confirmation_id}
        </div>
        <Button
            class="text-slate-100 bg-emerald-500 hover:bg-emerald-600 dark:bg-emerald-600 dark:hover:bg-emerald-700"
            on_click=move |_| {
                copy_confirmation_id_action.dispatch(confirmation_id.get());
            }
        >
            <span class="mr-2">
                <CopyIcon />
            </span>
            <span>"Copy"</span>
        </Button>
    }
}
