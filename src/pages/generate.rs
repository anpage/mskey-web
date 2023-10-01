use std::rc::Rc;

use leptos::*;
use wasm_bindgen_futures::JsFuture;

use crate::gen::KeyGen;

#[cfg(web_sys_unstable_apis)]
#[component]
pub fn Generate(keygen: Rc<KeyGen>) -> impl IntoView {
    let (product, set_product) = create_signal("Windows XP Pro VLK".to_string());
    let (bink_id, set_bink_id) = create_signal(0x2E_u8);
    let (channel_id, set_channel_id) = create_signal("640".to_string());

    let update_channel_id = move |ev| {
        set_channel_id.set(event_target_value(&ev));
    };

    let update_bink_id = move |ev| {
        let bink_id = event_target_value(&ev);
        let bink_id = u8::from_str_radix(&bink_id, 16).unwrap();
        set_bink_id.set(bink_id);
    };

    let keygen_product_clone = keygen.clone();
    let update_product = move |ev| {
        let product_name = event_target_value(&ev);
        let product = keygen_product_clone
            .sorted_products()
            .iter()
            .find(|product| product.name == product_name)
            .unwrap();
        set_product.set(product_name);
        let bink_id = product.bink_ids.first().unwrap();
        set_bink_id.set(*bink_id);
    };

    let keygen_key_clone = keygen.clone();
    let key = create_memo(move |_| {
        let bink_id = bink_id.get();
        keygen_key_clone
            .gen_key(bink_id, &channel_id.get())
            .unwrap_or_else(|_| "".to_string())
    });

    let copy_key_action = create_action(|input: &String| {
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

    let keygen_view_clone = keygen.clone();
    view! {
        <div class="block">
            <div class="field">
                <label class="label" for="products">"Product"</label>
                <div class="select">
                    <select
                        name="products"
                        id="products"
                        on:input=update_product
                    >
                        {keygen_view_clone.sorted_products().iter().map(|p| {
                            view! {
                                <option value={p.name.clone()} selected={p.name == product.get()}>{p.name.clone()}</option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
            </div>
            <div class="field">
                <label class="label" for="bink">"BINK ID"</label>
                <div class="select">
                    <select
                        name="bink"
                        id="bink"
                        on:input=update_bink_id
                    >
                        {move || keygen_view_clone.sorted_products().iter().find(|p| p.name == product.get()).unwrap().bink_ids.iter().map(|b| {
                            let b_str = format!("{:02X}", b);
                            view! {
                                <option value={b_str.clone()} selected={*b == bink_id.get()}>{b_str}</option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
            </div>
            <div class="field">
                <label class="label" for="channel">"Channel ID"</label>
                <div class="control">
                    <input
                        class="input"
                        type="number"
                        min="0"
                        max="999"
                        name="channel"
                        id="channel"
                        on:change=update_channel_id
                        prop:value=channel_id.get()
                    >
                    </input>
                </div>
            </div>
        </div>
        <div class="block notification is-warning">
            <div class="columns is-mobile is-centered">
                <div class="column is-narrow">
                    <div class="content">
                        <code class="title is-4 is-size-6-mobile is-size-5-tablet is-size-4-desktop">
                            {key}
                        </code>
                    </div>
                </div>
            </div>
        </div>
        <p class="buttons block">
            <button
                class="button is-primary is-medium"
                on:click=move |_| {
                    copy_key_action.dispatch(key.get());
                }
            >
                <span class="icon">
                    <i class="fas fa-clipboard"></i>
                </span>
                <span>"Copy"</span>
            </button>
            <button
                class="button is-medium"
                on:click=move |_| {
                    set_bink_id.update(|_| ());
                }
            >
                <span class="icon">
                    <i class="fas fa-refresh"></i>
                </span>
                <span>"Regenerate"</span>
            </button>
        </p>
    }
}
