use std::rc::Rc;

use leptos::*;
use wasm_bindgen_futures::JsFuture;

use crate::gen::KeyGen;
use crate::icons::{CopyIcon, RefreshIcon};

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

    let label_classes = "block font-bold mb-2 text-gray-800 dark:text-slate-200";
    let input_classes =
        "block appearance-none w-full px-3 py-2 mr-8 rounded-lg smadow-sm bg-gray-100 text-gray-800 h-10 dark:text-slate-200 dark:bg-slate-900";

    let keygen_view_clone = keygen.clone();
    view! {
        <div class="mb-4">
            <label class=label_classes for="products">"Product"</label>
            <select
                name="products"
                id="products"
                class=input_classes
                on:input=update_product
            >
                {keygen_view_clone.sorted_products().iter().map(|p| {
                    view! {
                        <option value={p.name.clone()} selected={p.name == product.get()}>{p.name.clone()}</option>
                    }
                }).collect::<Vec<_>>()}
            </select>
        </div>
        <div class="flex flex-col sm:flex-row gap-4 mb-6">
            <div class="flex-1">
                <label class=label_classes for="bink">"BINK ID"</label>
                <select
                    name="bink"
                    id="bink"
                    class=input_classes
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
            <div class="flex-1">
                <label class=label_classes for="channel">"Channel ID"</label>
                <input
                    type="number"
                    min="0"
                    max="999"
                    name="channel"
                    id="channel"
                    class=format!("{input_classes} no-spinner")
                    on:change=update_channel_id
                    prop:value=channel_id.get()
                >
                </input>
            </div>
        </div>
        <div class="mb-8 mx-auto font-mono text-center text-xl text-gray-800 rounded-lg p-6 bg-gray-200 dark:bg-slate-700 dark:text-slate-200">
            {key}
        </div>
        <div class="flex flex-row gap-4">
            <button
                class="shadow-sm dark:bg-emerald-600 dark:hover:bg-emerald-700 dark:text-white bg-emerald-500 hover:bg-emerald-600 text-white rounded-lg px-4 py-2"
                on:click=move |_| {
                    copy_key_action.dispatch(key.get());
                }
            >
                <span class="mr-2">
                    <CopyIcon />
                </span>
                <span>"Copy"</span>
            </button>
            <button
                class="shadow-sm dark:bg-indigo-600 dark:hover:bg-indigo-700 dark:text-white bg-indigo-500 hover:bg-indigo-600 text-white rounded-lg px-4 py-2"
                on:click=move |_| {
                    set_bink_id.update(|_| ());
                }
            >
                <span class="mr-2">
                    <RefreshIcon />
                </span>
                <span>"Regenerate"</span>
            </button>
        </div>
    }
}
