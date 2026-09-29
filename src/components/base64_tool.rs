use base64::{engine::general_purpose::STANDARD, Engine as _};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(Base64Tool)]
pub fn base64_tool() -> Html {
    let input = use_state(String::new);
    let output = use_state(String::new);
    let error = use_state(|| Option::<String>::None);

    let on_input = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            input.set(textarea.value());
        })
    };

    let encode = {
        let input = input.clone();
        let output = output.clone();
        let error = error.clone();
        Callback::from(move |_: MouseEvent| {
            let encoded = STANDARD.encode((*input).as_bytes());
            output.set(encoded);
            error.set(None);
        })
    };

    let decode = {
        let input = input.clone();
        let output = output.clone();
        let error = error.clone();
        Callback::from(move |_: MouseEvent| match STANDARD.decode((*input).trim()) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(text) => {
                    output.set(text);
                    error.set(None);
                }
                Err(_) => {
                    error.set(Some("Decoded bytes are not valid UTF-8 text.".into()));
                }
            },
            Err(_) => {
                error.set(Some("That doesn't look like valid Base64.".into()));
            }
        })
    };

    html! {
        <div class="tool">
            <h2>{ "Base64 Encode / Decode" }</h2>
            <p class="tool-desc">{ "Converts text locally in your browser — nothing is uploaded." }</p>

            <label>{ "Input" }</label>
            <textarea
                rows="6"
                value={(*input).clone()}
                oninput={on_input}
                placeholder="Type or paste text here..."
            />

            <div class="button-row">
                <button class="primary" onclick={encode}>{ "Encode →" }</button>
                <button class="primary" onclick={decode}>{ "Decode →" }</button>
            </div>

            if let Some(err) = (*error).clone() {
                <p class="error">{ err }</p>
            }

            <label>{ "Output" }</label>
            <textarea rows="6" readonly=true value={(*output).clone()} />
        </div>
    }
}
