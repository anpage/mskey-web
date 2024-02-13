use std::rc::Rc;

use leptos::*;
use wasm_bindgen::JsValue;
use web_sys::{Url, UrlSearchParams};

use crate::{
    components::{
        bink1998_details::Bink1998Details, bink2002_details::Bink2002Details, error::Error,
        fields::TextField,
    },
    gen::{KeyGen, ProductKey},
};

#[component]
pub fn Validate(keygen: Rc<KeyGen>) -> impl IntoView {
    let search = web_sys::window().unwrap().location().search().unwrap();
    let search = UrlSearchParams::new_with_str(&search).unwrap();

    let key = if let Some(key) = search.get("k") {
        key
    } else {
        "".to_string()
    };

    let (product_key, set_product_key) = create_signal(key);

    let update_product_key = move |ev| {
        set_product_key.set(event_target_value(&ev));
    };

    let validated_key = create_memo(move |_| {
        let key = product_key.get();
        let product_key = keygen.validate_key(&key);
        if let Ok(product_key) = product_key {
            let url = web_sys::window().unwrap().location().href().unwrap();
            let url = Url::new(&url).unwrap();
            if matches!(product_key, ProductKey::Invalid) {
                url.set_search("?validate");
            } else {
                url.set_search(&format!("?validate&k={}", key.trim()));
            }
            let url = url.href();
            let _ = web_sys::window()
                .unwrap()
                .history()
                .unwrap()
                .replace_state_with_url(&JsValue::NULL, "", Some(&url));

            product_key
        } else {
            let url = web_sys::window().unwrap().location().href().unwrap();
            let url = Url::new(&url).unwrap();
            url.set_search("?validate");
            let url = url.href();
            let _ = web_sys::window()
                .unwrap()
                .history()
                .unwrap()
                .replace_state_with_url(&JsValue::NULL, "", Some(&url));
            ProductKey::Invalid
        }
    });

    view! {
        <div class="mb-4">
            <TextField
                label="Product Key"
                id="productkey"
                on_change=update_product_key
                value=product_key
            />
        </div>
        {move || {
            if product_key.get().is_empty() {
                view! {
                    <div>
                    </div>
                }
            } else {
                match validated_key.get() {
                    ProductKey::Invalid => view! {
                        <div class="mt-6">
                            <Error>
                                "Invalid product key"
                            </Error>
                        </div>
                    },
                    ProductKey::Bink1998{ key, bink_ids } => view! {
                        <div>
                            <Bink1998Details key=key bink_ids=bink_ids />
                        </div>
                    },
                    ProductKey::Bink2002{ key, bink_ids } => view! {
                        <div>
                            <Bink2002Details key=key bink_ids=bink_ids />
                        </div>
                    }
                }
            }
        }}
    }
}
