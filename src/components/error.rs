use leptos::*;

use crate::icons::ErrorIcon;

#[component]
pub fn Error(children: Children) -> impl IntoView {
    view! {
        <div class="shadow-sm rounded-lg px-4 py-4 text-slate-50 dark:text-slate-50 bg-rose-700">
            <span class="mr-2 -mt-4">
                <ErrorIcon />
            </span>
            {children()}
        </div>
    }
}
