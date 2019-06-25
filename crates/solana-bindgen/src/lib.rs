use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type PublicKey;

    #[wasm_bindgen(constructor)]
    pub fn new(key: Vec<u8>) -> PublicKey;

    pub type Connection;

    #[wasm_bindgen(constructor)]
    pub fn new(url: &str) -> Connection;

    #[wasm_bindgen(method, js_name = getAccountInfo)]
    pub fn get_account_info(this: &Connection, account: PublicKey) -> Promise;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
