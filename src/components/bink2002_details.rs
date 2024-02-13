use std::collections::HashMap;

use leptos::*;
use umskt::pidgen3::bink2002::ProductKey;

use crate::components::fields::Label;

#[component]
pub fn Bink2002Details(key: ProductKey, bink_ids: HashMap<String, u8>) -> impl IntoView {
    let bink_id_list = {
        let bink_id_list = bink_ids
            .iter()
            .map(|(name, bink_id)| {
                let bink_id = format!("BINK: {:02X}", bink_id);
                view! {
                    <li>{format!("{} ({})", name, bink_id)}</li>
                }
            })
            .collect::<Vec<_>>();
        bink_id_list
    };
    view! {
        <div class="mb-4">
            <Label>"Products"</Label>
            <ul class="list-disc list-inside">
                {bink_id_list}
            </ul>
        </div>
        <div class="mb-4">
        <Label>"Details"</Label>
        <table class="w-full text-left">
                <tbody>
                    <tr>
                    <th class="font-semibold px-4 py-2">"Upgrade"</th>
                        <td class="font-mono px-4 py-2">{key.upgrade()}</td>
                        <td></td>
                    </tr>
                    <tr class="bg-slate-100 dark:bg-slate-700">
                    <th class="font-semibold px-4 py-2">"Channel ID"</th>
                        <td class="font-mono px-4 py-2">{key.channel_id()}</td>
                        <td class="font-mono px-4 py-2">{format!("0x{:X}", key.channel_id())}</td>
                    </tr>
                    <tr>
                    <th class="font-semibold px-4 py-2">"Hash"</th>
                        <td class="font-mono px-4 py-2">{key.hash()}</td>
                        <td class="font-mono px-4 py-2">{format!("0x{:X}", key.hash())}</td>
                    </tr>
                    <tr class="bg-slate-100 dark:bg-slate-700">
                    <th class="font-semibold px-4 py-2">"Signature"</th>
                        <td class="font-mono px-4 py-2">{key.signature()}</td>
                        <td class="font-mono px-4 py-2">{format!("0x{:X}", key.signature())}</td>
                    </tr>
                    <tr>
                    <th class="font-semibold px-4 py-2">"Auth Info"</th>
                        <td class="font-mono px-4 py-2">{key.auth_info()}</td>
                        <td class="font-mono px-4 py-2">{format!("0x{:X}", key.auth_info())}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
