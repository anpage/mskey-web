use leptos::*;

use crate::gen::KeyGen;

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
        <div class="block">
            <div class="field">
                <label class="label" for="installationid">"Installation ID"</label>
                <div class="control">
                    <input
                        class="input"
                        type="text"
                        name="installationid"
                        id="installationid"
                        on:input=update_installation_id
                    >
                    </input>
                </div>
            </div>
        </div>
        <label class="label" for="confirmationid">"Confirmation ID"</label>
        <div class="block notification is-warning">
            <div class="columns is-mobile is-centered">
                <div class="column is-narrow">
                    <div class="content">
                        <code class="title is-4 is-size-7-mobile is-size-6-tablet is-size-5-desktop">
                            {confirmation_id}
                        </code>
                    </div>
                </div>
            </div>
        </div>
        <p class="buttons block">
            <button
                class="button is-primary is-medium"
                on:click=move |_| {
                    copy_confirmation_id_action.dispatch(confirmation_id.get());
                }
            >
                <span class="icon">
                    <i class="fas fa-clipboard"></i>
                </span>
                <span>"Copy"</span>
            </button>
        </p>
    }
}
