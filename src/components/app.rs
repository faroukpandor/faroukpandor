use yew::prelude::*;

use super::base64_tool::Base64Tool;
use super::json_formatter::JsonFormatter;
use super::password::PasswordGenerator;

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Password,
    Base64,
    Json,
}

impl Tool {
    fn label(&self) -> &'static str {
        match self {
            Tool::Password => "Password Generator",
            Tool::Base64 => "Base64 Encode / Decode",
            Tool::Json => "JSON Formatter",
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let active = use_state(|| Tool::Password);

    let make_tab = |tool: Tool, active: UseStateHandle<Tool>| {
        let is_active = *active == tool;
        let onclick = {
            let active = active.clone();
            Callback::from(move |_| active.set(tool))
        };
        html! {
            <button
                class={classes!("tab", is_active.then_some("tab-active"))}
                onclick={onclick}
            >
                { tool.label() }
            </button>
        }
    };

    html! {
        <div class="page">
            <header class="header">
                <h1>{ "🦀 Dev Toolbox" }</h1>
                <p class="tagline">
                    { "Free, private, browser-only developer tools — built in Rust + WebAssembly. Nothing you type ever leaves your device." }
                </p>
            </header>

            <nav class="tabs">
                { make_tab(Tool::Password, active.clone()) }
                { make_tab(Tool::Base64, active.clone()) }
                { make_tab(Tool::Json, active.clone()) }
            </nav>

            <main class="tool-panel">
                {
                    match *active {
                        Tool::Password => html! { <PasswordGenerator /> },
                        Tool::Base64 => html! { <Base64Tool /> },
                        Tool::Json => html! { <JsonFormatter /> },
                    }
                }
            </main>

            <footer class="footer">
                <p>
                    { "Enjoying this? " }
                    <a href="https://buymeacoffee.com/" target="_blank" rel="noopener noreferrer">
                        { "Buy me a coffee" }
                    </a>
                    { " to keep new tools coming — this site runs 100% free, forever." }
                </p>
            </footer>
        </div>
    }
}
