use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(JsonFormatter)]
pub fn json_formatter() -> Html {
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

    let format_pretty = {
        let input = input.clone();
        let output = output.clone();
        let error = error.clone();
        Callback::from(move |_: MouseEvent| {
            match serde_json::from_str::<serde_json::Value>(&input) {
                Ok(value) => {
                    let pretty = serde_json::to_string_pretty(&value)
                        .unwrap_or_else(|_| "Failed to format.".into());
                    output.set(pretty);
                    error.set(None);
                }
                Err(e) => {
                    error.set(Some(format!("Invalid JSON: {e}")));
                }
            }
        })
    };

    let minify = {
        let input = input.clone();
        let output = output.clone();
        let error = error.clone();
        Callback::from(move |_: MouseEvent| {
            match serde_json::from_str::<serde_json::Value>(&input) {
                Ok(value) => {
                    let compact = serde_json::to_string(&value)
                        .unwrap_or_else(|_| "Failed to minify.".into());
                    output.set(compact);
                    error.set(None);
                }
                Err(e) => {
                    error.set(Some(format!("Invalid JSON: {e}")));
                }
            }
        })
    };

    html! {
        <div class="tool">
            <h2>{ "JSON Formatter" }</h2>
            <p class="tool-desc">{ "Validate, pretty-print, or minify JSON — parsed entirely on your device." }</p>

            <label>{ "Input" }</label>
            <textarea
                rows="8"
                value={(*input).clone()}
                oninput={on_input}
                placeholder="Paste JSON here..."
            />

            <div class="button-row">
                <button class="primary" onclick={format_pretty}>{ "Pretty Print" }</button>
                <button class="primary" onclick={minify}>{ "Minify" }</button>
            </div>

            if let Some(err) = (*error).clone() {
                <p class="error">{ err }</p>
            }

            <label>{ "Output" }</label>
            <textarea rows="8" readonly=true value={(*output).clone()} />
        </div>
    }
}
