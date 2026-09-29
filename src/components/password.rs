use rand::Rng;
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{};:,.<>?";

fn generate_password(length: usize, use_upper: bool, use_digits: bool, use_symbols: bool) -> String {
    let mut charset: Vec<u8> = LOWER.to_vec();
    if use_upper {
        charset.extend_from_slice(UPPER);
    }
    if use_digits {
        charset.extend_from_slice(DIGITS);
    }
    if use_symbols {
        charset.extend_from_slice(SYMBOLS);
    }

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

#[function_component(PasswordGenerator)]
pub fn password_generator() -> Html {
    let length = use_state(|| 16u32);
    let use_upper = use_state(|| true);
    let use_digits = use_state(|| true);
    let use_symbols = use_state(|| true);
    let password = use_state(|| {
        generate_password(16, true, true, true)
    });
    let copied = use_state(|| false);

    let regenerate = {
        let length = length.clone();
        let use_upper = use_upper.clone();
        let use_digits = use_digits.clone();
        let use_symbols = use_symbols.clone();
        let password = password.clone();
        Callback::from(move |_: MouseEvent| {
            password.set(generate_password(
                *length as usize,
                *use_upper,
                *use_digits,
                *use_symbols,
            ));
        })
    };

    let on_length_input = {
        let length = length.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<u32>() {
                length.set(value.clamp(4, 128));
            }
        })
    };

    let toggle = |flag: UseStateHandle<bool>| {
        Callback::from(move |_: Event| flag.set(!*flag))
    };

    let copy_to_clipboard = {
        let password = password.clone();
        let copied = copied.clone();
        Callback::from(move |_: MouseEvent| {
            let text = (*password).clone();
            let copied = copied.clone();
            if let Some(window) = web_sys::window() {
                let clipboard = window.navigator().clipboard();
                let promise = clipboard.write_text(&text);
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = JsFuture::from(promise).await;
                    copied.set(true);
                    let copied_reset = copied.clone();
                    gloo_timers::callback::Timeout::new(1500, move || {
                        copied_reset.set(false);
                    })
                    .forget();
                });
            }
        })
    };

    html! {
        <div class="tool">
            <h2>{ "Password Generator" }</h2>
            <p class="tool-desc">{ "Generated locally in your browser — never sent anywhere." }</p>

            <div class="password-output">
                <code>{ (*password).clone() }</code>
                <button onclick={copy_to_clipboard}>
                    { if *copied { "Copied!" } else { "Copy" } }
                </button>
            </div>

            <div class="controls">
                <label>
                    { format!("Length: {}", *length) }
                    <input
                        type="range"
                        min="4"
                        max="64"
                        value={length.to_string()}
                        oninput={on_length_input}
                    />
                </label>

                <label class="checkbox">
                    <input
                        type="checkbox"
                        checked={*use_upper}
                        onchange={toggle(use_upper.clone())}
                    />
                    { "Uppercase letters (A-Z)" }
                </label>

                <label class="checkbox">
                    <input
                        type="checkbox"
                        checked={*use_digits}
                        onchange={toggle(use_digits.clone())}
                    />
                    { "Numbers (0-9)" }
                </label>

                <label class="checkbox">
                    <input
                        type="checkbox"
                        checked={*use_symbols}
                        onchange={toggle(use_symbols.clone())}
                    />
                    { "Symbols (!@#$...)" }
                </label>
            </div>

            <button class="primary" onclick={regenerate}>{ "Generate New Password" }</button>
        </div>
    }
}
