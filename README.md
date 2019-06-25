# solana-feed-rs

Solana web app built with Rust

#### Project Structure

`crates/`  
\- `contract/` - Rust smart contract code  
\- `contract-data/` - Rust smart contract data types  
\- `solana-bindgen/` - Rust lib that binds to global values from JS  
\- `solana-contract-types/` - Solana BPF SDK data types  
\- `solana-contract-utils/` - Solana BPF SDK  
\- `webapp/` - Rust web app  

* I chose to split up contract data types from implementation since that was all I wanted to depend on in `webapp` but ultimately I don't think that was necessary.
* `solana-bindgen` uses `wasm-bindgen` to bind to JS. The JS can either be a module, a local file, or a global [[docs]](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-js-imports/index.html). I wasn't aware of binding to a module so I hacked `solana-web3.js` to export some global variables 😅.
* `webapp` depends on `solana/sdk` but I had to feature gate the `memmap` dependency which is used for generating the genesis block because it doesn't support WASM.
* `webapp` was unable to leverage `solana/client` because WASM doesn't have support for I/O and threading. You have to bind to JS in order to make http requests and use WebSockets for example. More details [here](https://rustwasm.github.io/docs/book/reference/add-wasm-support-to-crate.html).

#### Wins

* Deserialization from JS values is pretty great with `serde_wasm_bindgen`
```rust
serde_wasm_bindgen::from_value<solana_sdk::account::Account>(js_val);
````
* It was super easy to read from buffers into the contract data types
```rust
message_feed_contract::MessageAccountData::new(account.data.as_mut_slice());
```
* Calling into `solana-web3.js` was pretty seamless once the bindings were written
```rust
use solana_bindgen::{Connection, PublicKey};
use solana_sdk::pubkey::Pubkey;
let js_connection = Connection::new("https://api.beta.testnet.solana.com");
let pubkey = Pubkey::from_str("J56CEQXnxkEvQ7yT1D82gu9fG5t6ykWC3eu4nUABXabt").unwrap();
let js_key = PublicKey::new(pubkey.as_ref().to_vec());
js_connection.get_account_info(js_key).then(/* closure */);
```

#### Going Forward

* Building out a wasm friendly solana client would be pretty ideal for webapp development with Rust.
  * This would require JS or WASM-friendly JSON RPC and WebSocket clients. The former would require wasm bindings.
  * A wasm friendly client in Rust could be distributed as an NPM package using [`wasm-pack`](https://github.com/rustwasm/wasm-pack)
  * Wouldn't need to bind to the `solana-web3.js` module
* Binding to `solana-web3.js` could be a pain
  * It's possible that you could generate the bindings somehow from Flow but haven't looked into it.
  * For this proof of concept, I had to manually write up all the bindings and there were no compile time guarantees that I got it right
