use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
struct MessageRequest {
    name: String,
    content: String,
    recipient: String,
}

#[derive(Deserialize)]
struct MessageResponse {
    status: String,
    message: String,
}

#[function_component(App)]
fn app() -> Html {
    let name = use_state(|| String::from(""));
    let message = use_state(|| String::from(""));
    let recipient = use_state(|| String::from("Bob"));
    let status = use_state(|| String::from(""));

    let onchange_name = {
        let name = name.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let onchange_message = {
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            message.set(input.value());
        })
    };

    let onchange_recipient = {
        let recipient = recipient.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            recipient.set(input.value());
        })
    };

    let onsubmit = {
        let name = name.clone();
        let message = message.clone();
        let recipient = recipient.clone();
        let status = status.clone();
        let msg_state = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name_val = (*name).clone();
            let message_val = (*message).clone();
            let recipient_val = (*recipient).clone();

            if name_val.is_empty() || message_val.is_empty() {
                status.set("Name and message cannot be empty".to_string());
                return;
            }

            let status = status.clone();
            let msg_state = msg_state.clone();

            spawn_local(async move {
                let request = MessageRequest {
                    name: name_val,
                    content: message_val,
                    recipient: recipient_val,
                };

                status.set("Sending message...".to_string());

                let result = invoke("send_message", JsValue::from_serde(&request).unwrap()).await;

                match result.into_serde::<MessageResponse>() {
                    Ok(response) => {
                        status.set(response.message);
                        if response.status == "success" {
                            msg_state.set(String::new()); // Clear message input on success
                        }
                    }
                    Err(e) => {
                        status.set(format!("Error: {}", e));
                    }
                }
            });
        })
    };

    html! {
        <div class="app">
            <div class="header">
                <h1>{"QuietDrop"}</h1>
                <p>{"End-to-End Encrypted Messaging"}</p>
            </div>

            <div class="chat-container">
                <h2>{"Messages"}</h2>
                <p>{"No messages yet. Start a conversation!"}</p>

                if !(*status).is_empty() {
                    <div class="status-message">
                        <p>{&*status}</p>
                    </div>
                }
            </div>

            <form {onsubmit}>
                <div class="message-input">
                    <input
                        type="text"
                        placeholder="Your name"
                        value={(*name).clone()}
                        onchange={onchange_name}
                    />
                </div>
                <div class="message-input">
                    <input
                        type="text"
                        placeholder="Recipient"
                        value={(*recipient).clone()}
                        onchange={onchange_recipient}
                    />
                </div>
                <div class="message-input">
                    <input
                        type="text"
                        placeholder="Type your message..."
                        value={(*message).clone()}
                        onchange={onchange_message}
                    />
                    <button type="submit">{"Send"}</button>
                </div>
            </form>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
