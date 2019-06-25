use log::info;
use message_feed_data::*;
use solana_bindgen::{Connection, PublicKey};
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;
use yew::services::{Task, TimeoutService};

pub struct App {
    link: ComponentLink<Self>,
    timeout: TimeoutService,
    closure: Option<Closure<dyn FnMut(JsValue) -> ()>>,
    connection: Connection,
    first_message_key: Pubkey,
    refresh_task: Option<Box<Task>>,
    messages: Vec<String>,
}

pub enum Msg {
    FetchAccountInfo(Pubkey, bool),
    AccountInfo(Pubkey, JsValue, bool),
}

// const EMPTY_KEY: Pubkey = Pubkey::new(&[1; 32]);
const CONNECTION_URL: &str = "https://api.beta.testnet.solana.com";
const B58_FIRST_MESSAGE_KEY: &str = "J56CEQXnxkEvQ7yT1D82gu9fG5t6ykWC3eu4nUABXabt";

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let first_message_key = Pubkey::from_str(B58_FIRST_MESSAGE_KEY).unwrap();
        link.send_self(Msg::FetchAccountInfo(first_message_key.clone(), false));
        App {
            link,
            closure: None,
            connection: Connection::new(CONNECTION_URL),
            first_message_key,
            timeout: TimeoutService::new(),
            refresh_task: None,
            messages: vec![],
        }
    }

    fn update<'a>(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchAccountInfo(pubkey, repeat) => {
                if self.closure.is_none() {
                    if repeat {
                        info!("poll for updates");
                    } else {
                        info!("fetch message: {}", pubkey);
                    }
                    let key = PublicKey::new(pubkey.as_ref().to_vec());
                    let callback = self
                        .link
                        .send_back(move |v| Msg::AccountInfo(pubkey, v, repeat));
                    let closure = Closure::once(move |val| callback.emit(val));
                    self.connection.get_account_info(key).then(&closure);
                    self.closure = Some(closure);
                } else {
                    return false;
                }
            }
            Msg::AccountInfo(key, val, repeat) => {
                self.closure = None;
                let mut account: Account = serde_wasm_bindgen::from_value(val).unwrap();
                let message_feed_data = MessageAccountData::new(account.data.as_mut_slice());
                let text = std::str::from_utf8(message_feed_data.text).unwrap();
                info!("text: {}", text);
                if !repeat {
                    self.messages.push(String::from(text));
                }
                let next_key = Pubkey::new(message_feed_data.next_message);
                let empty_key = Pubkey::new(&[0; 32]);
                if next_key != empty_key {
                    self.link.send_self(Msg::FetchAccountInfo(next_key, false));
                } else {
                    let callback = self
                        .link
                        .send_back(move |_| Msg::FetchAccountInfo(key, true));
                    let handle = self.timeout.spawn(Duration::from_millis(1000), callback);
                    self.refresh_task = Some(Box::new(handle));
                }
                if !repeat {
                    return false;
                }
            }
        }

        true
    }
}

impl Renderable<Self> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="app">
                <h1>{ "Message Feed" }</h1>
                <ol>
                    { for self.messages.iter().map(|text| html! { <li> {text} </li> } ) }
                </ol>
            </div>
        }
    }
}
