use std::rc::Rc;

use leptos::*;
use wasm_bindgen_futures::JsFuture;

use crate::components::{
    button::Button,
    fields::{NumberField, SelectField},
};
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

    let keygen_view_clone_1 = keygen.clone();
    let keygen_view_clone_2 = keygen.clone();
    view! {
        <div class="mb-4">
            <SelectField
                label="Product"
                id="product"
                on_input=update_product
            >
                {keygen_view_clone_1.sorted_products().iter().map(|p| {
                    view! {
                        <option value={p.name.clone()} selected={p.name == product.get()}>{p.name.clone()}</option>
                    }
                }).collect::<Vec<_>>()}
            </SelectField>
        </div>
        <div class="flex flex-col sm:flex-row gap-4 mb-6">
            <div class="flex-1">
                <SelectField
                    label="BINK ID"
                    id="bink"
                    on_input=update_bink_id
                >
                    {move || keygen_view_clone_2.sorted_products().iter().find(|p| p.name == product.get()).unwrap().bink_ids.iter().map(|b| {
                        let b_str = format!("{:02X}", b);
                        view! {
                            <option value={b_str.clone()} selected={*b == bink_id.get()}>{b_str}</option>
                        }
                    }).collect::<Vec<_>>()}
                </SelectField>
            </div>
            <div class="flex-1">
                <NumberField
                    label="Channel ID"
                    id="channel"
                    min=0
                    max=999
                    channel_id=channel_id
                    on_change=update_channel_id
                />
            </div>
        </div>
        <div class="mb-8 mx-auto font-mono text-center text-xl text-gray-800 rounded-lg p-6 bg-gray-200 dark:bg-slate-700 dark:text-slate-200">
            {key}
        </div>
        <div class="flex flex-row gap-4">
            <Button
                class="bg-emerald-500 hover:bg-emerald-600 dark:bg-emerald-600 dark:hover:bg-emerald-700"
                on_click=move |_| {
                    copy_key_action.dispatch(key.get());
                }
            >
                <span class="mr-2">
                    <CopyIcon />
                </span>
                <span>"Copy"</span>
            </Button>
            <Button
                class="bg-indigo-500 hover:bg-indigo-600 dark:bg-indigo-600 dark:hover:bg-indigo-700"
                on_click=move |_| {
                    set_bink_id.update(|_| ());
                }
            >
                <span class="mr-2">
                    <RefreshIcon />
                </span>
                <span>"Regenerate"</span>
            </Button>
        </div>
    }
}
