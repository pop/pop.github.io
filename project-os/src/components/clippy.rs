use yew::prelude::*;
use gloo_timers::callback::Interval;
use crate::config::Quote;

#[derive(Properties, PartialEq)]
pub struct ClippyProps {
    pub quotes: Vec<Quote>,
}

#[function_component(Clippy)]
pub fn clippy(props: &ClippyProps) -> Html {
    let quote_idx = use_state(|| 0usize);
    let modal_open = use_state(|| false);

    // Rotate quotes every 5s
    {
        let qi = quote_idx.clone();
        let total = props.quotes.len();
        use_effect_with(total, move |_| -> Box<dyn FnOnce()> {
            if total == 0 {
                return Box::new(move || {});
            }
            let interval = Interval::new(5000, move || {
                qi.set((*qi + 1) % total);
            });
            Box::new(move || drop(interval))
        });
    }

    let current_quote = props.quotes.get(*quote_idx)
        .map(|q| q.text.clone())
        .unwrap_or_default();

    let open_modal = {
        let mo = modal_open.clone();
        Callback::from(move |_: MouseEvent| mo.set(true))
    };
    let close_modal = {
        let mo = modal_open.clone();
        Callback::from(move |_: MouseEvent| mo.set(false))
    };

    html! {
        <>
            <div class="clippy-widget">
                if !current_quote.is_empty() {
                    <div class="clippy-bubble">
                        <p>{ &current_quote }</p>
                    </div>
                }
                <div class="clippy-icon" onclick={open_modal} title="Click for info">
                    { "📎" }
                </div>
            </div>
            if *modal_open {
                <div class="clippy-modal-backdrop">
                    <div class="window clippy-modal">
                        <div class="title-bar">
                            <div class="title-bar-text">{ "About this Portfolio" }</div>
                        </div>
                        <div class="window-body" style="padding: 16px;">
                            <p>{ "This portfolio was built with assistance from Claude AI (Anthropic)." }</p>
                            <p>{ "The games and creative work are by the portfolio author." }</p>
                            <p>{ "Claude helped with Rust/WASM code and UI implementation." }</p>
                            <div style="text-align: right; margin-top: 12px;">
                                <button onclick={close_modal}>{ "OK" }</button>
                            </div>
                        </div>
                    </div>
                </div>
            }
        </>
    }
}
