use std::collections::HashMap;

use leptos::*;

use crate::{components::fields::Label, crypto::ProductKey2002};

#[component]
pub fn Bink2002Details(key: ProductKey2002, bink_ids: HashMap<String, u8>) -> impl IntoView {
    let bink_id_list = {
        let bink_id_list = bink_ids
            .iter()
            .map(|(name, bink_id)| {
                let bink_id = format!("BINK: {:02X}", bink_id);
                view! { <li>{format!("{} ({})", name, bink_id)}</li> }
            })
            .collect::<Vec<_>>();
        bink_id_list
    };
    view! {
        <div class="mb-4">
            <Label>"Products"</Label>
            <ul class="list-disc list-inside">{bink_id_list}</ul>
        </div>
        <div class="mb-4">
            <Label>"Details"</Label>
            <table class="w-full text-left">
                <tbody>
                    <tr>
                        <th class="font-semibold px-4 py-2">"Upgrade"</th>
                        <td class="font-mono px-4 py-2">{key.upgrade}</td>
                        <td></td>
                    </tr>
                    <tr class="bg-slate-100 dark:bg-slate-700">
                        <th class="font-semibold px-4 py-2">"Channel ID"</th>
                        <td class="font-mono px-4 py-2">{key.channel_id.clone()}</td>
                        <td class="font-mono px-4 py-2">
                            {
                                let channel_id = key.channel_id.parse::<u32>().unwrap();
                                format!("0x{:X}", channel_id)
                            }
                        </td>
                    </tr>
                    <tr>
                        <th class="font-semibold px-4 py-2">"Hash"</th>
                        <td class="font-mono px-4 py-2">{key.hash.clone()}</td>
                        <td class="font-mono px-4 py-2">
                            {
                                let hash = key.hash.parse::<u32>().unwrap();
                                format!("0x{:X}", hash)
                            }
                        </td>
                    </tr>
                    <tr class="bg-slate-100 dark:bg-slate-700">
                        <th class="font-semibold px-4 py-2">"Signature"</th>
                        <td class="font-mono px-4 py-2">{key.signature.clone()}</td>
                        <td class="font-mono px-4 py-2">
                            {
                                let signature = key.signature.parse::<u64>().unwrap();
                                format!("0x{:X}", signature)
                            }
                        </td>
                    </tr>
                    <tr>
                        <th class="font-semibold px-4 py-2">"Auth Info"</th>
                        <td class="font-mono px-4 py-2">{key.auth_info.clone()}</td>
                        <td class="font-mono px-4 py-2">
                            {
                                let auth_info = key.auth_info.parse::<u32>().unwrap();
                                format!("0x{:X}", auth_info)
                            }
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
