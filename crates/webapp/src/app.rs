use base58::FromBase58;
use log::info;
use message_feed_data::*;
use solana_bindgen::{Connection, PublicKey};
use solana_sdk::account::Account;
use solana_sdk_bpf_types::*;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    closure: Closure<dyn FnMut(JsValue) -> ()>,
    connection: Connection,
    program_key: SolPubkey,
    first_message_key: SolPubkey,
}

pub enum Msg {
    AccountInfo(JsValue),
}

const CONNECTION_URL: &str = "https://api.beta.testnet.solana.com";
const B58_PROGRAM_KEY: &str = "CgrNpmr487utCtzR3H7CK15a4VX87VjNQLmEeHHgRFiY";
const B58_FIRST_MESSAGE_KEY: &str = "FDxi8sFQ2bXD54CqygPVc4b3BFkEJGm5iPvzb52pGmv9";

fn from_base58_str(s: &str) -> SolPubkey {
    s.from_base58().unwrap().as_slice().try_into().unwrap()
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let connection = Connection::new(CONNECTION_URL);
        let program_key = from_base58_str(B58_PROGRAM_KEY);
        let first_message_key = from_base58_str(B58_FIRST_MESSAGE_KEY);
        let key = PublicKey::new(first_message_key.to_vec());
        info!("get account info");
        let promise = connection.get_account_info(key);
        let callback = link.send_back(Msg::AccountInfo);
        let closure = Closure::once(move |val| {
            info!("promise completed");
            callback.emit(val);
        });
        promise.then(&closure);

        App {
            link,
            closure,
            connection,
            program_key,
            first_message_key,
        }
    }

    fn update<'a>(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AccountInfo(val) => {
                info!("account info response: {:#?}", val);
                let mut account: Account = serde_wasm_bindgen::from_value(val).unwrap();
                let message_feed_data = MessageAccountData::new(account.data.as_mut_slice());
                let text = std::str::from_utf8(message_feed_data.text).unwrap();
                info!("text: {}", text);
            }
        }

        true
    }
}

impl Renderable<Self> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="app">
                { "Hello World yo!" }
            </div>
        }
    }
}
