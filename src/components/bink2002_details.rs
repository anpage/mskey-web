use std::collections::HashMap;

use leptos::*;
use umskt::pidgen3::bink2002::ProductKey;

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
        <label class="label">"Products"</label>
        <div class="content">
            <ul>
                {bink_id_list}
            </ul>
        </div>
        <label class="label">"Details"</label>
        <table class="table is-striped is-fullwidth">
            <tbody>
                <tr>
                    <td class="has-text-weight-semibold">"Upgrade"</td>
                    <td>{key.upgrade()}</td>
                    <td></td>
                </tr>
                <tr>
                    <td class="has-text-weight-semibold">"Channel ID"</td>
                    <td>{key.channel_id()}</td>
                    <td>{format!("0x{:X}", key.channel_id())}</td>
                </tr>
                <tr>
                    <td class="has-text-weight-semibold">"Hash"</td>
                    <td>{key.hash()}</td>
                    <td>{format!("0x{:X}", key.hash())}</td>
                </tr>
                <tr>
                    <td class="has-text-weight-semibold">"Signature"</td>
                    <td>{key.signature()}</td>
                    <td>{format!("0x{:X}", key.signature())}</td>
                </tr>
                <tr>
                    <td class="has-text-weight-semibold">"Auth Info"</td>
                    <td>{key.auth_info()}</td>
                    <td>{format!("0x{:X}", key.auth_info())}</td>
                </tr>
            </tbody>
        </table>
    }
}
