use leptos::*;
use leptos_workers::worker;
use web_sys::UrlSearchParams;

use crate::{
    components::{
        bink1998_details::Bink1998Details, bink2002_details::Bink2002Details, error::Error,
        fields::TextField,
    },
    crypto::{self, ProductKey},
};

#[worker]
async fn validate_key(key: String) -> ProductKey {
    let key_tool = crypto::KeyTool::new();
    let product_key = key_tool.validate_key(&key);
    product_key.unwrap_or(ProductKey::Invalid)
}

#[component]
pub fn Validate() -> impl IntoView {
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

    let validation_response = create_local_resource(
        move || (product_key.get()),
        |key| async {
            if key.is_empty() {
                Ok(ProductKey::Empty)
            } else {
                validate_key(key).await
            }
        },
    );

    view! {
        <div class="mb-4">
            <TextField
                label="Product Key"
                id="productkey"
                on_change=|_| ()
                on_input=update_product_key
                value=product_key
            />
        </div>

        <Suspense fallback=move || view! { <Loading /> }>
            {move || match validation_response.get() {
                Some(Ok(ProductKey::Empty)) | None => view! { <div></div> },
                Some(Ok(ProductKey::Invalid)) => {
                    view! {
                        <div class="mt-6">
                            <Error>"Invalid product key"</Error>
                        </div>
                    }
                }
                Some(Ok(ProductKey::Bink1998 { key, bink_ids })) => {
                    view! {
                        <div>
                            <Bink1998Details key=key bink_ids=bink_ids />
                        </div>
                    }
                }
                Some(Ok(ProductKey::Bink2002 { key, bink_ids })) => {
                    view! {
                        <div>
                            <Bink2002Details key=key bink_ids=bink_ids />
                        </div>
                    }
                }
                Some(Err(_)) => {
                    view! {
                        <div class="mt-6">
                            <Error>"Error validating key"</Error>
                        </div>
                    }
                }
            }}
        </Suspense>
    }
}

#[component]
fn Loading() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center pt-4">
            <div
            class="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-e-transparent align-[-0.125em] text-surface motion-reduce:animate-[spin_1.5s_linear_infinite]"
            role="status">
                <span class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]">
                    Loading...
                </span>
            </div>
        </div>
    }
}
